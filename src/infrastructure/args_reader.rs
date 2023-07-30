//! Чтение аргументов командной строки

use std::env;

use crate::errors::Errors;

#[derive(Debug)]
pub struct Args {
    pub target_file: String,
    pub folders: Vec<String>,
}

impl Args {
    pub fn new() -> Result<Self, Errors> {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 2 {
            let msg = format!(
                "Передано параметров: {}, необходимо минимум 2",
                args.len() - 1
            );
            return Err(Errors::FewParametersError(msg));
        }
        let target_file = args[1].clone();
        let mut folders: Vec<String> = vec![];
        for arg in args.iter().skip(2) {
            folders.push(arg.clone())
        }
        Ok(Self {
            target_file,
            folders,
        })
    }
}
