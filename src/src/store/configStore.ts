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
    model: 'whisper-tiny',
    modelUrl: 'https://huggingface.co/openai/whisper-tiny/resolve/main/model.safetensors',
    modelSha256: '7ebd0e69e78190ffe1438491fa05cc1f5c1aa3a4c4db3bc1723adbb551ea2395',
    modelFilename: 'model.safetensors',
    recordHotkey: 'ctrl+shift+space',
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
