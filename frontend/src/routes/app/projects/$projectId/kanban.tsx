import { createFileRoute } from '@tanstack/react-router'

import { Head } from '@/components/head'
import { KanbanBoard } from '@/features/tracker/components/kanban-board'
import { ProjectDetailLayout } from '@/features/tracker/components/project-detail-layout'

export const Route = createFileRoute('/app/projects/$projectId/kanban')({
  component: ProjectKanbanPage,
})

function ProjectKanbanPage() {
  const { projectId } = Route.useParams()

  return (
    <>
      <Head title="Kanban" />
      <ProjectDetailLayout projectId={projectId} activeTab="/kanban">
        <KanbanBoard projectId={projectId} />
      </ProjectDetailLayout>
    </>
  )
}
