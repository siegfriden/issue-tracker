import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import { userQueryKey } from './query-keys'

async function logout(): Promise<void> {
  await api.post('/api/auth/logout')
}

export function useLogout({
  mutationConfig,
}: {
  mutationConfig?: MutationConfig<typeof logout>
} = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.setQueryData(userQueryKey, null)
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: logout,
  })
}
