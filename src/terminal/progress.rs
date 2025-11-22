use std::{
    default,
    io::{self, Write},
};

use owo_colors::OwoColorize;

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
            Ok(v) => v.ws_col as usize,
            Err(_) => 0,
        };

        match columns.checked_sub(message.len() + 2) {
            Some(width) => {
                let active_width = (self.progress * width as f64) as usize;
                let inactive_width = width - active_width as usize;

                print!(
                    "{message}{open}{active}{inactive}{close}",
                    message = message.bold(),
                    open = "[".dimmed(),
                    active = "o".repeat(active_width).yellow(),
                    inactive = " ".repeat(inactive_width),
                    close = "]".dimmed(),
                )
            }
            None => {
                print!("{}", message);
            }
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
