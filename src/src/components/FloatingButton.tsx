import type { AudioStatus } from '../store/audioStore'

const labelMap: Record<AudioStatus, string> = {
  idle: 'Начать запись',
  recording: 'Остановить',
  processing: 'Обработка...',
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
    >
      <span className="floating-indicator" />
      {labelMap[status]}
    </button>
  )
}
