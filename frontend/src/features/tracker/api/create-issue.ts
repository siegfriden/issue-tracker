import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { CreateIssueInput } from '../schemas/issue'
import type { Issue } from '../types'
import { projectsQueryKey } from './query-keys'

async function createIssue({
  projectId,
  data,
}: {
  projectId: string
  data: CreateIssueInput
}): Promise<Issue> {
  return api.post(`/api/projects/${projectId}/issues`, data)
}

export function useCreateIssue({
  projectId,
  mutationConfig,
}: {
  projectId: string
  mutationConfig?: MutationConfig<typeof createIssue>
}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: [...projectsQueryKey, projectId, 'issues'],
      })
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: createIssue,
  })
}
