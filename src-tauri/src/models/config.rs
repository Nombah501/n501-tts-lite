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
            sha256: "07cadb9f25677c8d50df603e66a98fbd842cce45047139baeb16e6219a1e807b".to_string(),
            filename: "model.safetensors".to_string(),
        }),
        MODEL_PRESET_MEDIUM => Some(ModelPreset {
            name: "Whisper Medium".to_string(),
            url: "https://huggingface.co/openai/whisper-medium/resolve/main/model.safetensors"
                .to_string(),
            sha256: "62f73550fa6db24b0c6f6c5962bd0dae80fa644e93cde9cd9c3792971b47fd28".to_string(),
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

// ============= Tests =============

#[cfg(test)]
mod tests {
    use super::*;

    /// Проверяет, что SHA256 имеет правильный формат (64 hex символа)
    fn is_valid_sha256(sha256: &str) -> bool {
        sha256.len() == 64 && sha256.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Проверяет, что URL начинается с http/https
    fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    #[test]
    fn test_get_preset_tiny() {
        let preset = get_preset(MODEL_PRESET_TINY);
        assert!(preset.is_some(), "Пресет tiny должен существовать");

        let preset = preset.unwrap();
        assert_eq!(preset.name, "Whisper Tiny");
        assert!(is_valid_url(&preset.url));
        assert!(is_valid_sha256(&preset.sha256));
        assert_eq!(preset.filename, "model.safetensors");
    }

    #[test]
    fn test_get_preset_base() {
        let preset = get_preset(MODEL_PRESET_BASE);
        assert!(preset.is_some(), "Пресет base должен существовать");

        let preset = preset.unwrap();
        assert_eq!(preset.name, "Whisper Base");
        assert!(is_valid_url(&preset.url));
        eprintln!("Base SHA256 length: {} (expected: 64)", preset.sha256.len());
        eprintln!("Base SHA256: {}", preset.sha256);
        assert!(
            is_valid_sha256(&preset.sha256),
            "SHA256 должен быть 64 hex символа"
        );
        assert_eq!(preset.filename, "model.safetensors");
    }

    #[test]
    fn test_get_preset_medium() {
        let preset = get_preset(MODEL_PRESET_MEDIUM);
        assert!(preset.is_some(), "Пресет medium должен существовать");

        let preset = preset.unwrap();
        assert_eq!(preset.name, "Whisper Medium");
        assert!(is_valid_url(&preset.url));
        eprintln!(
            "Medium SHA256 length: {} (expected: 64)",
            preset.sha256.len()
        );
        eprintln!("Medium SHA256: {}", preset.sha256);
        assert!(
            is_valid_sha256(&preset.sha256),
            "SHA256 должен быть 64 hex символа"
        );
        assert_eq!(preset.filename, "model.safetensors");
    }

    #[test]
    fn test_get_preset_not_found() {
        let preset = get_preset("nonexistent");
        assert!(
            preset.is_none(),
            "Несуществующий пресет должен возвращать None"
        );
    }

    #[test]
    fn test_default_model_preset() {
        let preset = default_model_preset();
        assert_eq!(preset, MODEL_PRESET_TINY);
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.model_preset, MODEL_PRESET_TINY);
        assert_eq!(config.model, "whisper-tiny");
        assert!(is_valid_url(&config.model_url));
        assert!(is_valid_sha256(&config.model_sha256));
        assert_eq!(config.model_filename, "model.safetensors");
        assert!(!config.record_hotkey.is_empty());
    }

    #[test]
    fn test_default_record_hotkey() {
        let hotkey = default_record_hotkey();
        assert!(!hotkey.is_empty());
        assert!(hotkey.contains("shift"));
        assert!(hotkey.contains("space"));
    }

    #[test]
    fn test_sha256_valid_formats() {
        // Валидные SHA256
        assert!(is_valid_sha256(
            "7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395"
        ));
        assert!(is_valid_sha256(
            "0000000000000000000000000000000000000000000000000000000000000000"
        ));
        assert!(is_valid_sha256(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ));
        assert!(is_valid_sha256(
            "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
        ));

        // Невалидные SHA256
        assert!(!is_valid_sha256("")); // Пустой
        assert!(!is_valid_sha256("7ebd0e69")); // Слишком короткий
        assert!(!is_valid_sha256(
            "7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395abc"
        )); // Слишком длинный
        assert!(!is_valid_sha256(
            "ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395"
        )); // 63 символа
        assert!(!is_valid_sha256(
            "ghij0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395"
        )); // Недопустимые символы
        assert!(!is_valid_sha256(
            "7EBD0E69E78190FFE1438491FA05CC1F5C1AA3A4C4DB3BC1723ADBB551EA239"
        )); // 63 символа (заглавные - ок, но длина нет)
    }

    #[test]
    fn test_url_valid_formats() {
        // Валидные URL
        assert!(is_valid_url("https://example.com/model.safetensors"));
        assert!(is_valid_url("http://localhost/model.bin"));
        assert!(is_valid_url(
            "https://huggingface.co/openai/whisper-tiny/resolve/main/model.safetensors"
        ));

        // Невалидные URL
        assert!(!is_valid_url(""));
        assert!(!is_valid_url("ftp://example.com/model.bin"));
        assert!(!is_valid_url("example.com/model.bin"));
        assert!(!is_valid_url("/path/to/model.bin"));
    }
}
