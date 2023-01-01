mod editor;
mod terminal;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    let mut editor = editor::Editor::default();
    editor.run();
}
