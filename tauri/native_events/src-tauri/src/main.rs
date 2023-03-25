use std::process::exit;
use std::time::Instant;

use tauri::{Manager, Window};
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<Instant> = OnceCell::new();
const COUNT_NUM: usize = 10_000;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Payload {
    count: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Result {
    time: u128,
}

fn main() {
    tauri::Builder::default()
        .on_page_load(|window, _| {
            let mw_emmiter = window.clone();

            let _id = window.listen("pong", move |event| {
                let payload: Payload = serde_json::from_str(event.payload().unwrap()).unwrap();

                if payload.count == 0 {
                    INSTANCE.set(Instant::now()).unwrap();
                }

                if payload.count < COUNT_NUM {
                    let count = payload.count + 1;
                    mw_emmiter.emit("ping", Payload { count }).unwrap();
                } else {
                    let time = INSTANCE.get().unwrap().elapsed().as_nanos();
                    mw_emmiter.emit("result", Result { time }).unwrap();
                }
            });
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
