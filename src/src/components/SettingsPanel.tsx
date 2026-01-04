import { useState } from 'react'

import type { AppConfig } from '../services/tauri'

type SettingsPanelProps = {
  config: AppConfig
  onSave: (next: AppConfig) => void
}

export const SettingsPanel = ({ config, onSave }: SettingsPanelProps) => {
  const [modelName, setModelName] = useState(config.model)
  const [modelUrl, setModelUrl] = useState(config.modelUrl)
  const [modelSha256, setModelSha256] = useState(config.modelSha256)
  const [modelFilename, setModelFilename] = useState(config.modelFilename)
  const [recordHotkey, setRecordHotkey] = useState(config.recordHotkey)

  const hotkeyOptions = [
    { value: 'ctrl+shift+space', label: 'Ctrl+Shift+Space' },
    { value: 'alt+shift+space', label: 'Alt+Shift+Space' },
    { value: 'cmd+shift+space', label: 'Cmd+Shift+Space (macOS)' },
  ]

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
          <span className="field-label">Имя модели</span>
          <input
            className="field-input"
            type="text"
            value={modelName}
            onChange={(event) => setModelName(event.target.value)}
          />
        </label>
        <label className="field">
          <span className="field-label">URL модели</span>
          <input
            className="field-input"
            type="text"
            value={modelUrl}
            onChange={(event) => setModelUrl(event.target.value)}
            placeholder="https://..."
          />
        </label>
        <label className="field">
          <span className="field-label">SHA256</span>
          <input
            className="field-input"
            type="text"
            value={modelSha256}
            onChange={(event) => setModelSha256(event.target.value)}
            placeholder="64 hex"
          />
        </label>
        <label className="field">
          <span className="field-label">Имя файла</span>
          <input
            className="field-input"
            type="text"
            value={modelFilename}
            onChange={(event) => setModelFilename(event.target.value)}
          />
        </label>
        <label className="field">
          <span className="field-label">Горячая клавиша записи</span>
          <select
            className="field-input"
            value={recordHotkey}
            onChange={(event) => setRecordHotkey(event.target.value)}
          >
            {hotkeyOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <p className="field-hint">
          Можно менять хоткей между Ctrl/Alt и Cmd для macOS. Изменение применяется
          сразу после сохранения.
        </p>
        <p className="field-hint">
          Нужны URL и SHA256 для загрузки и проверки модели. Файл сохраняется в
          кэш приложения.
        </p>
        <button
          className="panel-action"
          type="button"
          onClick={() =>
            onSave({
              ...config,
              model: modelName,
              modelUrl,
              modelSha256,
              modelFilename,
              recordHotkey,
            })
          }
        >
          Сохранить
        </button>
      </div>
    </section>
  )
}
