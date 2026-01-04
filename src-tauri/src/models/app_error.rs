use serde::Serialize;

/// Error code для типизации ошибок приложения
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)] // TODO: использовать при расширении обработки ошибок
pub enum ErrorCode {
    /// Ошибка аудио устройства
    AudioDevice,
    /// Ошибка загрузки модели
    ModelLoad,
    /// Ошибка инференса
    Inference,
    /// Ошибка конфигурации
    Config,
    /// Ошибка ввода-вывода
    Io,
    /// Ошибка отправки команды аудио
    AudioSend,
    /// Ошибка эмиссии события
    ConfigEmit,
    /// Пресет не найден
    PresetNotFound,
    /// Неожиданная ошибка
    Unexpected,
}

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "UNKNOWN".to_string())
    }
}

/// Структура ошибки приложения для Tauri IPC
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    /// Создает новую ошибку с кодом и сообщением
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Создает ошибку с типизированным кодом
    pub fn from_code(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::from_code(ErrorCode::Unexpected, error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::from_code(ErrorCode::Io, error.to_string())
    }
}
