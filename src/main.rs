#![warn(clippy::all)]
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}
