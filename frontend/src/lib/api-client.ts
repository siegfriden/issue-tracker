import Axios, { type InternalAxiosRequestConfig } from 'axios'
import { toast } from 'sonner'

import { env } from '@/env'

const TOKEN_KEY = 'access_token'
const REFRESH_TOKEN_KEY = 'refresh_token'

export const getToken = () => localStorage.getItem(TOKEN_KEY)
export const getRefreshToken = () => localStorage.getItem(REFRESH_TOKEN_KEY)

export const setTokens = (accessToken: string, refreshToken: string) => {
  localStorage.setItem(TOKEN_KEY, accessToken)
  localStorage.setItem(REFRESH_TOKEN_KEY, refreshToken)
}

export const clearTokens = () => {
  localStorage.removeItem(TOKEN_KEY)
  localStorage.removeItem(REFRESH_TOKEN_KEY)
}

function authRequestInterceptor(config: InternalAxiosRequestConfig) {
  if (config.headers) {
    config.headers.Accept = 'application/json'
    const token = getToken()
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
  }
  return config
}

export const api = Axios.create({
  baseURL: env.API_URL,
})

api.interceptors.request.use(authRequestInterceptor)
api.interceptors.response.use(
  (response) => {
    return response.data
  },
  (error) => {
    const message = error.response?.data?.message || error.message

    if (error.response?.status !== 401) {
      toast.error(message)
    }

    if (error.response?.status === 401) {
      clearTokens()
      const searchParams = new URLSearchParams()
      const redirectTo = window.location.pathname + window.location.search
      if (redirectTo && redirectTo !== '/auth/login') {
        searchParams.set('redirect_to', redirectTo)
      }
      const search = searchParams.toString()
      window.location.replace(`/auth/login${search ? `?${search}` : ''}`)
    }

    return Promise.reject(error)
  },
)
