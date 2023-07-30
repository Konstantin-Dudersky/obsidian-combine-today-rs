//! Получение названия файла исходя из сегодняшней даты
//! Используется крейт chrono

use chrono::Local;

use crate::{application::IGetTimestamp, constants::NOTE_NAME_FORMAT};

pub struct GetTimestamp {}

impl GetTimestamp {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl IGetTimestamp for GetTimestamp {
    fn get(&self) -> String {
        Local::now().format(NOTE_NAME_FORMAT).to_string()
    }
}
