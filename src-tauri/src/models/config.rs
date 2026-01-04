use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub model: String,
    pub model_url: String,
    pub model_sha256: String,
    pub model_filename: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: "tiny".to_string(),
            model_url: String::new(),
            model_sha256: String::new(),
            model_filename: "whisper-tiny.bin".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUpdatedPayload {
    pub config: AppConfig,
}
