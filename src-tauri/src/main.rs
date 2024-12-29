// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lexai_lib::run;
// use std::sync::Arc;

// mod db;
// use db::{Document, DataStore, StoreError};
// use tauri::menu::{Menu, MenuItem, Submenu};




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
    // let store = LanceStore::new().expect("Failed to create store");
    run();

}
 