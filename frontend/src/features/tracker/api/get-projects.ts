import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type { PaginatedResponse, PaginationParams, Project } from '../types'
import { projectsQueryKey } from './query-keys'

async function getProjects(
  params: PaginationParams = {},
): Promise<PaginatedResponse<Project>> {
  return api.get('/api/projects', { params })
}

export function useProjects({
  params = {},
  queryConfig,
}: {
  params?: PaginationParams
  queryConfig?: QueryConfig<typeof getProjects>
} = {}) {
  return useQuery({
    queryKey: [...projectsQueryKey, params],
    queryFn: () => getProjects(params),
    ...queryConfig,
  })
}
