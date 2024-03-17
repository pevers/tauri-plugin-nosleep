const COMMANDS: &[&str] = &["prevent_display_sleep", "prevent_system_sleep", "unblock"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
    .ios_path("ios")
    .build();
}
