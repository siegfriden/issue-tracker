import { useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { QueryConfig } from '@/lib/react-query'

import type { Project } from '../types'
import { projectsQueryKey } from './query-keys'

async function getProject(projectId: string): Promise<Project> {
  return api.get(`/api/projects/${projectId}`)
}

export function useProject({
  projectId,
  queryConfig,
}: {
  projectId: string
  queryConfig?: QueryConfig<typeof getProject>
}) {
  return useQuery({
    queryKey: [...projectsQueryKey, projectId],
    queryFn: () => getProject(projectId),
    ...queryConfig,
  })
}
