import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { CreateCommentInput } from '../schemas/comment'
import type { Comment } from '../types'
import { projectsQueryKey } from './query-keys'

async function createComment({
  issueId,
  data,
}: {
  issueId: string
  data: CreateCommentInput
}): Promise<Comment> {
  return api.post(`/api/issues/${issueId}/comments`, data)
}

export function useCreateComment({
  issueId,
  projectId,
  mutationConfig,
}: {
  issueId: string
  projectId: string
  mutationConfig?: MutationConfig<typeof createComment>
}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: [
          ...projectsQueryKey,
          projectId,
          'issues',
          issueId,
          'comments',
        ],
      })
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: createComment,
  })
}
