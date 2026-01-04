use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub model: String,
    pub model_url: String,
    pub model_sha256: String,
    pub model_filename: String,
    #[serde(default = "default_record_hotkey")]
    pub record_hotkey: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: "whisper-tiny".to_string(),
            model_url: "https://huggingface.co/openai/whisper-tiny/resolve/main/model.safetensors"
                .to_string(),
            model_sha256: "7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395"
                .to_string(),
            model_filename: "model.safetensors".to_string(),
            record_hotkey: default_record_hotkey(),
        }
    }
}

pub fn default_record_hotkey() -> String {
    // Базовый хоткей записи: Cmd+Shift+Space на macOS, Ctrl+Shift+Space на остальных.
    if cfg!(target_os = "macos") {
        "cmd+shift+space".to_string()
    } else {
        "ctrl+shift+space".to_string()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdatedPayload {
    pub config: AppConfig,
}
