use std::sync::{Arc, Mutex};
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tracing::error;

use serde::Serialize;

use crate::services::InferenceCommand;
use crate::state::AppState;

#[derive(Debug)]
pub enum AudioCommand {
    Start,
    Stop,
}

pub fn spawn_audio_actor(
    app_handle: AppHandle,
    inference_sender: mpsc::Sender<InferenceCommand>,
) -> mpsc::Sender<AudioCommand> {
    let (sender, mut receiver) = mpsc::channel(8);

    thread::spawn(move || {
        let mut is_recording = false;
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let mut _current_stream: Option<cpal::Stream> = None;
        let mut current_sample_rate = 16_000_u32;

        while let Some(command) = receiver.blocking_recv() {
            match command {
                AudioCommand::Start => {
                    if is_recording {
                        continue;
                    }

                    clear_buffer(&buffer);

                    match start_input_stream(&buffer) {
                        Ok((stream, sample_rate)) => {
                            if let Err(error) = stream.play() {
                                error!("Не удалось запустить аудио поток: {error}");
                                update_recording_state(&app_handle, false);
                                continue;
                            }

                            _current_stream = Some(stream);
                            current_sample_rate = sample_rate;
                            is_recording = true;
                            update_recording_state(&app_handle, true);
                            let _ = app_handle.emit("audio:started", ());
                        }
                        Err(error) => {
                            error!("Не удалось открыть входной поток: {error}");
                            update_recording_state(&app_handle, false);
                        }
                    }
                }
                AudioCommand::Stop => {
                    if is_recording {
                        is_recording = false;
                        _current_stream = None;
                        let samples = take_buffer(&buffer);
                        let has_samples = !samples.is_empty();

                        if has_samples {
                            let _ = inference_sender.blocking_send(InferenceCommand::Transcribe {
                                samples,
                                sample_rate: current_sample_rate,
                            });
                        }

                        update_recording_state(&app_handle, false);
                        let _ =
                            app_handle.emit("audio:stopped", AudioStoppedPayload { has_samples });
                    }
                }
            }
        }
    });

    sender
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AudioStoppedPayload {
    has_samples: bool,
}

fn update_recording_state(app_handle: &AppHandle, value: bool) {
    let recording_state = {
        let state = app_handle.state::<AppState>();
        Arc::clone(&state.recording_state)
    };

    let lock_result = recording_state.lock();
    if let Ok(mut guard) = lock_result {
        *guard = value;
    }
}

fn start_input_stream(buffer: &Arc<Mutex<Vec<f32>>>) -> Result<(cpal::Stream, u32), String> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| "Не найдено устройство записи".to_string())?;

    let config = device
        .default_input_config()
        .map_err(|error| error.to_string())?;

    let sample_rate = config.sample_rate().0;
    let stream_config: StreamConfig = config.clone().into();
    let buffer = Arc::clone(buffer);

    let err_fn = |error| {
        error!("Ошибка захвата аудио: {error}");
    };

    let stream = match config.sample_format() {
        SampleFormat::F32 => device
            .build_input_stream(
                &stream_config,
                move |data: &[f32], _| push_samples(&buffer, data),
                err_fn,
                None,
            )
            .map_err(|error| error.to_string())?,
        SampleFormat::I16 => device
            .build_input_stream(
                &stream_config,
                move |data: &[i16], _| push_samples(&buffer, data),
                err_fn,
                None,
            )
            .map_err(|error| error.to_string())?,
        SampleFormat::U16 => device
            .build_input_stream(
                &stream_config,
                move |data: &[u16], _| push_samples(&buffer, data),
                err_fn,
                None,
            )
            .map_err(|error| error.to_string())?,
        _ => return Err("Неподдерживаемый формат аудио".to_string()),
    };

    Ok((stream, sample_rate))
}

fn push_samples<T>(buffer: &Arc<Mutex<Vec<f32>>>, data: &[T])
where
    T: Sample,
    f32: cpal::FromSample<T>,
{
    if let Ok(mut guard) = buffer.lock() {
        guard.extend(data.iter().map(|sample| f32::from_sample(*sample)));
    }
}

fn clear_buffer(buffer: &Arc<Mutex<Vec<f32>>>) {
    if let Ok(mut guard) = buffer.lock() {
        guard.clear();
    }
}

fn take_buffer(buffer: &Arc<Mutex<Vec<f32>>>) -> Vec<f32> {
    if let Ok(mut guard) = buffer.lock() {
        return std::mem::take(&mut *guard);
    }

    Vec::new()
}
