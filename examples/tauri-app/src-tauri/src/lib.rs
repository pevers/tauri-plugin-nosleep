// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_nosleep::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
