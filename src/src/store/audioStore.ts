import { create } from 'zustand'

import {
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
}))

const toMessage = (error: unknown) => {
  if (error instanceof Error) {
    return error.message
  }

  return 'Неизвестная ошибка'
}
