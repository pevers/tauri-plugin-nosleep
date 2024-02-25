use nosleep::NoSleepType;
use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{NoSleepState, Result};

#[command]
pub(crate) async fn block<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>,
    no_sleep_type: NoSleepType,
) -> Result<()> {
    state.no_sleep.lock().unwrap().start(no_sleep_type)?;
    Ok(())
}

#[command]
pub(crate) async fn unblock<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>,
) -> Result<()> {
    state.no_sleep.lock().unwrap().stop()?;
    Ok(())
}
