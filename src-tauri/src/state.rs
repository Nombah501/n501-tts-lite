use tokio::sync::mpsc;

use crate::services::{AudioCommand, ConfigManager, InferenceCommand};

pub struct AppState {
    pub audio_sender: mpsc::Sender<AudioCommand>,
    pub inference_sender: mpsc::Sender<InferenceCommand>,
    pub config_manager: ConfigManager,
}
