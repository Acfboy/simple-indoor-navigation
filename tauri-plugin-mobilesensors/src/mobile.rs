use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_mobilesensors);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Mobilesensors<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.mobilesensors", "MobilesensorsPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_mobilesensors)?;
  Ok(Mobilesensors(handle))
}

/// Access to the mobilesensors APIs.
pub struct Mobilesensors<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Mobilesensors<R> {
  pub fn get_orientation(&self) -> crate::Result<OrientationData> {
    self
      .0
      .run_mobile_plugin("getOrientation", {})
      .map_err(Into::into)
  }
}
