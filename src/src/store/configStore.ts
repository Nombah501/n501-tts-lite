import { create } from 'zustand'

import { getConfig, onConfigUpdated, updateConfig } from '../services/tauri'
import type { AppConfig } from '../services/tauri'

type ConfigState = {
  config: AppConfig
  isLoaded: boolean
  init: () => void
  load: () => Promise<void>
  save: (config: AppConfig) => Promise<void>
}

let listenersBound = false

export const useConfigStore = create<ConfigState>((set) => ({
  config: {
    model: 'tiny',
    modelUrl: '',
    modelSha256: '',
    modelFilename: 'whisper-tiny.bin',
  },
  isLoaded: false,
  init: () => {
    if (listenersBound) {
      return
    }

    listenersBound = true

    onConfigUpdated((payload) => {
      set({ config: payload.config })
    })
  },
  load: async () => {
    const config = await getConfig()
    set({ config, isLoaded: true })
  },
  save: async (config) => {
    await updateConfig(config)
    set({ config })
  },
}))
