import { create } from 'zustand'

import {
  onAudioStarted,
  onAudioStopped,
  onTranscriptionError,
  onTranscriptionSuccess,
  startRecord,
  stopRecord,
} from '../services/tauri'

export type AudioStatus = 'idle' | 'recording' | 'processing'

type AudioState = {
  status: AudioStatus
  lastText: string
  error: string | null
  init: () => void
  start: () => Promise<void>
  stop: () => Promise<void>
  clearError: () => void
}

const ERROR_MESSAGES = {
  unknown: 'Неизвестная ошибка',
} as const

const toMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message
  }

  return ERROR_MESSAGES.unknown
}

let listenersBound = false

export const useAudioStore = create<AudioState>((set, get) => ({
  status: 'idle',
  lastText: '',
  error: null,
  init: () => {
    if (listenersBound) {
      return
    }

    listenersBound = true

    onAudioStarted(() => {
      set({ status: 'recording', error: null })
    })

    onAudioStopped((payload) => {
      set({ status: payload.hasSamples ? 'processing' : 'idle' })
    })

    onTranscriptionSuccess((payload) => {
      set({ status: 'idle', lastText: payload.text, error: null })
    })

    onTranscriptionError((payload) => {
      set({ status: 'idle', error: payload.error.message })
    })
  },
  start: async () => {
    set({ status: 'recording', error: null })

    try {
      await startRecord()
    } catch (error) {
      set({ status: 'idle', error: toMessage(error) })
    }
  },
  stop: async () => {
    if (get().status !== 'recording') {
      return
    }

    set({ status: 'processing' })

    try {
      await stopRecord()
    } catch (error) {
      set({ status: 'idle', error: toMessage(error) })
    }
  },
  clearError: () => {
    set({ error: null })
  },
}))
