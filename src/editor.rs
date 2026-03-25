use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};
mod terminal;
use std::io::Error;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    cursor: Position,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            cursor: Position { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                // when pattern matching rust does implicit(automatic) deref.
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    // in the case of comparison
                    // we will deref. explicitly

                    self.should_quit = true;
                }

                crossterm::event::KeyCode::Right => {
                    let size = Terminal::size().unwrap();
                    let width = size.width;

                    if self.cursor.x < width - 1 {
                        self.cursor.x += 1;
                    }
                }

                crossterm::event::KeyCode::Left => {
                    if self.cursor.x > 0 {
                        self.cursor.x -= 1;
                    }
                }

                crossterm::event::KeyCode::Down => {
                    let size = Terminal::size().unwrap();
                    let height = size.height;

                    if self.cursor.y < height - 1 {
                        self.cursor.y += 1;
                    }
                }

                crossterm::event::KeyCode::Up => {
                    if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                    }
                }

                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {
                x: self.cursor.x,
                y: self.cursor.y,
            })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
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
        Terminal::print(welcome_message)?;
        Ok(())
    }

    fn draw_empty_rows() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or downCollapse comment

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_rows()?;
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
