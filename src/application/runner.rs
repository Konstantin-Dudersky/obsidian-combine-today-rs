use super::traits;

pub struct Runner {
    note_reader: Box<dyn traits::INoteReader>,
}

impl Runner {
    pub fn new(note_reader: Box<dyn traits::INoteReader>) -> Self {
        Self { note_reader }
    }

    pub fn run(&self) {}
}
