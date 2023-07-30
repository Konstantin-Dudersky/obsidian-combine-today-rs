use std::fs::write;

use crate::{application::IFileWriter, errors::Errors};

use super::std_use::PathBuf;

pub struct FileWriter {
    target_file: PathBuf,
}

impl FileWriter {
    pub fn new(target_file: &str) -> Box<Self> {
        Self {
            target_file: PathBuf::from(target_file),
        }
        .into()
    }
}

impl IFileWriter for FileWriter {
    fn write(&self, content: &str) -> Result<(), Errors> {
        match write(&self.target_file, content) {
            Ok(_) => Ok(()),
            Err(value) => {
                let msg = value.to_string();
                Err(Errors::WriteError(msg))
            }
        }
    }
}
