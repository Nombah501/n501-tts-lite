use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::whisper;
use candle_transformers::quantized_var_builder::VarBuilder as QuantizedVarBuilder;
use rubato::{FftFixedIn, Resampler};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tokenizers::Tokenizer;

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
    #[error("Ошибка конфигурации модели: {0}")]
    ModelConfig(String),
    #[error("Ошибка токенизатора: {0}")]
    Tokenizer(String),
    #[error("Ошибка загрузки весов: {0}")]
    ModelLoad(String),
    #[error("Ошибка инференса: {0}")]
    Inference(String),
}

#[derive(Debug, Clone)]
pub struct WhisperModelConfig {
    pub name: String,
    pub url: String,
    pub sha256: String,
    pub filename: String,
}

#[derive(Clone)]
pub struct WhisperEngineConfig {
    pub model: WhisperModelConfig,
    pub cache_dir: PathBuf,
    pub prefer_gpu: bool,
    pub download_progress: Option<DownloadProgressCallback>,
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub asset: &'static str,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
}

type DownloadProgressCallback = std::sync::Arc<dyn Fn(DownloadProgress) + Send + Sync>;

enum WhisperModel {
    Normal(whisper::model::Whisper),
    Quantized(whisper::quantized_model::Whisper),
}

impl WhisperModel {
    fn config(&self) -> &whisper::Config {
        match self {
            Self::Normal(model) => &model.config,
            Self::Quantized(model) => &model.config,
        }
    }

    fn encoder_forward(&mut self, x: &Tensor, flush: bool) -> candle_core::Result<Tensor> {
        match self {
            Self::Normal(model) => model.encoder.forward(x, flush),
            Self::Quantized(model) => model.encoder.forward(x, flush),
        }
    }

    fn decoder_forward(
        &mut self,
        x: &Tensor,
        xa: &Tensor,
        flush: bool,
    ) -> candle_core::Result<Tensor> {
        match self {
            Self::Normal(model) => model.decoder.forward(x, xa, flush),
            Self::Quantized(model) => model.decoder.forward(x, xa, flush),
        }
    }

    fn decoder_final_linear(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        match self {
            Self::Normal(model) => model.decoder.final_linear(x),
            Self::Quantized(model) => model.decoder.final_linear(x),
        }
    }
}

pub struct WhisperEngine {
    model_path: PathBuf,
    model: WhisperModel,
    tokenizer: Tokenizer,
    config: whisper::Config,
    mel_filters: Vec<f32>,
    device: Device,
    _prefer_gpu: bool,
}

const MAX_MODEL_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const DOWNLOAD_TIMEOUT_SECS: u64 = 30;

impl WhisperEngine {
    pub fn new(config: WhisperEngineConfig) -> Result<Self, InferenceError> {
        let model_path = config.cache_dir.join(&config.model.filename);
        let progress = config.download_progress.clone();
        ensure_model(&model_path, &config.model, progress.as_ref())?;

        let (config_path, tokenizer_path) =
            ensure_model_assets(&config.cache_dir, &config.model, progress.as_ref())?;
        let config_contents = fs::read_to_string(&config_path)
            .map_err(|error| InferenceError::ModelConfig(error.to_string()))?;
        let whisper_config: whisper::Config = serde_json::from_str(&config_contents)
            .map_err(|error| InferenceError::ModelConfig(error.to_string()))?;
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|error| InferenceError::Tokenizer(error.to_string()))?;

        let device = select_device(config.prefer_gpu)?;
        let mel_filters = build_mel_filters(
            whisper_config.num_mel_bins,
            whisper::N_FFT,
            whisper::SAMPLE_RATE as u32,
        )?;
        let model = load_model(&model_path, &device, &whisper_config)?;

        Ok(Self {
            model_path,
            model,
            tokenizer,
            config: whisper_config,
            mel_filters,
            device,
            _prefer_gpu: config.prefer_gpu,
        })
    }

    pub fn transcribe(
        &mut self,
        audio: &[f32],
        sample_rate: u32,
    ) -> Result<String, InferenceError> {
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

        let mel = whisper::audio::pcm_to_mel(&self.config, &resampled, &self.mel_filters);
        if mel.is_empty() {
            return Err(InferenceError::InvalidInput(
                "Не удалось получить мел-спектрограмму".to_string(),
            ));
        }

        let mel_len = mel.len();
        let mel = Tensor::from_vec(
            mel,
            (
                1,
                self.config.num_mel_bins,
                mel_len / self.config.num_mel_bins,
            ),
            &self.device,
        )
        .map_err(|error| InferenceError::Inference(error.to_string()))?;

        decode_greedy(&mut self.model, &self.tokenizer, &mel, &self.device)
    }

    pub fn model_path(&self) -> &Path {
        &self.model_path
    }
}

