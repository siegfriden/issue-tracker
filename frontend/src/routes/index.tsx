import { createFileRoute } from '@tanstack/react-router'

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { useExampleQuery } from '@/hooks/use-example-query'

// File-based route: this file maps to "/"
export const Route = createFileRoute('/')({
  component: HomePage,
})

function HomePage() {
  const { data, isLoading } = useExampleQuery()

  if (isLoading) {
    return <p className="text-muted-foreground">Loading...</p>
  }

  return (
    <div className="max-w-lg">
      <Card>
        <CardHeader>
          <CardTitle>{data?.title}</CardTitle>
          <CardDescription>v{data?.version}</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground text-sm">{data?.description}</p>
        </CardContent>
      </Card>
    </div>
  )
}
