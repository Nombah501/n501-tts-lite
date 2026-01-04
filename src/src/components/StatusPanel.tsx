import type { AudioStatus } from '../store/audioStore'

type StatusPanelProps = {
  status: AudioStatus
  lastText: string
  error: string | null
  recordHotkey: string
}

const statusText: Record<AudioStatus, string> = {
  idle: 'Готов к записи',
  recording: 'Идет запись',
  processing: 'Расшифровка',
}

export const StatusPanel = ({
  status,
  lastText,
  error,
  recordHotkey,
}: StatusPanelProps) => {
  return (
    <section className="status">
      <div className="status-row">
        <span className={`status-dot status-${status}`} />
        <p className="status-text">{statusText[status]}</p>
        {status === 'recording' && (
          <span className="status-pill">REC</span>
        )}
        {status === 'processing' && (
          <span className="status-pill status-pill-processing">AI</span>
        )}
      </div>
      <p className="status-hotkey">
        Хоткей: <span>{recordHotkey}</span>
      </p>
      {error ? (
        <p className="status-error">{error}</p>
      ) : (
        <p className="status-output">
          {lastText || 'Последняя расшифровка появится здесь'}
        </p>
      )}
    </section>
  )
}
