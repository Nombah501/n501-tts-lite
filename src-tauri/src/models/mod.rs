pub mod app_error;
pub mod config;

pub use app_error::AppError;
pub use config::{default_record_hotkey, AppConfig, ConfigUpdatedPayload};
