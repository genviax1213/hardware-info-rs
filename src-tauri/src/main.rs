// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod hardware;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_hardware_info,
            commands::get_hardware_live,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
