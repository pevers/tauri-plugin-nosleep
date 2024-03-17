// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    #[cfg(target_os = "ios")]
    {
        use objc::runtime::{YES, NO};
        use objc::{class, msg_send, sel, sel_impl};
        unsafe {
            let ui_app: *mut objc::runtime::Object = msg_send![class!(UIApplication), sharedApplication];
            let is_disabled: bool = msg_send![ui_app, isIdleTimerDisabled];
            return match is_disabled {
                YES => {
                    let _: () = msg_send![ui_app, setIdleTimerDisabled: NO];
                    "The idle timer is disabled".to_string()
                }
                NO => {
                    let _: () = msg_send![ui_app, setIdleTimerDisabled: YES];
                    "The idle timer is enabled".to_string()
                }
            }
        }
    }
    "Not running for target ios".to_string()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_nosleep::init())
        .init(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
