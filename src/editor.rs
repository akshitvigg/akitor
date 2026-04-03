use core::cmp::min;
use crossterm::event::read;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use std::{env, io::Error};
use terminal::{Position, Size, Terminal};
mod view;
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(event)?;
        }
        Ok(())
    }

    fn move_to(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }

        self.location = Location { x, y };
        self.view.needs_redraw = true;

        Ok(())
    }

    //needless_pass_by_value : Event is not huge, so there is not a
    //performance overhead in passing by value, and pattern matching in this
    //function would be needlessly complicated if we pass by reference here.
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                match (code, modifiers) {
                    // when pattern matching rust does implicit(automatic) deref.
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        // in the case of comparison
                        // we will deref. explicitly

                        self.should_quit = true;
                        self.view.needs_redraw = true;
                    }

                    (
                        KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::PageUp
                        | KeyCode::PageDown
                        | KeyCode::Home
                        | KeyCode::End,
                        _,
                    ) => self.move_to(code)?,

                    _ => (),
                }
            }

            Event::Resize(width_u16, height_u16) => {
                //clippy::as_conversions: will run into problem for rare edge case system where
                //usize < u16
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;

                //clippy:: as_conversions: will run into problem for  rare edge case system where
                //usize < u16
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;

                self.view.resize(Size { height, width });
            }
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
