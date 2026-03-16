import { Link } from '@tanstack/react-router'
import { CircleDot } from 'lucide-react'

import type { Project } from '../types'

export function ProjectCard({ project }: { project: Project }) {
  return (
    <Link
      to="/app/projects/$projectId"
      params={{ projectId: project.id }}
      className="group flex flex-col rounded-xl border bg-white no-underline transition-shadow hover:shadow-md"
    >
      {/* Header */}
      <div className="flex flex-col gap-1 p-6">
        <h3 className="text-foreground text-[15px] font-semibold">
          {project.name}
        </h3>
        <p className="text-muted-foreground text-[13px]">
          {project.description || 'No description'}
        </p>
      </div>

      {/* Badges */}
      <div className="flex flex-col gap-3 px-6 pb-6">
        <div className="flex items-center gap-2">
          <span className="rounded-full border px-2 py-0.5 text-[11px] font-medium">
            {project.identifier}
          </span>
          <span className="bg-muted rounded-full px-2 py-0.5 text-[11px] font-medium">
            {project.is_public ? 'Public' : 'Private'}
          </span>
        </div>
      </div>

      {/* Footer */}
      <div className="text-muted-foreground flex items-center justify-between border-t px-6 py-4 text-xs">
        <div className="flex items-center gap-1">
          <CircleDot className="size-[13px]" />
          <span>Issues</span>
        </div>
      </div>
    </Link>
  )
}
