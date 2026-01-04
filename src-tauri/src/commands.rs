use serde::Deserialize;
use tauri::{AppHandle, Emitter, State};

use crate::models::{AppConfig, AppError, ConfigUpdatedPayload, ModelPreset};
use crate::services::AudioCommand;
#[cfg(desktop)]
use crate::services::{parse_record_hotkey, rebind_record_hotkey};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigPayload {
  pub model_preset: String,
  pub model: String,
  pub model_url: String,
  pub model_sha256: String,
  pub model_filename: String,
  pub record_hotkey: String,
}

#[tauri::command]
pub async fn start_record(state: State<'_, AppState>) -> Result<(), AppError> {
  state
    .audio_sender
    .send(AudioCommand::Start)
    .await
    .map_err(|error| AppError::new("AUDIO_SEND", error.to_string()))?;

  Ok(())
}

#[tauri::command]
pub async fn stop_record(state: State<'_, AppState>) -> Result<(), AppError> {
  state
    .audio_sender
    .send(AudioCommand::Stop)
    .await
    .map_err(|error| AppError::new("AUDIO_SEND", error.to_string()))?;

  Ok(())
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> Result<AppConfig, AppError> {
  Ok(state.config_manager.get())
}

#[tauri::command]
pub fn update_config(
  app_handle: AppHandle,
  state: State<'_, AppState>,
  payload: UpdateConfigPayload,
) -> Result<(), AppError> {
  #[cfg(desktop)]
  let shortcut = parse_record_hotkey(&payload.record_hotkey)?;

  let new_config = AppConfig {
    model_preset: payload.model_preset,
    model: payload.model,
    model_url: payload.model_url,
    model_sha256: payload.model_sha256,
    model_filename: payload.model_filename,
    record_hotkey: payload.record_hotkey,
  };

  state.config_manager.update(new_config.clone())?;
  app_handle
    .emit("config:updated", ConfigUpdatedPayload { config: new_config.clone() })
    .map_err(|error| AppError::new("CONFIG_EMIT", error.to_string()))?;

  #[cfg(desktop)]
  rebind_record_hotkey(&app_handle, shortcut)?;

  Ok(())
}

#[tauri::command]
pub fn get_preset(preset_name: String) -> Result<ModelPreset, AppError> {
  use crate::models::get_preset;

  get_preset(&preset_name).ok_or_else(|| {
    AppError::new(
      "PRESET_NOT_FOUND",
      format!("Пресет с именем '{}' не найден", preset_name),
    )
  })
}
