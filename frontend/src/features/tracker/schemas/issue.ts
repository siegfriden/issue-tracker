import { z } from 'zod'

const issueTypes = ['bug', 'feature', 'task', 'support'] as const
const statuses = [
  'open',
  'in_progress',
  'resolved',
  'closed',
  'feedback',
] as const
const priorities = ['low', 'normal', 'high', 'urgent', 'immediate'] as const

export const createIssueSchema = z.object({
  subject: z.string().min(1, 'Subject is required.').max(300),
  description: z.string().max(10000).default(''),
  assignee_id: z.string().uuid().optional(),
  issue_type: z.enum(issueTypes).default('task'),
  status: z.enum(statuses).default('open'),
  priority: z.enum(priorities).default('normal'),
  due_date: z.string().date().optional(),
})

export const updateIssueSchema = z.object({
  subject: z.string().min(1).max(300).optional(),
  description: z.string().max(10000).optional(),
  assignee_id: z.string().uuid().nullable().optional(),
  issue_type: z.enum(issueTypes).optional(),
  status: z.enum(statuses).optional(),
  priority: z.enum(priorities).optional(),
  due_date: z.string().date().nullable().optional(),
})

export type CreateIssueInput = z.infer<typeof createIssueSchema>
export type UpdateIssueInput = z.infer<typeof updateIssueSchema>
