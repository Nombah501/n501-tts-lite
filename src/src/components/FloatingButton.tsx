import type { AudioStatus } from '../store/audioStore'

const BUTTON_LABELS: Record<AudioStatus, string> = {
  idle: 'Начать запись',
  recording: 'Остановить',
  processing: 'Обработка...',
}

const BUTTON_ARIA_LABELS: Record<AudioStatus, string> = {
  idle: 'Начать запись голоса',
  recording: 'Остановить запись',
  processing: 'Идет обработка аудио',
}

type FloatingButtonProps = {
  status: AudioStatus
  onToggle: () => void
}

export const FloatingButton = ({ status, onToggle }: FloatingButtonProps) => {
  const isBusy = status === 'processing'
  const isRecording = status === 'recording'

  return (
    <button
      className={`floating-button ${isRecording ? 'is-recording' : ''}`}
      onClick={onToggle}
      disabled={isBusy}
      type="button"
      aria-label={BUTTON_ARIA_LABELS[status]}
      aria-pressed={isRecording}
      aria-busy={isBusy}
    >
      <span className="floating-indicator" aria-hidden="true" />
      {BUTTON_LABELS[status]}
    </button>
  )
}