fn ensure_model_assets(
    cache_dir: &Path,
    model: &WhisperModelConfig,
    progress: Option<&DownloadProgressCallback>,
) -> Result<(PathBuf, PathBuf), InferenceError> {
    let config_path = cache_dir.join("config.json");
    let tokenizer_path = cache_dir.join("tokenizer.json");

    if config_path.exists() && tokenizer_path.exists() {
        return Ok((config_path, tokenizer_path));
    }

    let base_url = derive_base_url(&model.url, &model.filename).ok_or_else(|| {
        InferenceError::InvalidInput(
            "Не удалось определить базовый URL для config.json и tokenizer.json".to_string(),
        )
    })?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !config_path.exists() {
        let url = format!("{base_url}/config.json");
        download_to_path(&url, &config_path, "config", progress).map_err(|error| {
            InferenceError::Download(format!(
                "{error}. Ожидается config.json рядом с моделью или по адресу {url}"
            ))
        })?;
    }

    if !tokenizer_path.exists() {
        let url = format!("{base_url}/tokenizer.json");
        download_to_path(&url, &tokenizer_path, "tokenizer", progress).map_err(|error| {
            InferenceError::Download(format!(
                "{error}. Ожидается tokenizer.json рядом с моделью или по адресу {url}"
            ))
        })?;
    }

    Ok((config_path, tokenizer_path))
}

fn derive_base_url(url: &str, filename: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.ends_with(filename) {
        let base = trimmed.trim_end_matches(filename).trim_end_matches('/');
        Some(base.to_string())
    } else {
        None
    }
}

fn select_device(prefer_gpu: bool) -> Result<Device, InferenceError> {
    if prefer_gpu {
        #[cfg(feature = "cuda")]
        if let Ok(device) = Device::new_cuda(0) {
            return Ok(device);
        }
        #[cfg(feature = "metal")]
        if let Ok(device) = Device::new_metal(0) {
            return Ok(device);
        }
    }

    Ok(Device::Cpu)
}

fn load_model(
    model_path: &Path,
    device: &Device,
    config: &whisper::Config,
) -> Result<WhisperModel, InferenceError> {
    let extension = model_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    if extension == "gguf" {
        let vb = QuantizedVarBuilder::from_gguf(model_path, device)
            .map_err(|error| InferenceError::ModelLoad(error.to_string()))?;
        let model = whisper::quantized_model::Whisper::load(&vb, config.clone())
            .map_err(|error| InferenceError::ModelLoad(error.to_string()))?;
        return Ok(WhisperModel::Quantized(model));
    }

    if extension == "safetensors" {
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path.to_path_buf()], whisper::DTYPE, device)
                .map_err(|error| InferenceError::ModelLoad(error.to_string()))?
        };
        let model = whisper::model::Whisper::load(&vb, config.clone())
            .map_err(|error| InferenceError::ModelLoad(error.to_string()))?;
        return Ok(WhisperModel::Normal(model));
    }

    Err(InferenceError::InvalidInput(format!(
        "Неподдерживаемый формат модели: {extension}"
    )))
}

fn decode_greedy(
    model: &mut WhisperModel,
    tokenizer: &Tokenizer,
    mel: &Tensor,
    device: &Device,
) -> Result<String, InferenceError> {
    let audio_features = model
        .encoder_forward(mel, true)
        .map_err(|error| InferenceError::Inference(error.to_string()))?;

    let sot_token = token_id(tokenizer, whisper::SOT_TOKEN)?;
    let transcribe_token = token_id(tokenizer, whisper::TRANSCRIBE_TOKEN)?;
    let no_timestamps_token = token_id(tokenizer, whisper::NO_TIMESTAMPS_TOKEN)?;
    let eot_token = token_id(tokenizer, whisper::EOT_TOKEN)?;

    let mut tokens = vec![sot_token, transcribe_token, no_timestamps_token];
    let max_len = model.config().max_target_positions;

    for step in 0..max_len {
        let tokens_t = Tensor::new(tokens.as_slice(), device)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let tokens_t = tokens_t
            .unsqueeze(0)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let ys = model
            .decoder_forward(&tokens_t, &audio_features, step == 0)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let (_, seq_len, _) = ys
            .dims3()
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let last_step = ys
            .i((..1, seq_len - 1..))
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let logits = model
            .decoder_final_linear(&last_step)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let logits = logits
            .i(0)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let logits = logits
            .i(0)
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let logits_v: Vec<f32> = logits
            .to_vec1()
            .map_err(|error| InferenceError::Inference(error.to_string()))?;
        let next_token = logits_v
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index as u32)
            .ok_or_else(|| InferenceError::Inference("Пустые логиты".to_string()))?;

        tokens.push(next_token);
        if next_token == eot_token {
            break;
        }
    }

    tokenizer
        .decode(&tokens, true)
        .map_err(|error| InferenceError::Inference(error.to_string()))
}

