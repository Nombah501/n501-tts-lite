import { create } from 'zustand'

import {
  onModelDownloadError,
  onModelDownloadFinished,
  onModelDownloadProgress,
  onModelDownloadStarted,
} from '../services/tauri'

export type ModelDownloadState = {
  asset: string | null
  downloadedBytes: number
  totalBytes: number | null
  progress: number | null
}

type ModelState = {
  isDownloading: boolean
  modelName: string | null
  error: string | null
  download: ModelDownloadState
  init: () => void
}

let listenersBound = false

const initialDownload: ModelDownloadState = {
  asset: null,
  downloadedBytes: 0,
  totalBytes: null,
  progress: null,
}

export const useModelStore = create<ModelState>((set) => ({
  isDownloading: false,
  modelName: null,
  error: null,
  download: initialDownload,
  init: () => {
    if (listenersBound) {
      return
    }

    listenersBound = true

    onModelDownloadStarted((payload) => {
      set({
        isDownloading: true,
        modelName: payload.model,
        error: null,
        download: initialDownload,
      })
    })

    onModelDownloadProgress((payload) => {
      const totalBytes = payload.totalBytes ?? null
      const progress = totalBytes ? payload.downloadedBytes / totalBytes : null

      set({
        download: {
          asset: payload.asset,
          downloadedBytes: payload.downloadedBytes,
          totalBytes,
          progress,
        },
      })
    })

    onModelDownloadFinished(() => {
      set({
        isDownloading: false,
        download: {
          ...initialDownload,
          progress: 1,
        },
      })
    })

    onModelDownloadError((payload) => {
      set({
        isDownloading: false,
        error: payload.message,
      })
    })
  },
}))
