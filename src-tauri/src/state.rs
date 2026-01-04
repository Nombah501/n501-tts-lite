use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

use crate::services::{AudioCommand, ConfigManager};

pub struct AppState {
    pub audio_sender: mpsc::Sender<AudioCommand>,
    pub config_manager: ConfigManager,
    pub recording_state: Arc<Mutex<bool>>,
}
