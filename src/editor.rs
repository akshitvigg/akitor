use crossterm::event::read;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::{Size, Terminal};
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info)
        }));
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("Could not read event {err:?}");
                }
            };
        }
    }

    //needless_pass_by_value : Event is not huge, so there is not a
    //performance overhead in passing by value, and pattern matching in this
    //function would be needlessly complicated if we pass by reference here.
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
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
                    ) => {
                        let _ = self.view.move_cursor(code);
                    }

                    _ => {}
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
            _ => {}
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let pos = self.view.cursor_pos();
        let _ = Terminal::move_caret_to(pos);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
