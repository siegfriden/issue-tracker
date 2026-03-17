import { Link } from '@tanstack/react-router'
import { Pencil } from 'lucide-react'

import { Button } from '@/components/ui/button'

import { useProject } from '../api/get-project'
import { EditProjectDialog } from './edit-project-dialog'

const TABS = [
  { label: 'Issues', path: '' },
  { label: 'Kanban', path: '/kanban' },
  { label: 'Members', path: '/members' },
] as const

export function ProjectDetailLayout({
  projectId,
  activeTab,
  children,
}: {
  projectId: string
  activeTab: '' | '/kanban' | '/members'
  children: React.ReactNode
}) {
  const { data: project, isLoading } = useProject({ projectId })

  if (isLoading || !project) {
    return (
      <div className="flex flex-1 flex-col gap-4 px-8 py-6">
        <div className="bg-muted h-6 w-40 animate-pulse rounded" />
        <div className="bg-muted h-4 w-64 animate-pulse rounded" />
      </div>
    )
  }

  return (
    <div className="flex flex-1 flex-col gap-4 px-8 py-6">
      {/* Page header */}
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-foreground text-lg font-semibold">
            {project.name}
          </h1>
          <p className="text-muted-foreground text-[13px]">
            {project.description || 'No description'}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full border px-2 py-0.5 font-mono text-[11px] font-medium">
            {project.identifier}
          </span>
          <EditProjectDialog project={project}>
            <Button variant="outline" size="icon">
              <Pencil className="size-3.5" />
            </Button>
          </EditProjectDialog>
        </div>
      </div>

      {/* Tab bar */}
      <div className="flex border-b">
        {TABS.map((tab) => (
          <Link
            key={tab.path}
            to={`/app/projects/$projectId${tab.path}`}
            params={{ projectId }}
            className={`px-4 py-2 text-[13px] font-medium no-underline ${
              activeTab === tab.path
                ? 'text-foreground border-primary border-b-2'
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            {tab.label}
          </Link>
        ))}
      </div>

      {/* Tab content */}
      {children}
    </div>
  )
}
