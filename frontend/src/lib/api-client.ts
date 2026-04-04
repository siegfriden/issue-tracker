import Axios, { type AxiosError, type InternalAxiosRequestConfig } from 'axios'
import { toast } from 'sonner'

import { env } from '@/env'

function redirectToLogin() {
  if (window.location.pathname.startsWith('/auth/')) return
  const redirectTo = window.location.pathname + window.location.search
  window.location.replace(
    `/auth/login?redirect_to=${encodeURIComponent(redirectTo)}`,
  )
}

let isRefreshing = false
let refreshQueue: Array<{
  resolve: () => void
  reject: (err: unknown) => void
}> = []

function processQueue(error: unknown) {
  refreshQueue.forEach(({ resolve, reject }) =>
    error ? reject(error) : resolve(),
  )
  refreshQueue = []
}

export const api = Axios.create({
  baseURL: env.API_URL,
  withCredentials: true,
})

api.interceptors.request.use((config) => {
  config.headers.Accept = 'application/json'
  return config
})

api.interceptors.response.use(
  (response) => response.data,
  async (error: AxiosError) => {
    const original = error.config as InternalAxiosRequestConfig & {
      _retry?: boolean
    }
    const message =
      (error.response?.data as { message?: string })?.message || error.message

    if (error.response?.status !== 401) {
      toast.error(message)
      return Promise.reject(error)
    }

    // Don't retry if this request already retried or if the refresh call itself failed.
    if (original._retry || original.url === '/api/auth/refresh') {
      redirectToLogin()
      return Promise.reject(error)
    }

    // Queue concurrent 401s while a refresh is in flight.
    if (isRefreshing) {
      return new Promise<void>((resolve, reject) => {
        refreshQueue.push({ resolve, reject })
      }).then(() => api(original))
    }

    original._retry = true
    isRefreshing = true

    try {
      await api.post('/api/auth/refresh')
      processQueue(null)
      return api(original)
    } catch (refreshError) {
      processQueue(refreshError)
      redirectToLogin()
      return Promise.reject(refreshError)
    } finally {
      isRefreshing = false
    }
  },
)
