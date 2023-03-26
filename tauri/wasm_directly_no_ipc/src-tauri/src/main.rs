// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Instant;

use once_cell::sync::OnceCell;
use tauri::{Manager, Window};

static INSTANCE: OnceCell<Instant> = OnceCell::new();

#[tauri::command]
fn start() {
    INSTANCE.set(Instant::now()).unwrap();
}

#[tauri::command]
fn stop() -> String {
    format!("{}", INSTANCE.get().unwrap().elapsed().as_nanos())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start, stop])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
