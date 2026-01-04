use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rubato::{FftFixedIn, Resampler};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InferenceError {
    #[error("Некорректный ввод: {0}")]
    InvalidInput(String),
    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] io::Error),
    #[error("Ошибка загрузки модели: {0}")]
    Download(String),
    #[error("Неверная контрольная сумма модели: ожидалось {expected}, получено {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("Ошибка ресемплинга: {0}")]
    Resample(String),
    #[error("Инференс не реализован")]
    NotImplemented,
}

#[derive(Debug, Clone)]
pub struct WhisperModelConfig {
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub filename: String,
}

#[derive(Debug, Clone)]
pub struct WhisperEngineConfig {
    pub model: WhisperModelConfig,
    pub cache_dir: PathBuf,
    pub prefer_gpu: bool,
}

pub struct WhisperEngine {
    model_path: PathBuf,
    _prefer_gpu: bool,
}

const MAX_MODEL_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const DOWNLOAD_TIMEOUT_SECS: u64 = 30;

impl WhisperEngine {
    pub fn new(config: WhisperEngineConfig) -> Result<Self, InferenceError> {
        let model_path = config.cache_dir.join(&config.model.filename);
        ensure_model(&model_path, &config.model)?;

        Ok(Self {
            model_path,
            _prefer_gpu: config.prefer_gpu,
        })
    }

    pub fn transcribe(&self, audio: &[f32], sample_rate: u32) -> Result<String, InferenceError> {
        if audio.is_empty() {
            return Err(InferenceError::InvalidInput(
                "Пустой аудиобуфер".to_string(),
            ));
        }

        let resampled = resample_to_16khz(audio, sample_rate)?;
        if resampled.is_empty() {
            return Err(InferenceError::InvalidInput(
                "Аудио пустое после ресемплинга".to_string(),
            ));
        }

        Err(InferenceError::NotImplemented)
    }

    pub fn model_path(&self) -> &Path {
        &self.model_path
    }
}

fn ensure_model(path: &Path, model: &WhisperModelConfig) -> Result<(), InferenceError> {
    if model.url.trim().is_empty()
        || model.sha256.trim().is_empty()
        || model.filename.trim().is_empty()
    {
        return Err(InferenceError::InvalidInput(
            "Не задан URL, SHA256 или имя файла модели".to_string(),
        ));
    }

    if path.exists() {
        let checksum = compute_sha256(path)?;
        if checksum != model.sha256.to_lowercase() {
            return Err(InferenceError::ChecksumMismatch {
                expected: model.sha256.clone(),
                actual: checksum,
            });
        }

        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = path.with_extension("download");
    download_to_path(&model.url, &temp_path)?;

    let checksum = compute_sha256(&temp_path)?;
    if checksum != model.sha256.to_lowercase() {
        let _ = fs::remove_file(&temp_path);
        return Err(InferenceError::ChecksumMismatch {
            expected: model.sha256.clone(),
            actual: checksum,
        });
    }

    fs::rename(&temp_path, path)?;

    Ok(())
}

fn download_to_path(url: &str, path: &Path) -> Result<(), InferenceError> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .timeout_read(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .timeout_write(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .build();

    let response = agent
        .get(url)
        .call()
        .map_err(|error| InferenceError::Download(error.to_string()))?;

    if let Some(length) = response.header("Content-Length") {
        if let Ok(size) = length.parse::<u64>() {
            if size > MAX_MODEL_BYTES {
                return Err(InferenceError::Download(
                    "Файл модели превышает лимит размера".to_string(),
                ));
            }
        }
    }

    let mut reader = response.into_reader();
    let mut file = fs::File::create(path)?;
    let mut buffer = [0_u8; 8192];
    let mut total = 0_u64;

    loop {
        let read = reader.read(&mut buffer).map_err(|error| {
            let _ = fs::remove_file(path);
            InferenceError::Download(error.to_string())
        })?;

        if read == 0 {
            break;
        }

        total += read as u64;
        if total > MAX_MODEL_BYTES {
            let _ = fs::remove_file(path);
            return Err(InferenceError::Download(
                "Файл модели превышает лимит размера".to_string(),
            ));
        }

        if let Err(error) = file.write_all(&buffer[..read]) {
            let _ = fs::remove_file(path);
            return Err(InferenceError::Io(error));
        }
    }

    Ok(())
}

fn compute_sha256(path: &Path) -> Result<String, InferenceError> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}

fn resample_to_16khz(audio: &[f32], sample_rate: u32) -> Result<Vec<f32>, InferenceError> {
    if sample_rate == 16_000 {
        return Ok(audio.to_vec());
    }

    let input_rate = sample_rate as usize;
    let output_rate = 16_000;
    let chunk_size = 1024;
    let channels = 1;

    let sub_chunks = 1;
    let mut resampler = FftFixedIn::new(input_rate, output_rate, chunk_size, sub_chunks, channels)
        .map_err(|error| InferenceError::Resample(error.to_string()))?;

    let mut output_samples = Vec::new();
    let mut offset = 0;

    while offset < audio.len() {
        let end = (offset + chunk_size).min(audio.len());
        let mut chunk = audio[offset..end].to_vec();

        if chunk.len() < chunk_size {
            chunk.resize(chunk_size, 0.0);
        }

        let input = vec![chunk];
        let output = resampler
            .process(&input, None)
            .map_err(|error| InferenceError::Resample(error.to_string()))?;

        if let Some(channel) = output.get(0) {
            output_samples.extend_from_slice(channel);
        }

        offset = end;
    }

    Ok(output_samples)
}
