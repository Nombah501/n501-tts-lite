import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export type AppConfig = {
  model: string
  modelUrl: string
  modelSha256: string
  modelFilename: string
  recordHotkey: string
}

export type TranscriptionSuccessPayload = {
  text: string
}

export type AudioStoppedPayload = {
  hasSamples: boolean
}

export type AppErrorPayload = {
  error: {
    code: string
    message: string
  }
}

export const startRecord = () => invoke<void>('start_record')

export const stopRecord = () => invoke<void>('stop_record')

export const getConfig = () => invoke<AppConfig>('get_config')

export const updateConfig = (config: AppConfig) =>
  invoke<void>('update_config', { payload: config })

export const onTranscriptionSuccess = (
  handler: (payload: TranscriptionSuccessPayload) => void,
) =>
  listen<TranscriptionSuccessPayload>('transcription:success', (event) =>
    handler(event.payload),
  )

export const onTranscriptionError = (
  handler: (payload: AppErrorPayload) => void,
) => listen<AppErrorPayload>('transcription:error', (event) => handler(event.payload))

export const onAudioStarted = (handler: () => void) =>
  listen('audio:started', () => handler())

export const onAudioStopped = (
  handler: (payload: AudioStoppedPayload) => void,
) => listen<AudioStoppedPayload>('audio:stopped', (event) => handler(event.payload))

export const onConfigUpdated = (
  handler: (payload: { config: AppConfig }) => void,
) =>
  listen<{ config: AppConfig }>('config:updated', (event) => handler(event.payload))
