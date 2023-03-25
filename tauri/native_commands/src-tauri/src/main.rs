// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Instant;

use once_cell::sync::OnceCell;
use tauri::{Manager, Window};

static INSTANCE: OnceCell<Instant> = OnceCell::new();
static COUNT_NUM: usize = 10_000;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Result {
    time: u128,
}

#[tauri::command]
fn ping(window: Window, count: usize) -> usize {
    if count == 0 {
        INSTANCE.set(Instant::now()).unwrap();
    }

    if count < COUNT_NUM {
        count + 1
    } else {
        let time = INSTANCE.get().unwrap().elapsed().as_nanos();
        window.emit("result", Result { time }).unwrap();
        count
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
