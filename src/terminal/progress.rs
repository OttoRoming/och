use std::{
    default,
    io::{self, Write},
};

pub struct Progress {
    progress: f64,
}

impl default::Default for Progress {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}

impl Progress {
    fn display(&mut self, message: &str) -> io::Result<&mut Self> {
        let columns = match super::size() {
            Ok(v) => v.ws_col as f64,
            Err(_) => 0.0,
        };

        let progress_char_count = columns - message.len() as f64 - 3.0;
        let active_progress_char_count = self.progress * progress_char_count;

        // if we don't have space for the progressbar only print the label
        if progress_char_count < 0.0 {
            print!("{}", message);
        } else if message == "" {
            print!(
                "[{: <1$}]",
                "o".repeat(active_progress_char_count as usize),
                progress_char_count as usize,
            )
        } else {
            print!(
                "{} [{: <2$}]",
                message,
                "o".repeat(active_progress_char_count as usize),
                progress_char_count as usize,
            )
        }

        io::stdout().flush()?;
        Ok(self)
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.progress = 0.0;
        self.display("")?;

        Ok(())
    }

    pub fn update(&mut self, progress: f64, message: &str) -> io::Result<&mut Self> {
        self.progress = progress;
        print!("\r");
        self.display(message)?;

        Ok(self)
    }

    pub fn finish(&mut self) -> io::Result<&mut Self> {
        self.progress = 1.0;
        print!("\r");
        self.display("")?;
        print!("\n");

        Ok(self)
    }
}
