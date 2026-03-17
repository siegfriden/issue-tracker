import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { UpdateProjectInput } from '../schemas/project'
import type { Project } from '../types'
import { projectsQueryKey } from './query-keys'

async function updateProject({
  projectId,
  data,
}: {
  projectId: string
  data: UpdateProjectInput
}): Promise<Project> {
  return api.patch(`/api/projects/${projectId}`, data)
}

export function useUpdateProject({
  mutationConfig,
}: {
  mutationConfig?: MutationConfig<typeof updateProject>
} = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (data, ...rest) => {
      queryClient.invalidateQueries({ queryKey: projectsQueryKey })
      onSuccess?.(data, ...rest)
    },
    ...restConfig,
    mutationFn: updateProject,
  })
}
