use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<NoSleepApp<R>> {
    Ok(NoSleepApp(app.clone()))
}

/// Access to the nosleep APIs.
pub struct NoSleepApp<R: Runtime>(AppHandle<R>);
