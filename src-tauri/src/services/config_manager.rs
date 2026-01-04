use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

use crate::models::{AppConfig, AppError, ConfigUpdatedPayload};

struct ConfigState {
    config: AppConfig,
    last_written: Option<String>,
}

#[derive(Clone)]
pub struct ConfigManager {
    config_path: PathBuf,
    state: Arc<Mutex<ConfigState>>,
    watcher: Arc<Mutex<Option<RecommendedWatcher>>>,
}

impl ConfigManager {
    pub fn new(config_path: PathBuf) -> Result<Self, AppError> {
        let config = load_config(&config_path)?;
        let state = ConfigState {
            config,
            last_written: None,
        };

        Ok(Self {
            config_path,
            state: Arc::new(Mutex::new(state)),
            watcher: Arc::new(Mutex::new(None)),
        })
    }

    pub fn get(&self) -> AppConfig {
        let state = self.state.lock().expect("Config mutex poisoned");
        state.config.clone()
    }

    pub fn update(&self, new_config: AppConfig) -> Result<(), AppError> {
        let yaml = serde_yaml::to_string(&new_config)
            .map_err(|error| AppError::new("CONFIG_SERIALIZE", error.to_string()))?;

        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                AppError::new("CONFIG_IO", format!("Не удалось создать каталог: {error}"))
            })?;
        }

        fs::write(&self.config_path, yaml.as_bytes()).map_err(|error| {
            AppError::new("CONFIG_IO", format!("Не удалось записать конфиг: {error}"))
        })?;

        let mut state = self.state.lock().expect("Config mutex poisoned");
        state.config = new_config;
        state.last_written = Some(yaml);

        Ok(())
    }

    pub fn start_watcher(&self, app_handle: AppHandle) -> Result<(), AppError> {
        let mut watcher_guard = self.watcher.lock().expect("Watcher mutex poisoned");
        if watcher_guard.is_some() {
            return Ok(());
        }

        let config_path = self.config_path.clone();
        let state = Arc::clone(&self.state);
        let app_handle = app_handle.clone();

        let mut watcher =
            notify::recommended_watcher(move |result: Result<notify::Event, notify::Error>| {
                if let Ok(event) = result {
                    let is_relevant = matches!(
                        event.kind,
                        notify::EventKind::Modify(_) | notify::EventKind::Create(_)
                    );

                    if !is_relevant {
                        return;
                    }

                    if let Ok(contents) = fs::read_to_string(&config_path) {
                        let mut state_guard = state.lock().expect("Config mutex poisoned");

                        if state_guard
                            .last_written
                            .as_ref()
                            .is_some_and(|last| last == &contents)
                        {
                            state_guard.last_written = None;
                            return;
                        }

                        if let Ok(updated) = serde_yaml::from_str::<AppConfig>(&contents) {
                            state_guard.config = updated.clone();
                            let _ = app_handle
                                .emit("config:updated", ConfigUpdatedPayload { config: updated });
                        }
                    }
                }
            })
            .map_err(|error| AppError::new("CONFIG_WATCH", error.to_string()))?;

        watch_path(&mut watcher, &self.config_path)?;
        *watcher_guard = Some(watcher);

        Ok(())
    }
}

fn load_config(path: &Path) -> Result<AppConfig, AppError> {
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let contents = fs::read_to_string(path).map_err(|error| {
        AppError::new("CONFIG_IO", format!("Не удалось прочитать конфиг: {error}"))
    })?;

    let config = serde_yaml::from_str::<AppConfig>(&contents)
        .map_err(|error| AppError::new("CONFIG_PARSE", error.to_string()))?;

    Ok(config)
}

fn watch_path(watcher: &mut RecommendedWatcher, path: &Path) -> Result<(), AppError> {
    let target = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| path.to_path_buf());

    watcher
        .watch(&target, RecursiveMode::NonRecursive)
        .map_err(|error| AppError::new("CONFIG_WATCH", error.to_string()))?;

    Ok(())
}
