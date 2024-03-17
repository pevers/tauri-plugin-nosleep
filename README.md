# tauri-plugin-nosleep

![Test](https://github.com/pevers/tauri-plugin-nosleep/workflows/Test/badge.svg)
[![dependency status](https://deps.rs/repo/github/pevers/tauri-plugin-nosleep/status.svg)](https://deps.rs/repo/github/pevers/tauri-plugin-nosleep)

Tauri plugin to block the power save functionality in the OS

```rust
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_nosleep::init())
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
```

Add the NPM package.

```console
npm install tauri-plugin-nosleep-api
# or
yarn add tauri-plugin-nosleep-api
```

Use this within TS/JS.

```js
import { prevent_display_sleep, unblock } from 'tauri-plugin-nosleep-api'
prevent_display_sleep();
// To unblock whenever you are done
unblock();
```


## Supported Platforms

| Platform | Status | Tested |
|----------|--------|--------|
| Linux    | ✔️      | ✔️      |
| macOS    | ✔️      | ✔️      |
| Windows  | ✔️      | ✔️      |
| iOS      | ✔️      | ❌      |
| Android  | ❌      | ❌     |