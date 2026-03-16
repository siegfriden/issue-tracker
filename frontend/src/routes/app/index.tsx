import { createFileRoute } from '@tanstack/react-router'

import { Head } from '@/components/head'
import { ProjectList } from '@/features/tracker/components/project-list'

export const Route = createFileRoute('/app/')({
  component: ProjectsPage,
})

function ProjectsPage() {
  return (
    <>
      <Head title="Projects" />
      <ProjectList />
    </>
  )
}
