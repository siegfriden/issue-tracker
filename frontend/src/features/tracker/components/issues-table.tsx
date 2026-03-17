import { useNavigate } from '@tanstack/react-router'
import { ChevronLeft, ChevronRight, Plus } from 'lucide-react'
import { useState } from 'react'

import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

import { useIssues } from '../api/get-issues'
import type { IssueFilterParams, IssuePriority, IssueStatus } from '../types'
import { CreateIssueDialog } from './create-issue-dialog'
import { priorityColor, statusLabel } from './utils'

export function IssuesTable({ projectId }: { projectId: string }) {
  const navigate = useNavigate()
  const [page, setPage] = useState(1)
  const [filters, setFilters] = useState<IssueFilterParams>({})
  const limit = 20

  const { data, isLoading } = useIssues({
    projectId,
    params: { page, limit, ...filters },
  })

  const issues = data?.data ?? []
  const total = data?.total ?? 0
  const totalPages = Math.ceil(total / limit)
  const showFrom = total > 0 ? (page - 1) * limit + 1 : 0
  const showTo = Math.min(page * limit, total)

  return (
    <div className="flex flex-col gap-4">
      {/* Filter bar */}
      <div className="flex items-center justify-between">
        <div className="flex gap-2">
          <select
            className="border-input bg-background h-9 rounded-lg border px-3 text-[13px]"
            value={filters.status ?? ''}
            onChange={(e) =>
              setFilters((f) => ({
                ...f,
                status: (e.target.value || undefined) as
                  | IssueStatus
                  | undefined,
              }))
            }
          >
            <option value="">Status</option>
            <option value="open">Open</option>
            <option value="in_progress">In Progress</option>
            <option value="resolved">Resolved</option>
            <option value="closed">Closed</option>
            <option value="feedback">Feedback</option>
          </select>

          <select
            className="border-input bg-background h-9 rounded-lg border px-3 text-[13px]"
            value={filters.priority ?? ''}
            onChange={(e) =>
              setFilters((f) => ({
                ...f,
                priority: (e.target.value || undefined) as
                  | IssuePriority
                  | undefined,
              }))
            }
          >
            <option value="">Priority</option>
            <option value="low">Low</option>
            <option value="normal">Normal</option>
            <option value="high">High</option>
            <option value="urgent">Urgent</option>
            <option value="immediate">Immediate</option>
          </select>
        </div>

        <CreateIssueDialog projectId={projectId}>
          <Button>
            <Plus className="size-3.5" data-icon="inline-start" />
            New Issue
          </Button>
        </CreateIssueDialog>
      </div>

      {/* Table */}
      <div className="overflow-hidden rounded-xl border bg-white">
        <Table>
          <TableHeader>
            <TableRow className="bg-muted/50">
              <TableHead className="w-full">Subject</TableHead>
              <TableHead className="w-[100px]">Status</TableHead>
              <TableHead className="w-[100px]">Priority</TableHead>
              <TableHead className="w-[100px]">Type</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading &&
              Array.from({ length: 5 }).map((_, i) => (
                <TableRow key={i}>
                  <TableCell colSpan={4}>
                    <div className="bg-muted h-4 w-full animate-pulse rounded" />
                  </TableCell>
                </TableRow>
              ))}

            {!isLoading && issues.length === 0 && (
              <TableRow>
                <TableCell
                  colSpan={4}
                  className="text-muted-foreground py-8 text-center"
                >
                  No issues found
                </TableCell>
              </TableRow>
            )}

            {issues.map((issue) => (
              <TableRow
                key={issue.id}
                className="cursor-pointer"
                onClick={() =>
                  navigate({
                    to: '/app/projects/$projectId/issues/$issueId',
                    params: { projectId, issueId: issue.id },
                  })
                }
              >
                <TableCell className="text-foreground text-[13px] font-medium">
                  {issue.subject}
                </TableCell>
                <TableCell>
                  <span className="bg-secondary text-secondary-foreground rounded-full px-2 py-0.5 text-[11px] font-medium">
                    {statusLabel(issue.status)}
                  </span>
                </TableCell>
                <TableCell>
                  <span
                    className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${priorityColor(issue.priority)}`}
                  >
                    {issue.priority.charAt(0).toUpperCase() +
                      issue.priority.slice(1)}
                  </span>
                </TableCell>
                <TableCell className="text-muted-foreground text-[13px] capitalize">
                  {issue.issue_type}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      {total > limit && (
        <div className="flex items-center justify-between">
          <span className="text-muted-foreground text-[13px]">
            Showing {showFrom}-{showTo} of {total} issues
          </span>
          <div className="flex gap-1">
            <Button
              variant="outline"
              size="sm"
              disabled={page <= 1}
              onClick={() => setPage((p) => p - 1)}
            >
              <ChevronLeft className="size-3.5" data-icon="inline-start" />
              Previous
            </Button>
            <Button
              variant="outline"
              size="sm"
              disabled={page >= totalPages}
              onClick={() => setPage((p) => p + 1)}
            >
              Next
              <ChevronRight className="size-3.5" data-icon="inline-end" />
            </Button>
          </div>
        </div>
      )}
    </div>
  )
}
