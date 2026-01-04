use serde::{Deserialize, Serialize};

pub const MODEL_PRESET_TINY: &str = "tiny";
pub const MODEL_PRESET_BASE: &str = "base";
pub const MODEL_PRESET_MEDIUM: &str = "medium";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default = "default_model_preset")]
    pub model_preset: String,
    pub model: String,
    pub model_url: String,
    pub model_sha256: String,
    pub model_filename: String,
    #[serde(default = "default_record_hotkey")]
    pub record_hotkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreset {
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub filename: String,
}

pub fn default_model_preset() -> String {
    MODEL_PRESET_TINY.to_string()
}

pub fn get_preset(name: &str) -> Option<ModelPreset> {
    match name {
        MODEL_PRESET_TINY => Some(ModelPreset {
            name: "Whisper Tiny".to_string(),
            url: "https://huggingface.co/openai/whisper-tiny/resolve/main/model.safetensors"
                .to_string(),
            sha256: "7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395".to_string(),
            filename: "model.safetensors".to_string(),
        }),
        MODEL_PRESET_BASE => Some(ModelPreset {
            name: "Whisper Base".to_string(),
            url: "https://huggingface.co/openai/whisper-base/resolve/main/model.safetensors"
                .to_string(),
            sha256: "e37978b90ca9030d5170a5c07aadb050351a65bb".to_string(),
            filename: "model.safetensors".to_string(),
        }),
        MODEL_PRESET_MEDIUM => Some(ModelPreset {
            name: "Whisper Medium".to_string(),
            url: "https://huggingface.co/openai/whisper-medium/resolve/main/model.safetensors"
                .to_string(),
            sha256: "abdf7c39ab9d0397620ccaea8974cc764cd0953e".to_string(),
            filename: "model.safetensors".to_string(),
        }),
        _ => None,
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_preset: default_model_preset(),
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
