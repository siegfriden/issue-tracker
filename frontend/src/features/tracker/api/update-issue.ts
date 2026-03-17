import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { UpdateIssueInput } from '../schemas/issue'
import type { Issue } from '../types'
import { projectsQueryKey } from './query-keys'

async function updateIssue({
  issueId,
  data,
}: {
  issueId: string
  data: UpdateIssueInput
}): Promise<Issue> {
  return api.patch(`/api/issues/${issueId}`, data)
}

export function useUpdateIssue({
  projectId,
  mutationConfig,
}: {
  projectId: string
  mutationConfig?: MutationConfig<typeof updateIssue>
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
    mutationFn: updateIssue,
  })
}
