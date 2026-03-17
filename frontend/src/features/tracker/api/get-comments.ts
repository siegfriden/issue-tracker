import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type { Comment, PaginatedResponse, PaginationParams } from '../types'
import { projectsQueryKey } from './query-keys'

async function getComments({
  issueId,
  params,
}: {
  issueId: string
  params?: PaginationParams
}): Promise<PaginatedResponse<Comment>> {
  return api.get(`/api/issues/${issueId}/comments`, { params })
}

export function useComments({
  issueId,
  projectId,
  params = {},
  queryConfig,
}: {
  issueId: string
  projectId: string
  params?: PaginationParams
  queryConfig?: QueryConfig<typeof getComments>
}) {
  return useQuery({
    queryKey: [
      ...projectsQueryKey,
      projectId,
      'issues',
      issueId,
      'comments',
      params,
    ],
    queryFn: () => getComments({ issueId, params }),
    ...queryConfig,
  })
}
