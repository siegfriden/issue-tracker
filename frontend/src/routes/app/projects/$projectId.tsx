import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/app/projects/$projectId')({
  component: ProjectDetailsRoute,
})

function ProjectDetailsRoute() {
  const { projectId } = Route.useParams()
  return <div>Project {projectId}</div>
}
