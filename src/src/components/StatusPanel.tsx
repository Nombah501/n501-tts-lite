import type { AudioStatus } from '../store/audioStore'

type StatusPanelProps = {
  status: AudioStatus
  lastText: string
  error: string | null
}

const statusText: Record<AudioStatus, string> = {
  idle: 'Готов к записи',
  recording: 'Идет запись',
  processing: 'Расшифровка',
}

export const StatusPanel = ({ status, lastText, error }: StatusPanelProps) => {
  return (
    <section className="status">
      <div className="status-row">
        <span className={`status-dot status-${status}`} />
        <p className="status-text">{statusText[status]}</p>
      </div>
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
