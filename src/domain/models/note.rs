use std::path;

#[derive(Clone)]
pub struct Note {
    pub path_to_daily_notes: path::PathBuf,
    pub note_name: path::PathBuf,
    pub content: Option<String>,
}

impl Note {
    pub fn new(path: &str, note_name: &str) -> Self {
        let vault_path = path::PathBuf::from(path);
        Self {
            path_to_daily_notes: vault_path,
            note_name: path::PathBuf::from(note_name),
            content: None,
        }
    }

    pub fn full_path(&self) -> path::PathBuf {
        let mut path = self.path_to_daily_notes.clone();
        path.push(path::Path::new(&self.note_name));
        path
    }
}
