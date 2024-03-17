use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_nosleep);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<NoSleepApp<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "NoSleepPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_nosleep)?;
  Ok(NoSleepApp(handle))
}

/// Access to the nosleep APIs.
pub struct NoSleepApp<R: Runtime>(PluginHandle<R>);
