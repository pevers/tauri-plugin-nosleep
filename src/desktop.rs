use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Nosleep<R>> {
    Ok(Nosleep(app.clone()))
}

/// Access to the nosleep APIs.
pub struct Nosleep<R: Runtime>(AppHandle<R>);
