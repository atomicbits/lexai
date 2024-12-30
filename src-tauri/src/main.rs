// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lexai_lib::run;

mod vectordb;
use vectordb::store::DataStore;

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


// #[tauri::command]
// async fn insert_document(
//     state: State<'_, AppState>,
//     document: Document,
// ) -> Result<(), StoreError> {
//     state.store.insert(document)
// }

// #[tauri::command]
// async fn vector_search(
//     state: State<'_, AppState>,
//     query_vector: Vec<f32>,
//     limit: usize,
// ) -> Result<Vec<SearchResult>, StoreError> {
//     state.store.search(query_vector, limit)
// }

// // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }


fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    // example to add menu items and tray item: https://github.com/narwhal-apps/gitbar/blob/main/src-tauri/src/main.rs 
    .manage(AppState { store: DataStore::new("data/store") })
    .invoke_handler(tauri::generate_handler![greet]) // insert_document, vector_search
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
 