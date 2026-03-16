import { useQuery } from '@tanstack/react-query'

import { api, getToken } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type { User } from '../types'
import { userQueryKey } from './query-keys'

export async function getUser(): Promise<User | null> {
  if (!getToken()) return null
  return api.get('/api/users/me')
}

export function useUser({
  queryConfig,
}: {
  queryConfig?: QueryConfig<typeof getUser>
} = {}) {
  return useQuery({
    queryKey: userQueryKey,
    queryFn: getUser,
    staleTime: Infinity,
    ...queryConfig,
  })
}
