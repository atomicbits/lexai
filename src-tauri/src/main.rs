// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lexai_lib::run;
use tauri::Manager;


mod storage;
use crate::storage::{
    Database,
    agent::{Agent, AgentStorage},
    assistant::{Assistant, AssistantStorage},
    workflow::{Workflow, WorkflowStorage},
};

use std::{io::Read, path::PathBuf, sync::Arc};

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
    store: DataStore,
    db: Database,
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
    .setup(|app| {
        // Get app data directory
        let app_data_dir = PathBuf::from("./data"); // sqlite db 
        // Create directories if they don't exist
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    
        // Set up database path
        let db_path = app_data_dir.join("database.db");
        println!("Database path: {:?}", db_path);
    
        // Initialize database
        let db = Database::new(db_path.clone()).expect("Failed to create database");
        let conn = rusqlite::Connection::open(db_path)?;
        db.init_internal(&conn).expect("Failed to initialize database");
    
        // Create storage instances
        let agent_storage = AgentStorage::new(&db);
        let assistant_storage = AssistantStorage::new(&db);
        let workflow_storage = WorkflowStorage::new(&db);
    
        // Example: Create test data
        let test_agent = storage::agent::Agent::new(
            "Test Agent".to_string(),
            Some("Description".to_string()),
            "gpt-4".to_string(),
            Some(0.7),
        );

        let assistant = Assistant::new(
            "Test Assistant".to_string(),
            Some("Description".to_string()),
            "gpt-3.5-turbo".to_string(),
            Some(0.8),
        );
    
        let workflow = Workflow::new(
            "Test Workflow".to_string(),
            Some("Description".to_string()),
        );

        let state = AppState { 
            store: DataStore::new("data/store"), 
            db: db
         };
         
        // Store in app state
        app.manage(state);
    
        // Try to create the agent and print result
        // match agent_storage.create(&test_agent) {
        //     Ok(_) => println!("Successfully created test agent"),
        //     Err(e) => println!("Error creating test agent: {}", e),
        // }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet]) // insert_document, vector_search
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
 