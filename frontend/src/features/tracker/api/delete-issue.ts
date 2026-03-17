import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import { projectsQueryKey } from './query-keys'

async function deleteIssue({ issueId }: { issueId: string }): Promise<void> {
  return api.delete(`/api/issues/${issueId}`)
}

export function useDeleteIssue({
  projectId,
  mutationConfig,
}: {
  projectId: string
  mutationConfig?: MutationConfig<typeof deleteIssue>
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
    mutationFn: deleteIssue,
  })
}
