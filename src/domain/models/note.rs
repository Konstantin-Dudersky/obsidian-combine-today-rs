use std::path;

#[derive(Clone)]
pub struct Note {
    pub vault_path: path::PathBuf,
    pub folder_inside_vault: path::PathBuf,
    pub note_name: path::PathBuf,
    pub content: Option<String>,
}

impl Note {
    pub fn new() -> Self {
        let vault_path = path::PathBuf::from("/home/konstantin/documents/.obsidian/obsidian/");
        Self {
            vault_path: vault_path,
            folder_inside_vault: path::PathBuf::from("_calendar"),
            note_name: path::PathBuf::from("2023-07-30.md"),
            content: None,
        }
    }

    pub fn full_path(&self) -> path::PathBuf {
        let mut path = self.vault_path.clone();
        path.push(path::Path::new(&self.folder_inside_vault));
        path.push(path::Path::new(&self.note_name));
        path
    }
}
