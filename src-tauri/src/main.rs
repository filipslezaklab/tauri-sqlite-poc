// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use setup::setup_password_file;

mod setup;
mod yubi;

use yubi::{decrypt_message, encrypt_input, list_yk, select_yubikey};

#[derive(Debug, Serialize, Deserialize, Default)]
struct AppState {
    selected_serial: Mutex<Option<String>>,
    original_input: Mutex<Option<String>>,
    encrypted_input: Mutex<Option<Vec<u8>>>,
    decrypted_input: Mutex<Option<String>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            setup_password_file(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_yk,
            select_yubikey,
            encrypt_input,
            decrypt_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
