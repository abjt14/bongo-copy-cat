#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Window;
use rdev::{listen, Event};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn listener(window: Window) {

    let emit_event = move || {
        window.eval("document.querySelector('button').click()");
    };

    let callback = move |event: Event| {
        match event.name {
            Some(string) => emit_event(),
            None => (),
        }
    };

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![listener])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
