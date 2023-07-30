use crate::{domain::models::Note, errors::Errors};

use super::traits;

pub struct Runner {
    note_reader: Box<dyn traits::INoteReader>,
    get_timestamp: Box<dyn traits::IGetTimestamp>,
    file_writer: Box<dyn traits::IFileWriter>,
    folders: Vec<String>,
}

impl Runner {
    pub fn new(
        note_reader: Box<dyn traits::INoteReader>,
        get_timestamp: Box<dyn traits::IGetTimestamp>,
        file_writer: Box<dyn traits::IFileWriter>,
        folders: &[String],
    ) -> Self {
        Self {
            note_reader,
            get_timestamp,
            folders: folders.to_vec(),
            file_writer,
        }
    }

    pub fn run(&self) -> Result<(), Errors> {
        let note_name = self.get_timestamp.get();
        let mut notes: Vec<Note> = vec![];
        for folder in &self.folders {
            let note = Note::new(folder, &note_name);
            let note = self.note_reader.read(&note)?;
            notes.push(note);
        }

        let mut combine_content = "".to_string();
        for note in &notes {
            combine_content += note.content.as_ref().unwrap().as_str();
        }

        self.file_writer.write(&combine_content)?;

        Ok(())
    }
}
