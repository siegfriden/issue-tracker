import { useMutation, useQueryClient } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import type { MutationConfig } from '@/lib/react-query'

import type { AddMemberInput } from '../schemas/member'
import { projectsQueryKey } from './query-keys'

async function addMember({
  projectId,
  data,
}: {
  projectId: string
  data: AddMemberInput
}): Promise<void> {
  return api.post(`/api/projects/${projectId}/members`, data)
}

export function useAddMember({
  projectId,
  mutationConfig,
}: {
  projectId: string
  mutationConfig?: MutationConfig<typeof addMember>
}) {
  const queryClient = useQueryClient()
  const { onSuccess, ...restConfig } = mutationConfig || {}

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: [...projectsQueryKey, projectId, 'members'],
      })
      onSuccess?.(...args)
    },
    ...restConfig,
    mutationFn: addMember,
  })
}
