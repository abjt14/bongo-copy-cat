#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Window;
use rdev::{listen, Event};

#[tauri::command]
fn listener(window: Window) {

    let emit_event = move || {
        if let Err(error) = window.eval("document.querySelector('button').click()") {
            println!("Error: {:?}", error)
        };
    };

    let callback = move |event: Event| {
        match event.name {
            Some(_string) => emit_event(),
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
