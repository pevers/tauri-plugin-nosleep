use tauri::{command, AppHandle, Runtime, State, Window};
use nosleep::NoSleepTrait;
use crate::{NoSleepState, Result};

#[command]
pub(crate) async fn prevent_display_sleep<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>
) -> Result<()> {
    state.no_sleep.lock().unwrap().prevent_display_sleep()?;
    Ok(())
}

#[command]
pub(crate) async fn prevent_system_sleep<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, NoSleepState>
) -> Result<()> {
    state.no_sleep.lock().unwrap().prevent_system_sleep()?;
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
