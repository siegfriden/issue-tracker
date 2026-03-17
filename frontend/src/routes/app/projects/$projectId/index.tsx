import { createFileRoute } from '@tanstack/react-router'

import { Head } from '@/components/head'
import { IssuesTable } from '@/features/tracker/components/issues-table'
import { ProjectDetailLayout } from '@/features/tracker/components/project-detail-layout'

export const Route = createFileRoute('/app/projects/$projectId/')({
  component: ProjectIssuesPage,
})

function ProjectIssuesPage() {
  const { projectId } = Route.useParams()

  return (
    <>
      <Head title="Issues" />
      <ProjectDetailLayout projectId={projectId} activeTab="">
        <IssuesTable projectId={projectId} />
      </ProjectDetailLayout>
    </>
  )
}
