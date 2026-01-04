import type { AudioStatus } from '../store/audioStore'

type StatusPanelProps = {
  status: AudioStatus
  lastText: string
  error: string | null
  recordHotkey: string
}

const STATUS_TEXTS: Record<AudioStatus, string> = {
  idle: 'Готов к записи',
  recording: 'Идет запись',
  processing: 'Расшифровка',
}

const STATUS_ARIA_LABELS: Record<AudioStatus, string> = {
  idle: 'Состояние: готов к записи',
  recording: 'Состояние: идет запись',
  processing: 'Состояние: обработка аудио',
}

export const StatusPanel = ({
  status,
  lastText,
  error,
  recordHotkey,
}: StatusPanelProps) => {
  return (
    <section className="status" aria-live="polite">
      <div className="status-row">
        <span
          className={`status-dot status-${status}`}
          aria-label={STATUS_ARIA_LABELS[status]}
        />
        <p className="status-text">{STATUS_TEXTS[status]}</p>
        {status === 'recording' && (
          <span className="status-pill" aria-label="Запись">
            REC
          </span>
        )}
        {status === 'processing' && (
          <span className="status-pill status-pill-processing" aria-label="Обработка">
            AI
          </span>
        )}
      </div>
      <p className="status-hotkey">
        Хоткей: <span>{recordHotkey}</span>
      </p>
      {error ? (
        <p className="status-error" role="alert" aria-live="assertive">
          {error}
        </p>
      ) : (
        <p className="status-output" aria-live="polite">
          {lastText || 'Последняя расшифровка появится здесь'}
        </p>
      )}
    </section>
  )
}
