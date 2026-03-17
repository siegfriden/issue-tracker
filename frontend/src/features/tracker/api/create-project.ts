import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { CreateProjectInput } from '../schemas/project'
import type { Project } from '../types'
import { projectsQueryKey } from './query-keys'

async function createProject(data: CreateProjectInput): Promise<Project> {
  return api.post('/api/projects', data)
}

export function useCreateProject({
  mutationConfig,
}: {
  mutationConfig?: MutationConfig<typeof createProject>
} = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: projectsQueryKey })
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: createProject,
  })
}
