import { createFileRoute } from '@tanstack/react-router'
import { FolderOpen } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { useUser } from '@/features/auth/api/get-user'

export const Route = createFileRoute('/app/')({
  component: ProjectsPage,
})

function ProjectsPage() {
  const user = useUser()

  return (
    <div className="mx-auto max-w-5xl px-8 py-10">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-foreground text-xl font-semibold">Projects</h1>
          <p className="text-muted-foreground mt-1 text-sm">
            Welcome back, {user.data?.display_name ?? 'User'}
          </p>
        </div>
        <Button>New Project</Button>
      </div>

      <div className="mt-12 flex flex-col items-center justify-center py-16 text-center">
        <FolderOpen className="text-muted-foreground/50 size-12" />
        <h3 className="text-foreground mt-4 text-sm font-medium">
          No projects yet
        </h3>
        <p className="text-muted-foreground mt-1 text-sm">
          Create your first project to get started.
        </p>
      </div>
    </div>
  )
}
