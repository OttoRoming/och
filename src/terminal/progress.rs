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
    /// Display the current progressbar written to stdout without any carrige return or newline
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

    /// Clamps the progressbar's progress to between 0.0 and 1.0
    fn clamp(&mut self) -> &mut Self {
        self.progress = self.progress.clamp(0.0, 1.0);
        self
    }

    /// Starts displaying the progressbar
    pub fn start(&mut self) -> io::Result<()> {
        self.progress = 0.0;
        self.display("")?;

        Ok(())
    }

    /// Mutates the current progress of the progressbar with a message
    pub fn add(&mut self, delta: f64, message: &str) -> io::Result<&mut Self> {
        self.progress += delta;
        self.clamp();
        print!("\r");
        self.display(message)?;

        Ok(self)
    }

    /// Updates the progressbar with a message
    pub fn update(&mut self, progress: f64, message: &str) -> io::Result<&mut Self> {
        self.progress = progress;
        self.clamp();
        print!("\r");
        self.display(message)?;

        Ok(self)
    }

    /// Finishes displaying the progressbar and prints a newline
    pub fn finish(&mut self) -> io::Result<&mut Self> {
        self.progress = 1.0;
        print!("\r");
        self.display("")?;
        print!("\n");

        Ok(self)
    }
}
