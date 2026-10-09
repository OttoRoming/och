//! Build packages

use crate::paths;
use crate::terminal::log;
use deko::AnyDecoder;
use podman_api::{
    Podman,
    conn::TtyChunk,
    opts::{ContainerListFilter, ContainerListOpts, ExecCreateOpts, ExecStartOpts, UserOpt},
};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use futures_util::StreamExt;
use tempfile::tempfile;
use url::Url;

use super::{Package, Source};

const CONTAINER: &str = "builder";

/// The shell inside the container that runs build scripts.
const SHELL: &str = "/usr/bin/brush";

/// The flags [`SHELL`] runs build scripts with.
///
/// `-e` stops a recipe at its first failing command. Without it the recipe
/// keeps going, so a failed `make` is followed by `make install` and the
/// package can look built while nothing was installed.
const SHELL_FLAGS: [&'static str; 1] = ["-e"];

/// Where the container sees the package work tree. It is the `./work` directory
/// of the checkout, which `docker-compose.yml` mounts.
const CONTAINER_WORK: &'static str = "/home/builder/work";

/// Where packages should be install to inside the container
const CONTAINER_DESTDIR: &'static str = "/home/builder/destdir";

/// Everything that can go wrong while talking to the build container.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to talk to podman: {0}")]
    Podman(#[from] podman_api::Error),
    #[error("the `{name}` container is not running; start it with `docker compose up -d`")]
    Missing { name: String },
    #[error("`{name}` names {count} containers, expected exactly one")]
    Ambiguous { name: String, count: usize },
    #[error("the exec session for `{command}` returned no exit status")]
    NoExitStatus { command: String },
    #[error("failed to unpack a source archive: {0}")]
    Unpack(#[source] io::Error),
    #[error("`{package}` exited with status {status} in `{name}`: {command}")]
    Command {
        name: String,
        package: String,
        command: String,
        status: i32,
    },
}

fn unpack(reader: impl BufRead, into: &Path) -> io::Result<()> {
    let mut decoder = AnyDecoder::new(reader);
    decoder.fail_on_unknown_format(true);

    let mut archive = tar::Archive::new(decoder);

    archive.unpack(into)?;

    Ok(())
}

/// The names of the entries currently in a directory.
///
/// The work tree is shared by every package, so the directory a recipe runs in
/// is the one entry an archive adds, not the only entry that is there.
fn entries(dir: &Path) -> HashSet<String> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return HashSet::new();
    };

    read.flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

/// The single entry an archive added to `into`, as a path inside the container.
///
/// Sources are extracted under the shared work tree, so this is the directory a
/// build recipe runs in. A tarball that unpacks to several entries, or to loose
/// files rather than a directory, reports `None` and the recipe runs in the
/// work tree itself.
fn unpacked_dir(into: &Path, before: &HashSet<String>, unpacked: &str) -> Option<PathBuf> {
    let added: Vec<_> = entries(into).difference(before).cloned().collect();

    let [name] = added.as_slice() else {
        return None;
    };

    let path = Path::new(unpacked).join(name);
    into.join(name).is_dir().then_some(path)
}

async fn fetch_http(
    url: Url,
    hash: &[u8; 32],
    before: &HashSet<String>,
) -> Result<Option<PathBuf>, Error> {
    let response = reqwest::get(url).await.unwrap(); //.bytes().await.unwrap();
    assert!(response.status().is_success());

    let mut file = tempfile().unwrap();
    let mut hasher = Sha256::new();
    let mut byte_stream = response.bytes_stream();

    while let Some(chunk_result) = byte_stream.next().await {
        let chunk = chunk_result.unwrap();
        file.write_all(&chunk).unwrap();
        hasher.update(&chunk);
    }

    let fetched_hash: [u8; 32] = hasher.finalize().into();
    assert_eq!(hash, &fetched_hash);

    file.seek(SeekFrom::Start(0)).unwrap();
    let reader = BufReader::new(file);

    let work = paths::work();
    unpack(reader, &work).map_err(Error::Unpack)?;

    Ok(unpacked_dir(&work, before, CONTAINER_WORK))
}

/// A handle on the container that build scripts run in.
///
/// It connects to the Podman socket of the current user and resolves the
/// container by name, so it keeps working when a rebuild changes the container
/// id.
pub struct Container {
    podman: Podman,
    name: String,
}

impl Container {
    /// Connects to the Podman socket of the current user.
    pub fn connect(name: impl Into<String>) -> Self {
        let uid = unsafe { libc::getuid() };

        Self {
            podman: Podman::unix(format!("/run/user/{uid}/podman/podman.sock")),
            name: name.into(),
        }
    }

    /// The name the container is looked up under.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Resolves the container name to an id.
    async fn find(&self) -> Result<String, Error> {
        let containers = self
            .podman
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .filter([ContainerListFilter::Name(self.name.clone())])
                    .build(),
            )
            .await?;

        match containers.as_slice() {
            [] => Err(Error::Missing {
                name: self.name.clone(),
            }),
            [container] => container.id.clone().ok_or_else(|| Error::Missing {
                name: self.name.clone(),
            }),
            containers => Err(Error::Ambiguous {
                name: self.name.clone(),
                count: containers.len(),
            }),
        }
    }

    /// Runs one shell command inside the container.
    ///
    /// Standard output and standard error of the command are streamed to the
    /// terminal as they are produced, and the exit status is returned so the
    /// caller can decide whether a non-zero status is fatal.
    ///
    /// The command runs in [`CONTAINER_WORK`] unless `working_dir` says
    /// otherwise, and with the given environment variables.
    pub async fn exec(
        &self,
        command: impl Into<String>,
        working_dir: Option<&Path>,
        env: &[(&str, &str)],
        as_root: bool,
    ) -> Result<i32, Error> {
        let command = command.into();
        let id = self.find().await?;
        let container = self.podman.containers().get(&id);

        let user_opt = if as_root {
            UserOpt::Uid(0)
        } else {
            UserOpt::User("builder".to_string())
        };

        let mut create = ExecCreateOpts::builder()
            .command(
                [SHELL]
                    .into_iter()
                    .chain(SHELL_FLAGS)
                    .chain(["-c", &command]),
            )
            .user(user_opt)
            .attach_stdout(true)
            .attach_stderr(true)
            .env(env.to_vec());

        if let Some(dir) = working_dir {
            create = create.working_dir(dir.to_string_lossy());
        }

        let exec = container.create_exec(&create.build()).await?;
        let start = ExecStartOpts::builder().build();

        // A start that is not detached streams the output, and only returns
        // `None` when the request asked to detach, which this one does not.
        let Some(mut stream) = exec.start(&start).await? else {
            return Err(Error::NoExitStatus { command });
        };

        while let Some(chunk) = stream.next().await {
            match chunk.map_err(podman_api::Error::from)? {
                TtyChunk::StdOut(bytes) => print!("{}", String::from_utf8_lossy(&bytes)),
                TtyChunk::StdErr(bytes) => eprint!("{}", String::from_utf8_lossy(&bytes)),
                TtyChunk::StdIn(_) => {}
            }
        }

        exec_status(&exec, &command).await
    }
}

