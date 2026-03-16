import { z } from 'zod'

const roles = ['admin', 'member', 'viewer'] as const

export const addMemberSchema = z.object({
  user_id: z.string().uuid('Must be a valid UUID.'),
  role: z.enum(roles).default('member'),
})

export type AddMemberInput = z.infer<typeof addMemberSchema>
