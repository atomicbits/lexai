// use tauri::State;
mod db;
use db::store::DataStore;

mod document;
use document::base::Document;

mod prelude;
mod embedder;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppState {
    store: DataStore
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // example to add menu items and tray item: https://github.com/narwhal-apps/gitbar/blob/main/src-tauri/src/main.rs 
        .manage(AppState { store: DataStore::new("data/store") })
        .invoke_handler(tauri::generate_handler![greet]) // insert_document, vector_search
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
