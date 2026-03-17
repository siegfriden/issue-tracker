import { Link } from '@tanstack/react-router'
import { useMemo } from 'react'

import { useIssues } from '../api/get-issues'
import type { Issue, IssueStatus } from '../types'
import { priorityColor } from './utils'

const COLUMNS: { status: IssueStatus; label: string }[] = [
  { status: 'open', label: 'Open' },
  { status: 'in_progress', label: 'In Progress' },
  { status: 'resolved', label: 'Resolved' },
  { status: 'closed', label: 'Closed' },
]

function KanbanCard({ issue, projectId }: { issue: Issue; projectId: string }) {
  return (
    <Link
      to="/app/projects/$projectId/issues/$issueId"
      params={{ projectId, issueId: issue.id }}
      className="flex flex-col gap-2 rounded-lg border bg-white p-3 no-underline transition-shadow hover:shadow-sm"
    >
      <span className="text-foreground text-[13px] font-medium">
        {issue.subject}
      </span>
      <div className="flex items-center gap-1.5">
        <span
          className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${priorityColor(issue.priority)}`}
        >
          {issue.priority.charAt(0).toUpperCase() + issue.priority.slice(1)}
        </span>
      </div>
    </Link>
  )
}

export function KanbanBoard({ projectId }: { projectId: string }) {
  const { data, isLoading } = useIssues({
    projectId,
    params: { limit: 100 },
  })

  const grouped = useMemo(() => {
    const map: Record<string, Issue[]> = {}
    for (const col of COLUMNS) map[col.status] = []
    const issues = data?.data ?? []
    for (const issue of issues) {
      if (map[issue.status]) map[issue.status].push(issue)
    }
    return map
  }, [data?.data])

  if (isLoading) {
    return (
      <div className="flex flex-1 gap-4">
        {COLUMNS.map((col) => (
          <div
            key={col.status}
            className="bg-muted flex-1 animate-pulse rounded-xl p-3"
          >
            <div className="bg-muted-foreground/20 mb-2 h-4 w-20 rounded" />
          </div>
        ))}
      </div>
    )
  }

  return (
    <div className="flex flex-1 gap-4">
      {COLUMNS.map((col) => {
        const items = grouped[col.status] ?? []
        return (
          <div
            key={col.status}
            className="bg-muted flex flex-1 flex-col gap-2 rounded-xl p-3"
          >
            <div className="flex items-center justify-between">
              <span className="text-foreground text-[13px] font-semibold">
                {col.label}
              </span>
              <span className="text-muted-foreground text-xs">
                {items.length}
              </span>
            </div>
            {items.length === 0 && (
              <p className="text-muted-foreground py-4 text-center text-xs">
                No issues
              </p>
            )}
            {items.map((issue) => (
              <KanbanCard key={issue.id} issue={issue} projectId={projectId} />
            ))}
          </div>
        )
      })}
    </div>
  )
}
