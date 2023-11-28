// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod glock_17;
mod my_db;
mod utils;

use glock_17::{EventData, EventLine};
use utils::check_db_file;

fn main() {
    initdb();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_overlap_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initdb() {
    check_db_file().unwrap();
}

#[tauri::command]
fn get_overlap_event(event_data: Vec<EventLine>) -> EventData {
    EventData::check_data_conflict(&EventData { data: event_data })
}
