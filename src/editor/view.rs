use core::cmp::min;
mod buffer;
use crate::editor::terminal::Position;

use super::terminal::{Size, Terminal};
use buffer::Buffer;
use crossterm::event::KeyCode;

use std::io::Error;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

pub struct View {
    buffer: Buffer,
    pub needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn update_scroll(&mut self) {
        let width = self.size.width;
        let height = self.size.height;

        let cursor_x = self.location.x;
        let cursor_y = self.location.y;

        if cursor_x >= self.scroll_offset.x + width {
            self.scroll_offset.x = cursor_x - (width - 1);
        } else if cursor_x < self.scroll_offset.x {
            self.scroll_offset.x = cursor_x;
        }

        if cursor_y >= self.scroll_offset.y + height {
            self.scroll_offset.y = cursor_y - (height - 1);
        } else if cursor_y < self.scroll_offset.y {
            self.scroll_offset.y = cursor_y;
        }
    }

    pub fn cursor_pos(&self) -> Position {
        Position {
            col: self.location.x - self.scroll_offset.x,
            row: self.location.y - self.scroll_offset.y,
        }
    }

    pub fn move_cursor(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        // let Size { height, width } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = y.saturating_add(1),
            KeyCode::Right => x = x.saturating_add(1),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y += 10,
            KeyCode::Home => x = 0,
            KeyCode::End => x += 10,
            _ => (),
        }

        self.location = Location { x, y };
        self.update_scroll();
        self.needs_redraw = true;

        Ok(())
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;

        if height == 0 || width == 0 {
            return;
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit too far up or down
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row + self.scroll_offset.y) {
                let start = self.scroll_offset.x;
                let end = min(start + width, line.len());
                let truncated_line = if start >= line.len() {
                    ""
                } else {
                    &line[start..end]
                };

                Self::render_line(current_row, truncated_line);
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}
