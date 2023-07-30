mod application;
mod domain;
pub mod errors;
mod infrastructure;

use application::{INoteReader, Runner};
use domain::models::Note;

const NOTE: &str = "/home/konstantin/documents/.obsidian/obsidian/_calendar/2023-07-30.md";

pub fn bootstrap() {
    let note_reader = infrastructure::NoteReader::new();

    let runner = Runner::new(note_reader);

    let note = Note::new();

    let note = infrastructure::NoteReader {}.read(&note).unwrap();

    println!("{}", note.content.unwrap())
}
