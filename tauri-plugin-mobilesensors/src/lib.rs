use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Mobilesensors;
#[cfg(mobile)]
use mobile::Mobilesensors;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mobilesensors APIs.
pub trait MobilesensorsExt<R: Runtime> {
  fn mobilesensors(&self) -> &Mobilesensors<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MobilesensorsExt<R> for T {
  fn mobilesensors(&self) -> &Mobilesensors<R> {
    self.state::<Mobilesensors<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("mobilesensors")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let mobilesensors = mobile::init(app, api)?;
      #[cfg(desktop)]
      let mobilesensors = desktop::init(app, api)?;
      app.manage(mobilesensors);
      Ok(())
    })
    .build()
}
