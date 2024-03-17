use nosleep::{NoSleep, NoSleepTrait};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::sync::Mutex;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::NoSleepApp;
#[cfg(mobile)]
use mobile::NoSleepApp;

struct NoSleepState {
    no_sleep: Mutex<NoSleep>,
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the nosleep APIs.
pub trait NosleepExt<R: Runtime> {
    fn nosleep(&self) -> &NoSleepApp<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NosleepExt<R> for T {
    fn nosleep(&self) -> &NoSleepApp<R> {
        self.state::<NoSleepApp<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nosleep")
        .invoke_handler(tauri::generate_handler![commands::prevent_display_sleep, commands::prevent_system_sleep, commands::unblock])
        .setup(|app, api| {
            #[cfg(mobile)]
            let nosleep = mobile::init(app, api)?;
            #[cfg(desktop)]
            let nosleep = desktop::init(app, api)?;
            app.manage(nosleep);

            // manage state so it is accessible by the commands
            app.manage(NoSleepState {
                no_sleep: Mutex::new(NoSleep::new()?),
            });
            Ok(())
        })
        .build()
}
