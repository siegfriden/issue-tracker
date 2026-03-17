import { Link, useNavigate } from '@tanstack/react-router'
import { ArrowLeft, Trash2 } from 'lucide-react'
import { useState } from 'react'

import { Button } from '@/components/ui/button'

import { useCreateComment } from '../api/create-comment'
import { useDeleteIssue } from '../api/delete-issue'
import { useComments } from '../api/get-comments'
import { useIssue } from '../api/get-issue'
import { useUpdateIssue } from '../api/update-issue'
import type { IssuePriority, IssueStatus, IssueType } from '../types'
import { formatDate, initials, relativeTime } from './utils'

export function IssueDetail({
  projectId,
  issueId,
}: {
  projectId: string
  issueId: string
}) {
  const navigate = useNavigate()
  const { data: issue, isLoading } = useIssue({ issueId, projectId })
  const { data: commentsData } = useComments({ issueId, projectId })
  const comments = commentsData?.data ?? []

  const updateIssue = useUpdateIssue({ projectId })
  const deleteIssue = useDeleteIssue({
    projectId,
    mutationConfig: {
      onSuccess: () =>
        navigate({
          to: '/app/projects/$projectId',
          params: { projectId },
        }),
    },
  })
  const createComment = useCreateComment({ issueId, projectId })

  const [commentBody, setCommentBody] = useState('')

  if (isLoading || !issue) {
    return (
      <div className="flex flex-1 flex-col gap-4 px-8 py-6">
        <div className="bg-muted h-6 w-40 animate-pulse rounded" />
        <div className="bg-muted h-8 w-80 animate-pulse rounded" />
      </div>
    )
  }

  const handleFieldChange = (field: string, value: string) => {
    updateIssue.mutate({
      issueId,
      data: { [field]: value || undefined },
    })
  }

  const handleSubmitComment = (e: React.FormEvent) => {
    e.preventDefault()
    if (!commentBody.trim()) return
    createComment.mutate(
      { issueId, data: { body: commentBody } },
      { onSuccess: () => setCommentBody('') },
    )
  }

  return (
    <div className="flex flex-1 flex-col gap-4 px-8 py-6">
      <Link
        to="/app/projects/$projectId"
        params={{ projectId }}
        className="text-foreground flex items-center gap-1.5 text-[13px] font-medium no-underline hover:underline"
      >
        <ArrowLeft className="size-3.5" />
        Back to project
      </Link>

      <h1 className="text-foreground text-lg font-semibold">{issue.subject}</h1>

      <div className="flex flex-1 gap-6">
        {/* Main column */}
        <div className="flex flex-1 flex-col gap-4">
          {/* Description */}
          {issue.description && (
            <div className="rounded-xl border p-4">
              <p className="text-foreground text-[13px] leading-relaxed whitespace-pre-wrap">
                {issue.description}
              </p>
            </div>
          )}

          <hr className="border-border" />

          {/* Comments */}
          <h3 className="text-foreground text-sm font-medium">Comments</h3>

          {comments.map((comment) => (
            <div
              key={comment.id}
              className="bg-muted flex flex-col gap-2 rounded-lg p-3"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="bg-secondary text-muted-foreground flex size-6 items-center justify-center rounded-full text-[10px] font-semibold">
                    {initials(comment.author?.display_name ?? '?')}
                  </div>
                  <span className="text-foreground text-[13px] font-medium">
                    {comment.author?.display_name ?? 'Unknown'}
                  </span>
                </div>
                <span className="text-muted-foreground text-xs">
                  {relativeTime(comment.created_at)}
                </span>
              </div>
              <p className="text-foreground text-[13px] leading-relaxed">
                {comment.body}
              </p>
            </div>
          ))}

          {/* Comment form */}
          <form onSubmit={handleSubmitComment} className="flex flex-col gap-2">
            <textarea
              value={commentBody}
              onChange={(e) => setCommentBody(e.target.value)}
              placeholder="Add a comment..."
              className="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 w-full rounded-lg border bg-white px-3 py-2 text-sm outline-none focus-visible:ring-3"
              rows={3}
            />
            <div className="flex justify-end">
              <Button type="submit" disabled={createComment.isPending}>
                {createComment.isPending ? 'Posting...' : 'Post Comment'}
              </Button>
            </div>
          </form>
        </div>

        {/* Sidebar */}
        <div className="flex w-[250px] flex-col gap-4 rounded-xl border p-4">
          <div className="flex flex-col gap-1">
            <span className="text-muted-foreground text-xs font-medium">
              Status
            </span>
            <select
              className="border-input bg-background h-9 rounded-lg border px-3 text-[13px]"
              value={issue.status}
              onChange={(e) =>
                handleFieldChange('status', e.target.value as IssueStatus)
              }
            >
              <option value="open">Open</option>
              <option value="in_progress">In Progress</option>
              <option value="resolved">Resolved</option>
              <option value="closed">Closed</option>
              <option value="feedback">Feedback</option>
            </select>
          </div>

          <div className="flex flex-col gap-1">
            <span className="text-muted-foreground text-xs font-medium">
              Priority
            </span>
            <select
              className="border-input bg-background h-9 rounded-lg border px-3 text-[13px]"
              value={issue.priority}
              onChange={(e) =>
                handleFieldChange('priority', e.target.value as IssuePriority)
              }
            >
              <option value="low">Low</option>
              <option value="normal">Normal</option>
              <option value="high">High</option>
              <option value="urgent">Urgent</option>
              <option value="immediate">Immediate</option>
            </select>
          </div>

          <div className="flex flex-col gap-1">
            <span className="text-muted-foreground text-xs font-medium">
              Type
            </span>
            <select
              className="border-input bg-background h-9 rounded-lg border px-3 text-[13px]"
              value={issue.issue_type}
              onChange={(e) =>
                handleFieldChange('issue_type', e.target.value as IssueType)
              }
            >
              <option value="bug">Bug</option>
              <option value="feature">Feature</option>
              <option value="task">Task</option>
              <option value="support">Support</option>
            </select>
          </div>

          {issue.due_date && (
            <div className="flex flex-col gap-1">
              <span className="text-muted-foreground text-xs font-medium">
                Due Date
              </span>
              <span className="text-foreground text-[13px]">
                {formatDate(issue.due_date)}
              </span>
            </div>
          )}

          <hr className="border-border" />

          <Button
            variant="outline"
            className="text-destructive hover:text-destructive"
            onClick={() => {
              if (confirm('Delete this issue?')) {
                deleteIssue.mutate({ issueId })
              }
            }}
          >
            <Trash2 className="size-3.5" data-icon="inline-start" />
            Delete Issue
          </Button>
        </div>
      </div>
    </div>
  )
}
