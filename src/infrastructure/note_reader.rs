use std::fs;
use std::path;

use crate::{application, domain::models::Note, errors::Errors};

pub struct NoteReader {}

impl NoteReader {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl application::INoteReader for NoteReader {
    fn read(&self, note: &Note) -> Result<Note, Errors> {
        check_folder(note.vault_path.as_path())?;
        let content = read_note_content(note.full_path().as_path())?;
        let content = String::from_utf8_lossy(&content);
        let new_note = Note {
            content: Some(content.to_string()),
            ..note.clone()
        };
        Ok(new_note)
    }
}

/// Проверка, что хранилище существует
fn check_folder(path: &path::Path) -> Result<(), Errors> {
    if !path.exists() {
        return Err(Errors::VaultNotExistError(
            path.to_str().unwrap_or_default().to_string(),
        ));
    }
    Ok(())
}

/// Читаем содержимое заметки
fn read_note_content(path: &path::Path) -> Result<Vec<u8>, Errors> {
    match fs::read(&path) {
        Ok(value) => Ok(value),
        Err(_) => {
            let msg = path.to_str().unwrap_or_default().to_string();
            return Err(Errors::NoteNotExist(msg));
        }
    }
}
