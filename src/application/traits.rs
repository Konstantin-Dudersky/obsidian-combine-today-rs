use crate::{domain::models::Note, errors};

pub trait INoteReader {
    fn read(&self, path: &Note) -> Result<Note, errors::Errors>;
}
