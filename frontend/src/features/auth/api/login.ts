import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api, setTokens } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { LoginInput } from '../schemas'
import type { AuthResponse, User } from '../types'
import { getUser } from './get-user'
import { userQueryKey } from './query-keys'

async function login(data: LoginInput): Promise<User | null> {
  const response: AuthResponse = await api.post('/api/auth/login', data)
  setTokens(response.access_token, response.refresh_token)
  return getUser()
}

export function useLogin({
  mutationConfig,
}: {
  mutationConfig?: MutationConfig<typeof login>
} = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.setQueryData(userQueryKey, args[0])
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: login,
  })
}
