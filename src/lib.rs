use nosleep::{NoSleep, NoSleepType};
use serde::{ser::Serializer, Serialize};
use std::sync::Mutex;
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State, Window,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ScreenLockError(#[from] nosleep::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub struct NoSleepState {
    no_sleep: Mutex<NoSleep>,
}

#[command]
async fn block<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>,
    no_sleep_type: NoSleepType,
) -> Result<()> {
    state.no_sleep.lock().unwrap().start(no_sleep_type)?;
    Ok(())
}

#[command]
async fn unblock<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>,
) -> Result<()> {
    state.no_sleep.lock().unwrap().stop()?;
    Ok(())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nosleep")
        .invoke_handler(tauri::generate_handler![block, unblock])
        .setup(|app| {
            app.manage(NoSleepState {
                no_sleep: Mutex::new(NoSleep::new()?),
            });
            Ok(())
        })
        .build()
}
