use tauri::menu::{Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // .setup(|app| {
        //     app.menu().insert(Submenu::new(
        //         title: "",
        //         menu: Menu::new().add_n
        //     ));
        //     Ok(())
        // })
        // .setup(|app| {
        //     let handle = app.handle();
        //     let submenu = Submenu::with_items(
        //         handle,
        //         "File",
        //         true,
        //         &[&MenuItem::with_id(handle, "select-folder", "Select &Folder", true, None::<&str>)?]
        //     )?;
            
        //     let menu = Menu::with_items(handle, &[&submenu])?;
            
        //     app.set_menu(menu)?;
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}