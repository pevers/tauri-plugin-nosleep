use nosleep::NoSleep;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::sync::Mutex;

pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Nosleep;

struct NoSleepState {
    no_sleep: Mutex<NoSleep>,
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the nosleep APIs.
pub trait NosleepExt<R: Runtime> {
    fn nosleep(&self) -> &Nosleep<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NosleepExt<R> for T {
    fn nosleep(&self) -> &Nosleep<R> {
        self.state::<Nosleep<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nosleep")
        .invoke_handler(tauri::generate_handler![commands::block, commands::unblock])
        .setup(|app, api| {
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
