#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]

mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();

    let arg: Vec<String> = std::env::args().collect();

    if let Some(first_arg) = arg.get(1) {
        editor.open(first_arg);
    }
    editor.run();
}
