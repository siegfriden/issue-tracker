import { ChevronLeft, ChevronRight, FolderOpen, Plus } from 'lucide-react'
import { useState } from 'react'

import { Button } from '@/components/ui/button'

import { useProjects } from '../api/get-projects'
import { CreateProjectDialog } from './create-project-dialog'
import { ProjectCard } from './project-card'

export function ProjectList() {
  const [page, setPage] = useState(1)
  const limit = 6

  const { data, isLoading } = useProjects({
    params: { page, limit },
  })

  const projects = data?.data ?? []
  const total = data?.total ?? 0
  const totalPages = Math.ceil(total / limit)
  const showFrom = (page - 1) * limit + 1
  const showTo = Math.min(page * limit, total)

  return (
    <div className="flex flex-1 flex-col gap-4 px-8 py-6">
      {/* Page header */}
      <div className="flex items-center justify-between">
        <h1 className="text-foreground text-lg font-semibold">Projects</h1>
        <CreateProjectDialog>
          <Button size="lg">
            <Plus className="size-3.5" data-icon="inline-start" />
            New Project
          </Button>
        </CreateProjectDialog>
      </div>

      {/* Loading skeleton */}
      {isLoading && (
        <div className="grid grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <div
              key={i}
              className="h-[180px] animate-pulse rounded-xl border bg-white"
            />
          ))}
        </div>
      )}

      {/* Empty state */}
      {!isLoading && projects.length === 0 && (
        <div className="flex flex-1 flex-col items-center justify-center py-16 text-center">
          <FolderOpen className="text-muted-foreground/50 size-12" />
          <h3 className="text-foreground mt-4 text-sm font-medium">
            No projects yet
          </h3>
          <p className="text-muted-foreground mt-1 text-sm">
            Create your first project to get started.
          </p>
        </div>
      )}

      {/* Card grid */}
      {!isLoading && projects.length > 0 && (
        <>
          <div className="grid grid-cols-3 gap-4">
            {projects.map((project) => (
              <ProjectCard key={project.id} project={project} />
            ))}
          </div>

          {/* Pagination */}
          {total > limit && (
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground text-[13px]">
                Showing {showFrom}-{showTo} of {total} projects
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
        </>
      )}
    </div>
  )
}
