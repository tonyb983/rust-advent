use lazy_static::lazy_static;
use std::io::{self, Write};

use ansi_term::{Color, Style};

pub struct ConsoleWriter {
    out: io::Stdout,
}

impl ConsoleWriter {
    pub fn create() -> Self {
        Self { out: io::stdout() }
    }

    pub fn write_green(&mut self, text: &str, newline: bool) -> io::Result<()> {
        lazy_static! {
            static ref GREEN: Style = Style::new().fg(ansi_term::Color::Green);
        }

        if newline {
            writeln!(self.out, "{}", GREEN.paint(text))
        } else {
            write!(self.out, "{}", GREEN.paint(text))
        }
    }

    pub fn write_color(&mut self, text: &str, color: Color) -> io::Result<()> {
        let style = Style::new().fg(color);
        write!(self.out, "{}", style.paint(text))
    }

    pub fn writeln_color(&mut self, text: &str, color: Color) -> io::Result<()> {
        let style = Style::new().fg(color);
        writeln!(self.out, "{}", style.paint(text))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
