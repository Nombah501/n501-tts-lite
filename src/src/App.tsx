import { useEffect } from 'react'

import './App.css'
import { FloatingButton } from './components/FloatingButton'
import { SettingsPanel } from './components/SettingsPanel'
import { StatusPanel } from './components/StatusPanel'
import { useAudioStore } from './store/audioStore'
import { useConfigStore } from './store/configStore'

function App() {
  const { status, lastText, error, init: initAudio, start, stop } =
    useAudioStore()
  const { config, isLoaded, init: initConfig, load, save } = useConfigStore()

  useEffect(() => {
    initAudio()
    initConfig()
    load()
  }, [initAudio, initConfig, load])

  const handleToggle = () => {
    if (status === 'recording') {
      void stop()
      return
    }

    void start()
  }

  return (
    <div className="app">
      {error && <div className="toast">{error}</div>}
      <header className="hero">
        <div className="hero-text">
          <p className="hero-eyebrow">n501-tts-lite</p>
          <h1>Голос в буфере обмена за один жест</h1>
          <p className="hero-subtitle">
            Локальный диктофон для разработчиков: без облаков, без задержек,
            только текст.
          </p>
          <div className="hero-tags">
            <span>offline</span>
            <span>Rust + Tauri</span>
            <span>Whisper</span>
          </div>
        </div>
        <div className="hero-card">
          <StatusPanel
            status={status}
            lastText={lastText}
            error={error}
            recordHotkey={config.recordHotkey}
          />
        </div>
      </header>

      <main className="content">
        <section className="insights">
          <div>
            <h2>Быстрый контур</h2>
            <p>
              Запуск по горячей клавише, запись в один буфер и автоматическая
              расшифровка. Всегда локально.
            </p>
          </div>
          <div>
            <h2>Режимы модели</h2>
            <p>
              Выберите tiny для скорости или base для точности. Конфигурация
              живет в config.yaml.
            </p>
          </div>
        </section>

        {isLoaded && (
          <SettingsPanel
            key={`${config.model}-${config.modelUrl}-${config.modelSha256}-${config.modelFilename}-${config.recordHotkey}`}
            config={config}
            onSave={save}
          />
        )}
      </main>

      <FloatingButton status={status} onToggle={handleToggle} />
    </div>
  )
}

export default App
