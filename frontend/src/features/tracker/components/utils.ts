import type { IssuePriority, IssueStatus } from '../types'

export function statusLabel(status: IssueStatus): string {
  const map: Record<IssueStatus, string> = {
    open: 'Open',
    in_progress: 'In Progress',
    resolved: 'Resolved',
    closed: 'Closed',
    feedback: 'Feedback',
  }
  return map[status]
}

export function priorityColor(priority: IssuePriority): string {
  const map: Record<IssuePriority, string> = {
    immediate: 'bg-red-500 text-white',
    urgent: 'bg-red-500 text-white',
    high: 'bg-red-500 text-white',
    normal: 'bg-amber-500 text-white',
    low: 'bg-secondary text-secondary-foreground',
  }
  return map[priority]
}

export function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  })
}

export function relativeTime(dateStr: string): string {
  const now = Date.now()
  const then = new Date(dateStr).getTime()
  const diff = now - then
  const minutes = Math.floor(diff / 60000)
  if (minutes < 1) return 'just now'
  if (minutes < 60) return `${minutes}m ago`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}h ago`
  const days = Math.floor(hours / 24)
  return `${days}d ago`
}

export function initials(name: string): string {
  return name
    .split(' ')
    .map((w) => w[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}
