use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use std::convert::TryFrom;

use super::terminal::Size;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

pub enum EditorCommand {
    Move(Directions),
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Directions::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Directions::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Directions::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Directions::Right)),
                (KeyCode::Home, _) => Ok(Self::Move(Directions::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Directions::End)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Directions::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Directions::PageDown)),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(height_u16, width_u16) => {
                //clippy::as_conversions- will run into problems in rare edge case
                //systems where usize < u16
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;

                //clippy::as_conversions- will run into problems in rare edge case
                //systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;

                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