fn token_id(tokenizer: &Tokenizer, token: &str) -> Result<u32, InferenceError> {
    match tokenizer.token_to_id(token) {
        Some(id) => Ok(id),
        None => Err(InferenceError::Inference(format!(
            "Нет токена {token} в словаре",
        ))),
    }
}

fn build_mel_filters(
    num_mel_bins: usize,
    n_fft: usize,
    sample_rate: u32,
) -> Result<Vec<f32>, InferenceError> {
    if num_mel_bins == 0 || n_fft == 0 || sample_rate == 0 {
        return Err(InferenceError::InvalidInput(
            "Некорректные параметры мел-фильтров".to_string(),
        ));
    }

    let n_freqs = 1 + n_fft / 2;
    let sample_rate = sample_rate as f32;

    let mel_min = hz_to_mel(0.0);
    let mel_max = hz_to_mel(sample_rate / 2.0);
    let mel_step = (mel_max - mel_min) / (num_mel_bins + 1) as f32;

    // Стандартные mel-фильтры по формуле HTK.
    let mut mel_points = Vec::with_capacity(num_mel_bins + 2);
    for i in 0..(num_mel_bins + 2) {
        mel_points.push(mel_min + mel_step * i as f32);
    }

    let mut hz_points = Vec::with_capacity(num_mel_bins + 2);
    for mel in mel_points {
        hz_points.push(mel_to_hz(mel));
    }

    let mut bin_points = Vec::with_capacity(num_mel_bins + 2);
    for hz in hz_points {
        let bin = ((n_fft + 1) as f32 * hz / sample_rate).floor() as usize;
        bin_points.push(bin.min(n_freqs.saturating_sub(1)));
    }

    let mut filters = vec![0.0_f32; num_mel_bins * n_freqs];
    for i in 0..num_mel_bins {
        let left = bin_points[i];
        let center = bin_points[i + 1];
        let right = bin_points[i + 2];

        if center == left || right == center {
            continue;
        }

        for j in left..center {
            let value = (j - left) as f32 / (center - left) as f32;
            filters[i * n_freqs + j] = value;
        }

        for j in center..right {
            let value = (right - j) as f32 / (right - center) as f32;
            filters[i * n_freqs + j] = value;
        }
    }

    Ok(filters)
}

fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10_f32.powf(mel / 2595.0) - 1.0)
}

fn ensure_model(
    path: &Path,
    model: &WhisperModelConfig,
    progress: Option<&DownloadProgressCallback>,
) -> Result<(), InferenceError> {
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
    download_to_path(&model.url, &temp_path, "model", progress)?;

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

fn download_to_path(
    url: &str,
    path: &Path,
    asset: &'static str,
    progress: Option<&DownloadProgressCallback>,
) -> Result<(), InferenceError> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .timeout_read(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .timeout_write(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .build();

    let response = agent
        .get(url)
        .call()
        .map_err(|error| InferenceError::Download(error.to_string()))?;

    let content_length = response
        .header("Content-Length")
        .and_then(|length| length.parse::<u64>().ok());

    if let Some(size) = content_length {
        if size > MAX_MODEL_BYTES {
            return Err(InferenceError::Download(
                "Файл модели превышает лимит размера".to_string(),
            ));
        }
    }

    report_progress(progress, asset, 0, content_length);

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

        report_progress(progress, asset, total, content_length);
    }

    Ok(())
}

fn report_progress(
    progress: Option<&DownloadProgressCallback>,
    asset: &'static str,
    downloaded_bytes: u64,
    total_bytes: Option<u64>,
) {
    if let Some(callback) = progress {
        callback(DownloadProgress {
            asset,
            downloaded_bytes,
            total_bytes,
        });
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_mel_filters_has_expected_shape() {
        let filters = build_mel_filters(80, whisper::N_FFT, whisper::SAMPLE_RATE as u32).unwrap();
        let n_freqs = 1 + whisper::N_FFT / 2;
        assert_eq!(filters.len(), 80 * n_freqs);
    }

    #[test]
    fn build_mel_filters_are_bounded() {
        let filters = build_mel_filters(80, whisper::N_FFT, whisper::SAMPLE_RATE as u32).unwrap();
        let max = filters
            .iter()
            .copied()
            .fold(0.0_f32, |acc, value| acc.max(value));
        let min = filters
            .iter()
            .copied()
            .fold(0.0_f32, |acc, value| acc.min(value));
        assert!(min >= 0.0);
        assert!(max <= 1.0);
    }
}