/// Reads the exit status of a finished exec session.
async fn exec_status(exec: &podman_api::api::Exec, command: &str) -> Result<i32, Error> {
    let info = exec.inspect().await?;

    info.get("ExitCode")
        .and_then(|code| code.as_i64())
        .map(|code| code as i32)
        .ok_or_else(|| Error::NoExitStatus {
            command: command.to_owned(),
        })
}

pub struct Builder<'a> {
    package: &'a Package,
}

impl<'a> Builder<'a> {
    pub fn new(package: &'a Package) -> Self {
        Self { package }
    }

    pub async fn build(&self) -> Result<(), Error> {
        dbg!("Fetching sources");
        let dir = self.fetch_sources().await;

        dbg!("Running recepie with", &dir);
        self.run_recepie(dir).await
    }

    /// Downloads every source and unpacks it into the shared work tree.
    ///
    /// Returns the directory inside the work tree that the recipe should run
    /// in, when the sources unpacked into exactly one. A source that cannot be
    /// fetched is reported and leaves the recipe with no directory to run in,
    /// so the container reports the failure the recipe itself hits.
    async fn fetch_sources(&self) -> Option<PathBuf> {
        let work = paths::work();
        let mut dir = None;

        for source in self.package.iter_sources() {
            match &source {
                &Source::Http { url, hash_sha256 } => {
                    let before = entries(&work);

                    match fetch_http(url.clone(), hash_sha256, &before).await {
                        Ok(Some(unpacked)) => dir = Some(unpacked),
                        Ok(None) => {}
                        Err(error) => {
                            panic!("Failed to fetch http {}", error);
                        }
                    }
                }
            }
        }

        dir
    }

    /// Runs the package's build recipe inside the container.
    ///
    /// The sources were unpacked into the host's work directory, which the
    /// container sees as [`CONTAINER_WORK`], so the recipe runs there
    /// unchanged.
    async fn run_recepie(&self, dir: Option<PathBuf>) -> Result<(), Error> {
        let Some(build) = self.package.build() else {
            return Ok(());
        };

        let container = Container::connect(CONTAINER);

        let command = match build {
            super::Build::Script(script) => {
                format!("export DESTDIR=\"$HOME/destdir\"\n\n {}", script)
            }
            super::Build::Python(_module) => todo!("Python module building not implemented"),
        };

        let working_dir = dir
            .map(|dir| dir.to_string_lossy().into_owned())
            .unwrap_or_else(|| CONTAINER_WORK.to_owned());

        let status = container
            .exec(
                "chown -R builder:builder .".to_string(),
                Some(Path::new(&working_dir)),
                &[],
                true,
            )
            .await?;
        if status != 0 {
            panic!("Failed to chown");
        }

        let status = container
            .exec(
                "chown -R builder:builder .".to_string(),
                Some(Path::new(CONTAINER_DESTDIR)),
                &[],
                true,
            )
            .await?;
        if status != 0 {
            panic!("Failed to chown");
        }

        let status = container
            .exec(command.clone(), Some(Path::new(&working_dir)), &[], false)
            .await?;

        if status != 0 {
            return Err(Error::Command {
                name: container.name().to_owned(),
                package: self.package.name().to_owned(),
                command,
                status,
            });
        }

        log::info(&format!("{} built", self.package.name()));

        Ok(())
    }
}
