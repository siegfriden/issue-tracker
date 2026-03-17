import { createFileRoute } from '@tanstack/react-router'

import { Head } from '@/components/head'
import { IssueDetail } from '@/features/tracker/components/issue-detail'

export const Route = createFileRoute(
  '/app/projects/$projectId/issues/$issueId/',
)({
  component: IssueDetailPage,
})

function IssueDetailPage() {
  const { projectId, issueId } = Route.useParams()

  return (
    <>
      <Head title="Issue" />
      <IssueDetail projectId={projectId} issueId={issueId} />
    </>
  )
}
