import { createFileRoute } from '@tanstack/react-router'

import { Head } from '@/components/head'
import { MembersTable } from '@/features/tracker/components/members-table'
import { ProjectDetailLayout } from '@/features/tracker/components/project-detail-layout'

export const Route = createFileRoute('/app/projects/$projectId/members')({
  component: ProjectMembersPage,
})

function ProjectMembersPage() {
  const { projectId } = Route.useParams()

  return (
    <>
      <Head title="Members" />
      <ProjectDetailLayout projectId={projectId} activeTab="/members">
        <MembersTable projectId={projectId} />
      </ProjectDetailLayout>
    </>
  )
}
