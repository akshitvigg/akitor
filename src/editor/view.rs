mod buffer;
use super::terminal::{Size, Terminal};
use buffer::Buffer;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let file_contents = std::fs::read_to_string(file_name)?;

        self.buffer.lines.clear();
        for line in file_contents.lines() {
            self.buffer.lines.push(line.to_string());
        }

        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or downCollapse comment

            if !self.buffer.is_empty() {
                if let Some(line) = self.buffer.lines.get(current_row) {
                    Terminal::print(line)?;
                    Terminal::print("\r\n")?;
                    continue;
                } else {
                    Self::draw_empty_rows()?;
                }
            } else {
                #[allow(clippy::integer_division)]
                if current_row == height / 3 {
                    Self::draw_welcome_message()?;
                } else {
                    Self::draw_empty_rows()?;
                }
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let width = Terminal::size()?.width;
        let length = welcome_message.len();

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(length)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");

        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_rows() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
