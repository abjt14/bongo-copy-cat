#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Window;

#[cfg(target_os = "macos")]
use rdev::{listen, Event};

#[cfg(target_os = "windows")]
use inputbot::KeybdKey;

#[cfg(target_os = "macos")]
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

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn listener(window: Window) {
    let emit_event = move || {
        if let Err(error) = window.eval("document.querySelector('button').click()") {
            println!("Error: {:?}", error)
        };
    };

    KeybdKey::bind_all(move |event| {
        match inputbot::from_keybd_key(event) {
            Some(c) => emit_event(),
            None => (),
        };
    });

    inputbot::handle_input_events();
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![listener])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
