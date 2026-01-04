import { useState } from 'react'

import { useConfigStore } from '../store/configStore'

import type { AppConfig } from '../services/tauri'
import { getPreset } from '../services/tauri'

type ModelPresetOption = {
  value: string
  label: string
}

type SettingsPanelProps = {
  config: AppConfig
  onSave: (next: AppConfig) => void
  download: {
    asset: string | null
    downloadedBytes: number
    totalBytes: number | null
    progress: number | null
  }
  isDownloading: boolean
}

const PRESET_OPTIONS: ModelPresetOption[] = [
  { value: 'tiny', label: 'Whisper Tiny' },
  { value: 'base', label: 'Whisper Base' },
  { value: 'medium', label: 'Whisper Medium' },
]

const HOTKEY_OPTIONS = [
  { value: 'ctrl+shift+space', label: 'Ctrl+Shift+Space' },
  { value: 'alt+shift+space', label: 'Alt+Shift+Space' },
  { value: 'cmd+shift+space', label: 'Cmd+Shift+Space (macOS)' },
]

const CUSTOM_PRESET_VALUE = 'custom'

export const SettingsPanel = ({
  config,
  onSave,
  download,
  isDownloading,
}: SettingsPanelProps) => {
  const [modelName, setModelName] = useState(config.model)
  const [modelUrl, setModelUrl] = useState(config.modelUrl)
  const [modelSha256, setModelSha256] = useState(config.modelSha256)
  const [modelFilename, setModelFilename] = useState(config.modelFilename)
  const [recordHotkey, setRecordHotkey] = useState(config.recordHotkey)
  const [isSaving, setIsSaving] = useState(false)

  const { selectPreset } = useConfigStore()

  const handlePresetChange = async (presetName: string) => {
    try {
      await selectPreset(presetName)
    } catch (error) {
      console.error('Ошибка выбора пресета:', error)
    }
  }

  const handleClearManual = () => {
    setModelName('')
    setModelUrl('')
    setModelSha256('')
    setModelFilename('')
  }

  const isCustomPreset =
    config.modelPreset === '' || config.modelPreset === CUSTOM_PRESET_VALUE
  const percent = download.progress != null ? Math.round(download.progress * 100) : null

  const isValidConfig = (): boolean => {
    if (isCustomPreset) {
      return (
        modelName.trim().length > 0 &&
        modelUrl.trim().length > 0 &&
        modelSha256.trim().length === 64 &&
        modelFilename.trim().length > 0
      )
    }
    return true
  }

  const handleSave = async () => {
    if (!isValidConfig()) {
      console.error('Некорректная конфигурация')
      return
    }

    setIsSaving(true)

    try {
      const preset =
        !isCustomPreset && config.modelPreset
          ? await getPreset(config.modelPreset)
          : null

      const newConfig: AppConfig = preset
        ? {
            modelPreset: config.modelPreset,
            model: preset.name,
            modelUrl: preset.url,
            modelSha256: preset.sha256,
            modelFilename: preset.filename,
            recordHotkey,
          }
        : {
            modelPreset: CUSTOM_PRESET_VALUE,
            model: modelName.trim(),
            modelUrl: modelUrl.trim(),
            modelSha256: modelSha256.trim(),
            modelFilename: modelFilename.trim(),
            recordHotkey,
          }

      onSave(newConfig)
    } catch (error) {
      console.error('Ошибка сохранения конфигурации:', error)
    } finally {
      setIsSaving(false)
    }
  }

  return (
    <section className="panel">
      <div className="panel-header">
        <div>
          <p className="panel-eyebrow">Настройки</p>
          <h2>Модель распознавания</h2>
        </div>
        <span className="panel-pill">offline-first</span>
      </div>
      <div className="panel-body">
        <label className="field">
          <span className="field-label">Пресет модели</span>
          <select
            className="field-input"
            value={isCustomPreset ? '' : config.modelPreset}
            onChange={(event) => handlePresetChange(event.target.value)}
            disabled={isDownloading || isSaving}
          >
            <option value="">Ручной ввод</option>
            {PRESET_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>

        {isCustomPreset && (
          <>
            <label className="field">
              <span className="field-label">Имя модели</span>
              <input
                className="field-input"
                type="text"
                value={modelName}
                onChange={(event) => setModelName(event.target.value)}
                disabled={isDownloading || isSaving}
                placeholder="Например: whisper-tiny"
              />
            </label>
            <label className="field">
              <span className="field-label">URL модели</span>
              <input
                className="field-input"
                type="url"
                value={modelUrl}
                onChange={(event) => setModelUrl(event.target.value)}
                disabled={isDownloading || isSaving}
                placeholder="https://huggingface.co/..."
              />
            </label>
            <label className="field">
              <span className="field-label">SHA256</span>
              <input
                className="field-input"
                type="text"
                value={modelSha256}
                onChange={(event) => setModelSha256(event.target.value)}
                disabled={isDownloading || isSaving}
                placeholder="64 hex символа"
                pattern="[a-fA-F0-9]{64}"
                title="SHA256 должен содержать 64 hex символа"
              />
            </label>
            <label className="field">
              <span className="field-label">Имя файла</span>
              <input
                className="field-input"
                type="text"
                value={modelFilename}
                onChange={(event) => setModelFilename(event.target.value)}
                disabled={isDownloading || isSaving}
                placeholder="model.safetensors"
              />
            </label>
          </>
        )}

        <label className="field">
          <span className="field-label">Горячая клавиша записи</span>
          <select
            className="field-input"
            value={recordHotkey}
            onChange={(event) => setRecordHotkey(event.target.value)}
            disabled={isDownloading || isSaving}
          >
            {HOTKEY_OPTIONS.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>

        {isDownloading && (
          <div className="download-panel">
            <div className="download-row">
              <span className="download-label">
                Загрузка: {download.asset ?? 'model'}
              </span>
              {percent != null && (
                <span className="download-percent">{percent}%</span>
              )}
            </div>
            <div className="download-bar">
              <div
                className="download-progress"
                style={{ width: `${percent ?? 0}%` }}
              />
            </div>
          </div>
        )}

        <div className="field-hints">
          <p className="field-hint">
            При выборе пресета URL/SHA/filename подставляются автоматически.
          </p>
          <p className="field-hint">
            Нужны URL и SHA256 для загрузки и проверки модели. Файл сохраняется в
            кэш приложения.
          </p>
        </div>

        <div className="panel-actions">
          <button
            className="panel-action"
            type="button"
            onClick={handleSave}
            disabled={isDownloading || isSaving || !isValidConfig()}
          >
            {isSaving ? 'Сохранение...' : 'Сохранить'}
          </button>
          {isCustomPreset && (
            <button
              className="panel-action-secondary"
              type="button"
              onClick={handleClearManual}
              disabled={isDownloading || isSaving}
            >
              Очистить ручные
            </button>
          )}
        </div>
      </div>
    </section>
  )
}
