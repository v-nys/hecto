#![warn(clippy::all)]
mod editor;
use editor::Editor;

mod terminal;
// BIJHOUDEN: dit is een manier om in terminal gebruik te maken van Position zonder rechtstreeks afhankelijk te worden van feit dat dit in editor zit, i.e. facade pattern
pub use editor::Position;

fn main() {
    Editor::default().run();
}
