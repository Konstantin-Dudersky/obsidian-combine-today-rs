#[derive(Debug)]
pub enum Errors {
    VaultNotExistError(String),
    FolderInVaultNotExist(String),
    NoteNotExist(String),
}
