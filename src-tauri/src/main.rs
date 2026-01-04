mod commands;
mod models;
mod services;
mod state;

use anyhow::anyhow;
use tauri::Manager;

use crate::services::{spawn_audio_actor, spawn_inference_actor, ConfigManager};
use crate::state::AppState;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle().clone();
      let config_path = app_handle
        .path()
        .app_config_dir()
        .map_err(|error| anyhow!(error.to_string()))?
        .join("config.yaml");

      let config_manager =
        ConfigManager::new(config_path).map_err(|error| anyhow!(error.message))?;

      config_manager
        .start_watcher(app_handle.clone())
        .map_err(|error| anyhow!(error.message))?;

      let inference_sender =
        spawn_inference_actor(app_handle.clone(), config_manager.clone());
      let audio_sender = spawn_audio_actor(inference_sender.clone());

      app.manage(AppState {
        audio_sender,
        inference_sender,
        config_manager,
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::start_record,
      commands::stop_record,
      commands::get_config,
      commands::update_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
