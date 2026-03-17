import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type { Issue } from '../types'
import { projectsQueryKey } from './query-keys'

async function getIssue(issueId: string): Promise<Issue> {
  return api.get(`/api/issues/${issueId}`)
}

export function useIssue({
  issueId,
  projectId,
  queryConfig,
}: {
  issueId: string
  projectId: string
  queryConfig?: QueryConfig<typeof getIssue>
}) {
  return useQuery({
    queryKey: [...projectsQueryKey, projectId, 'issues', issueId],
    queryFn: () => getIssue(issueId),
    ...queryConfig,
  })
}
