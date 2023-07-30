#[derive(Debug)]
pub enum Errors {
    VaultNotExistError(String),
    NoteNotExist(String),
    WriteError(String),
    FewParametersError(String),
}
