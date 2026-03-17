import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type {
  PaginatedResponse,
  PaginationParams,
  ProjectMember,
} from '../types'
import { projectsQueryKey } from './query-keys'

async function getMembers({
  projectId,
  params,
}: {
  projectId: string
  params?: PaginationParams
}): Promise<PaginatedResponse<ProjectMember>> {
  return api.get(`/api/projects/${projectId}/members`, { params })
}

export function useMembers({
  projectId,
  params = {},
  queryConfig,
}: {
  projectId: string
  params?: PaginationParams
  queryConfig?: QueryConfig<typeof getMembers>
}) {
  return useQuery({
    queryKey: [...projectsQueryKey, projectId, 'members', params],
    queryFn: () => getMembers({ projectId, params }),
    ...queryConfig,
  })
}
