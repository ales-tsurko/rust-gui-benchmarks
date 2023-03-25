use std::process::exit;
use std::time::Instant;

use once_cell::sync::OnceCell;
use tauri::{Manager, Window, async_runtime};
use ws::{listen, Message};

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

#[tauri::command]
fn start() -> String {
    let address = "127.0.0.1:5858".to_string();
    let addr_clone = address.clone();
    std::thread::spawn(move || {
        listen(&addr_clone, |out| {
            move |msg| {
                if let Message::Text(msg) = msg {
                    let count: usize = msg.parse().unwrap();

                    if count == 0 {
                        INSTANCE.set(Instant::now()).unwrap();
                    }

                    if count < COUNT_NUM {
                        return out.send(format!("{}", count + 1));
                    } else {
                        let time = INSTANCE.get().unwrap().elapsed().as_nanos();
                        println!("Result: {}", time);
                        return Ok(());
                    }
                }

                Ok(())
            }
        });
    });

    address
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
