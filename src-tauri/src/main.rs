// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod glock_17;
mod my_db;
mod utils;

use glock_17::{EventData, EventLine, EventOverlapData, ResData};
use utils::check_db_file;

fn main() {
    initdb();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_overlap_event,
            parse_data_from_xlsx,
            generate_xlsx_from_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initdb() {
    check_db_file().unwrap();
}

#[tauri::command]
fn get_overlap_event(event_data: Vec<EventLine>) -> EventOverlapData {
    EventData::check_data_conflict(&EventData { data: event_data })
}

#[tauri::command]
fn parse_data_from_xlsx(path: &str) -> EventData {
    if let Ok(data) = EventData::parse_from_xlsx(path) {
        println!("parse success!");
        data
    } else {
        println!("parse error!");
        EventData { data: vec![] }
    }
}

#[tauri::command]
fn generate_xlsx_from_data(event_data: Vec<EventLine>, path: &str) -> ResData<()> {
    let data = EventData::new(event_data);
    let mut res = ResData {
        status: "SUCCESS".to_string(),
        data: (),
    };
    if let Ok(_) = data.xlsx_from_data(path) {
        res
    } else {
        res.status = "FAILED".to_string();
        res
    }
}
