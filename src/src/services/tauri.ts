import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

// ============= Types =============

/**
 * Конфигурация приложения
 */
export type AppConfig = {
  modelPreset: string
  model: string
  modelUrl: string
  modelSha256: string
  modelFilename: string
  recordHotkey: string
}

/**
 * Экспортированный пресет модели
 */
export type ModelPresetExport = {
  name: string
  url: string
  sha256: string
  filename: string
}

/**
 * Payload успешной транскрипции
 */
export type TranscriptionSuccessPayload = {
  text: string
}

/**
 * Payload остановки записи
 */
export type AudioStoppedPayload = {
  hasSamples: boolean
}

/**
 * Payload прогресса загрузки модели
 */
export type ModelDownloadProgressPayload = {
  asset: string
  downloadedBytes: number
  totalBytes?: number
}

/**
 * Payload статуса модели
 */
export type ModelStatusPayload = {
  model: string
}

/**
 * Payload ошибки модели
 */
export type ModelErrorPayload = {
  message: string
  kind: string
}

/**
 * Payload ошибки приложения
 */
export type AppErrorPayload = {
  error: {
    code: string
    message: string
  }
}

// ============= Commands =============

/**
 * Запускает запись аудио
 */
export const startRecord = () => invoke<void>('start_record')

/**
 * Останавливает запись аудио
 */
export const stopRecord = () => invoke<void>('stop_record')

/**
 * Получает текущую конфигурацию
 */
export const getConfig = () => invoke<AppConfig>('get_config')

/**
 * Обновляет конфигурацию
 */
export const updateConfig = (config: AppConfig) =>
  invoke<void>('update_config', { payload: config })

/**
 * Получает пресет модели по имени
 */
export const getPreset = (presetName: string) =>
  invoke<ModelPresetExport>('get_preset', { presetName })

// ============= Event Listeners =============

/**
 * Подписывается на успешную транскрипцию
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onTranscriptionSuccess = (
  handler: (payload: TranscriptionSuccessPayload) => void,
): Promise<UnlistenFn> =>
  listen<TranscriptionSuccessPayload>('transcription:success', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на ошибки транскрипции
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onTranscriptionError = (
  handler: (payload: AppErrorPayload) => void,
): Promise<UnlistenFn> =>
  listen<AppErrorPayload>('transcription:error', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на событие начала записи
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onAudioStarted = (handler: () => void): Promise<UnlistenFn> =>
  listen('audio:started', () => handler())

/**
 * Подписывается на событие остановки записи
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onAudioStopped = (
  handler: (payload: AudioStoppedPayload) => void,
): Promise<UnlistenFn> =>
  listen<AudioStoppedPayload>('audio:stopped', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на событие начала загрузки модели
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onModelDownloadStarted = (
  handler: (payload: ModelStatusPayload) => void,
): Promise<UnlistenFn> =>
  listen<ModelStatusPayload>('model:download-started', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на прогресс загрузки модели
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onModelDownloadProgress = (
  handler: (payload: ModelDownloadProgressPayload) => void,
): Promise<UnlistenFn> =>
  listen<ModelDownloadProgressPayload>('model:download-progress', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на событие завершения загрузки модели
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onModelDownloadFinished = (
  handler: (payload: ModelStatusPayload) => void,
): Promise<UnlistenFn> =>
  listen<ModelStatusPayload>('model:download-finished', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на ошибки загрузки модели
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onModelDownloadError = (
  handler: (payload: ModelErrorPayload) => void,
): Promise<UnlistenFn> =>
  listen<ModelErrorPayload>('model:download-error', (event) =>
    handler(event.payload),
  )

/**
 * Подписывается на обновление конфигурации
 * @param handler - обработчик события
 * @returns функция для отписки
 */
export const onConfigUpdated = (
  handler: (payload: { config: AppConfig }) => void,
): Promise<UnlistenFn> =>
  listen<{ config: AppConfig }>('config:updated', (event) =>
    handler(event.payload),
  )
