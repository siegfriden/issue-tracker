import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api, setTokens } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { RegisterInput } from '../schemas'
import type { AuthResponse, User } from '../types'
import { getUser } from './get-user'
import { userQueryKey } from './query-keys'

async function register(data: RegisterInput): Promise<User | null> {
  const response: AuthResponse = await api.post('/api/auth/register', data)
  setTokens(response.access_token, response.refresh_token)
  return getUser()
}

export function useRegister({
  mutationConfig,
}: {
  mutationConfig?: MutationConfig<typeof register>
} = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.setQueryData(userQueryKey, args[0])
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: register,
  })
}
