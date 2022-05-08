# tauri-plugin-nosleep

Tauri plugin to block the power save functionality in the OS

```rust
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_nosleep::init())
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
```