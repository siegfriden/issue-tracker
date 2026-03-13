import { createFileRoute } from '@tanstack/react-router'

// File-based route: this file maps to "/about"
export const Route = createFileRoute('/about')({
  component: AboutPage,
})

function AboutPage() {
  return (
    <div className="max-w-lg space-y-4">
      <h1 className="text-2xl font-bold">About</h1>
      <p className="text-muted-foreground">
        Issue Tracker is a project management tool for tracking issues, tasks,
        and team progress. Built with React, TanStack Router, TanStack Query,
        and shadcn/ui.
      </p>
    </div>
  )
}
