// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod color_space;
mod input_output;
mod cameras;
mod state;
mod common;

use state::{get_init_state, get_current_state, merge_state, AppState};
use input_output::{set_input_output};
use color_space::{set_color_space};
use common::POLLING_INTERVAL;
use tauri::{Manager, Emitter};
use std::sync::Mutex;

fn main() {
    run()
}

fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_init_state,
            set_input_output,
            set_color_space
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            listen_current_state(app.get_webview_window("main").unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn listen_current_state(window: tauri::WebviewWindow) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(POLLING_INTERVAL));
        let new_state = get_current_state();
        let updates = merge_state(window.app_handle().clone(), new_state);
        window.emit("state-update", updates).unwrap();
    });
}