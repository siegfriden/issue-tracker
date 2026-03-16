import { z } from 'zod'

export const createProjectSchema = z.object({
  name: z.string().min(1, 'Name is required.').max(200),
  identifier: z
    .string()
    .min(1, 'Identifier is required.')
    .max(100)
    .regex(
      /^[a-z0-9]([a-z0-9-]*[a-z0-9])?$/,
      'Identifier may only contain lowercase letters, digits, and hyphens, and must not start or end with a hyphen.',
    ),
  description: z.string().max(10000).default(''),
  is_public: z.boolean().default(true),
})

export const updateProjectSchema = z.object({
  name: z.string().min(1).max(200).optional(),
  description: z.string().max(10000).optional(),
  is_public: z.boolean().optional(),
})

export type CreateProjectInput = z.infer<typeof createProjectSchema>
export type UpdateProjectInput = z.infer<typeof updateProjectSchema>
