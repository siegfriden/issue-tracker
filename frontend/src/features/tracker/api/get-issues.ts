import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type {
  Issue,
  IssueFilterParams,
  PaginatedResponse,
  PaginationParams,
} from '../types'
import { projectsQueryKey } from './query-keys'

async function getIssues({
  projectId,
  params,
}: {
  projectId: string
  params?: PaginationParams & IssueFilterParams
}): Promise<PaginatedResponse<Issue>> {
  return api.get(`/api/projects/${projectId}/issues`, { params })
}

export function useIssues({
  projectId,
  params = {},
  queryConfig,
}: {
  projectId: string
  params?: PaginationParams & IssueFilterParams
  queryConfig?: QueryConfig<typeof getIssues>
}) {
  return useQuery({
    queryKey: [...projectsQueryKey, projectId, 'issues', params],
    queryFn: () => getIssues({ projectId, params }),
    ...queryConfig,
  })
}
