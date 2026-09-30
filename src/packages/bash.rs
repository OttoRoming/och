use super::*;

fn package() -> Package {
    Package::new("bash", "5.3")
        .dependencies((glibc.package(), ncurses.package(), readline.package()))
        .make_dependencies((
            binutils.package(),
            bison.package(),
            coreutils.package(),
            diffutils.package(),
            gawk.package(),
            gcc.package(),
            grep.package(),
            make.package(),
            patch.package(),
            sed.package(),
            texinfo.package(),
        ))
        .self_dependant()
        .build(Build::Configure(
            Configure::new()
                .opt("without-bash-malloc")
                .opt("with-installed-readline")
                .var("prefix", "/usr")
                .var("docdir", "/usr/share/doc/bash-5.3"),
        ))
        .check(Check::Make)
}
