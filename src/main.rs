use std::fs;

use obsidian_combine_today_rs::bootstrap;

const VAULT_PATH: &str = "/home/konstantin/documents/.obsidian/obsidian/";
const FOLDER_CALENDAR: &str = "calendar";
const NOTE_NAME_FORMAT: &str = "%Y-%m-%d.md";

const TARGET_FILE: &str = "/home/konstantin/desktop/today_tasks.md";

const NOTE: &str = "/home/konstantin/documents/.obsidian/obsidian/_calendar/2023-07-30.md";

fn main() {
    bootstrap();
    // let file = fs::read(NOTE).unwrap();

    // let content = String::from_utf8_lossy(&file);
}
