use serde::Deserialize;
use tauri::{AppHandle, Emitter, State};

use crate::models::{AppConfig, AppError, ConfigUpdatedPayload};
use crate::services::AudioCommand;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigPayload {
  pub model: String,
  pub model_url: String,
  pub model_sha256: String,
  pub model_filename: String,
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
  let new_config = AppConfig {
    model: payload.model,
    model_url: payload.model_url,
    model_sha256: payload.model_sha256,
    model_filename: payload.model_filename,
  };

  state.config_manager.update(new_config.clone())?;
  app_handle
    .emit("config:updated", ConfigUpdatedPayload { config: new_config })
    .map_err(|error| AppError::new("CONFIG_EMIT", error.to_string()))?;

  Ok(())
}
