// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parser;

use shared::Log;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_ignore_cursor_events(true).unwrap();
            window.set_decorations(false).unwrap();

            let (sender, receiver) = std::sync::mpsc::sync_channel::<Log>(8);
            let path = std::path::Path::new(
                "C:\\Program Files (x86)\\Grinding Gear Games\\Path of Exile\\logs\\Client.txt",
            );
            tauri::async_runtime::spawn(async move { parser::parse_log_tail(path, sender) });

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_millis(500));
                for log in receiver {
                    window.emit("log-parse", log).unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
