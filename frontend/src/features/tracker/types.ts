export type Project = {
  id: string
  name: string
  identifier: string
  description: string
  is_public: boolean
  owner_id: string
  created_at: string
  updated_at: string
}

export type Member = {
  id: string
  email: string
  display_name: string
  created_at: string
  updated_at: string
}

export type MemberRole = 'admin' | 'member' | 'viewer'

export type ProjectMember = {
  project_id: string
  user_id: string
  role: MemberRole
  joined_at: string
  user?: Member
}

export type IssueType = 'bug' | 'feature' | 'task' | 'support'
export type IssueStatus =
  | 'open'
  | 'in_progress'
  | 'resolved'
  | 'closed'
  | 'feedback'
export type IssuePriority = 'low' | 'normal' | 'high' | 'urgent' | 'immediate'

export type Issue = {
  id: string
  project_id: string
  author_id: string
  assignee_id: string | null
  subject: string
  description: string
  issue_type: IssueType
  status: IssueStatus
  priority: IssuePriority
  due_date: string | null
  created_at: string
  updated_at: string
}

export type Comment = {
  id: string
  issue_id: string
  author_id: string
  body: string
  created_at: string
  updated_at: string
  author?: Member
}

export type PaginatedResponse<T> = {
  data: T[]
  total: number
  page: number
  limit: number
}

export type PaginationParams = {
  page?: number
  limit?: number
}

export type IssueFilterParams = {
  status?: IssueStatus
  priority?: IssuePriority
  assignee_id?: string
}
