use std::sync::Arc;

use arboard::Clipboard;
use inference::{InferenceError, WhisperEngine, WhisperEngineConfig, WhisperModelConfig};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;

use crate::models::{AppConfig, AppError};
use crate::services::ConfigManager;

#[derive(Debug)]
pub enum InferenceCommand {
  Transcribe { samples: Vec<f32>, sample_rate: u32 },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TranscriptionSuccessPayload {
  text: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TranscriptionErrorPayload {
  error: AppError,
}

pub fn spawn_inference_actor(
  app_handle: AppHandle,
  config_manager: ConfigManager,
) -> mpsc::Sender<InferenceCommand> {
  let cache_dir = app_handle
    .path()
    .app_cache_dir()
    .unwrap_or_else(|_| app_handle.path().app_data_dir().unwrap_or_default());

  let cache_dir: Arc<std::path::PathBuf> = Arc::new(cache_dir);
  let (sender, mut receiver) = mpsc::channel(8);

  tokio::spawn(async move {
    let mut cached_engine: Option<WhisperEngine> = None;
    let mut last_model_key: Option<String> = None;

    while let Some(command) = receiver.recv().await {
      match command {
        InferenceCommand::Transcribe { samples, sample_rate } => {
          let config = config_manager.get();
          let model_key = model_key(&config);

          if last_model_key.as_ref() != Some(&model_key) {
            cached_engine = None;
            last_model_key = Some(model_key);
          }

          let model_config = match build_model_config(&config) {
            Ok(value) => value,
            Err(error) => {
              let _ = app_handle.emit(
                "transcription:error",
                TranscriptionErrorPayload { error },
              );
              continue;
            }
          };

          let app_handle = app_handle.clone();
          let cache_dir = Arc::clone(&cache_dir);
          let engine_slot = cached_engine.take();

          let result = tokio::task::spawn_blocking(move || {
            let engine_config = WhisperEngineConfig {
              model: model_config,
              cache_dir: (*cache_dir).clone(),
              prefer_gpu: false,
            };

            let mut engine = if let Some(existing) = engine_slot {
              existing
            } else {
              WhisperEngine::new(engine_config)?
            };

            let text = engine.transcribe(&samples, sample_rate)?;
            Ok::<(WhisperEngine, String), InferenceError>((engine, text))
          })
          .await;

          match result {
            Ok(Ok((engine, text))) => {
              if text.trim().is_empty() {
                let app_error =
                  AppError::new("INFERENCE_EMPTY", "Пустой результат расшифровки");
                let _ = app_handle.emit(
                  "transcription:error",
                  TranscriptionErrorPayload { error: app_error },
                );
                continue;
              }

              if let Err(error) = copy_to_clipboard(&text) {
                let _ = app_handle.emit(
                  "transcription:error",
                  TranscriptionErrorPayload { error },
                );
                continue;
              }

              let _ = app_handle.emit(
                "transcription:success",
                TranscriptionSuccessPayload { text },
              );

              cached_engine = Some(engine);
            }
            Ok(Err(error)) => {
              let app_error = AppError::new("INFERENCE", error.to_string());
              let _ = app_handle.emit(
                "transcription:error",
                TranscriptionErrorPayload { error: app_error },
              );
            }
            Err(error) => {
              let app_error = AppError::new("INFERENCE_PANIC", error.to_string());
              let _ = app_handle.emit(
                "transcription:error",
                TranscriptionErrorPayload { error: app_error },
              );
            }
          }
        }
      }
    }
  });

  sender
}

fn copy_to_clipboard(text: &str) -> Result<(), AppError> {
  let mut clipboard = Clipboard::new().map_err(|error| {
    AppError::new("CLIPBOARD", format!("Не удалось открыть буфер: {error}"))
  })?;

  clipboard
    .set_text(text.to_string())
    .map_err(|error| AppError::new("CLIPBOARD", error.to_string()))?;

  Ok(())
}

fn build_model_config(config: &AppConfig) -> Result<WhisperModelConfig, AppError> {
  if config.model_url.trim().is_empty()
    || config.model_sha256.trim().is_empty()
    || config.model_filename.trim().is_empty()
  {
    return Err(AppError::new(
      "MODEL_CONFIG",
      "Не заданы URL, SHA256 или имя файла модели",
    ));
  }

  Ok(WhisperModelConfig {
    name: config.model.clone(),
    url: config.model_url.clone(),
    sha256: config.model_sha256.clone(),
    filename: config.model_filename.clone(),
  })
}

fn model_key(config: &AppConfig) -> String {
  format!(
    "{}|{}|{}|{}",
    config.model, config.model_url, config.model_sha256, config.model_filename
  )
}
