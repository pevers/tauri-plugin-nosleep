const COMMANDS: &[&str] = &["block", "unblock"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
