use crate::{domain::models::Note, errors};

/// Чтение содержания заметки
pub trait INoteReader {
    fn read(&self, path: &Note) -> Result<Note, errors::Errors>;
}

/// Получить название файла заметки по времени
pub trait IGetTimestamp {
    fn get(&self) -> String;
}

pub trait IFileWriter {
    fn write(&self, content: &str) -> Result<(), errors::Errors>;
}
