import { z } from 'zod/v4'

export const loginInputSchema = z.object({
  email: z
    .string()
    .min(1, 'Email is required.')
    .email('Please enter a valid email address.'),
  password: z.string().min(1, 'Password is required.'),
})

export const registerInputSchema = z.object({
  display_name: z.string().min(1, 'Display name is required.'),
  email: z
    .string()
    .min(1, 'Email is required.')
    .email('Please enter a valid email address.'),
  password: z.string().min(8, 'Password must be at least 8 characters long.'),
})

export const updateUserSchema = z.object({
  display_name: z
    .string()
    .min(1, 'Display name is required.')
    .max(100, 'Display name must be between 1 and 100 characters.')
    .optional(),
  new_password: z
    .string()
    .min(8, 'New password must be at least 8 characters long.')
    .optional(),
})

export type LoginInput = z.infer<typeof loginInputSchema>
export type RegisterInput = z.infer<typeof registerInputSchema>
export type UpdateUserInput = z.infer<typeof updateUserSchema>
