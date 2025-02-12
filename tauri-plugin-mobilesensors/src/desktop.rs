use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Mobilesensors<R>> {
  Ok(Mobilesensors(app.clone()))
}

/// Access to the mobilesensors APIs.
pub struct Mobilesensors<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Mobilesensors<R> {
  pub fn get_orientation(&self) -> crate::Result<OrientationData> {
    Ok(OrientationData {
      orientation: -1.0
    })
  }
}
