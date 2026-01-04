mod commands;
mod models;
mod services;
mod state;

use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use tauri::Manager;
#[cfg(desktop)]
use tauri_plugin_global_shortcut::ShortcutState;
#[cfg(desktop)]
use tracing::error;

#[cfg(desktop)]
use crate::models::default_record_hotkey;
use crate::services::{spawn_audio_actor, spawn_inference_actor, ConfigManager};
#[cfg(desktop)]
use crate::services::{parse_record_hotkey, rebind_record_hotkey, AudioCommand};
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
      let audio_sender = spawn_audio_actor(app_handle.clone(), inference_sender.clone());
      let recording_state = Arc::new(Mutex::new(false));

      app.manage(AppState {
        audio_sender,
        config_manager,
        recording_state: Arc::clone(&recording_state),
      });

      #[cfg(desktop)]
      {
        let config = app.state::<AppState>().config_manager.get();

        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, _shortcut, event| {
              if event.state() != ShortcutState::Pressed {
                return;
              }

              let state = app_handle.state::<AppState>();
              let mut guard = match state.recording_state.lock() {
                Ok(value) => value,
                Err(_) => {
                  error!("Не удалось получить состояние записи для хоткея");
                  return;
                }
              };

              let command = if *guard {
                AudioCommand::Stop
              } else {
                AudioCommand::Start
              };

              if state.audio_sender.blocking_send(command).is_err() {
                error!("Не удалось отправить команду аудио по хоткею");
                return;
              }

              *guard = !*guard;
            })
            .build(),
        )?;

        let fallback = default_record_hotkey();
        let shortcut = match parse_record_hotkey(&config.record_hotkey) {
          Ok(value) => value,
          Err(error) => {
            error!(
              "Не удалось разобрать хоткей '{}': {}",
              config.record_hotkey, error.message
            );
            parse_record_hotkey(&fallback).map_err(|error| anyhow!(error.message))?
          }
        };

        rebind_record_hotkey(app.handle(), shortcut).map_err(|error| anyhow!(error.message))?;
      }

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
