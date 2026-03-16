import { createFileRoute, useNavigate } from '@tanstack/react-router'

import { Navbar } from '@/components/layouts/navbar'
import { Button } from '@/components/ui/button'
import { useUser } from '@/features/auth/api/get-user'

export const Route = createFileRoute('/')({
  component: LandingPage,
})

function LandingPage() {
  const navigate = useNavigate()
  const user = useUser()

  const handleStart = () => {
    if (user.data) {
      navigate({ to: '/app' })
    } else {
      navigate({ to: '/auth/login' })
    }
  }

  return (
    <div className="bg-background flex min-h-screen flex-col">
      <Navbar />
      <div className="flex flex-1 items-center justify-center">
        <div className="text-center">
          <h1 className="text-foreground text-3xl font-bold tracking-tight">
            Issue Tracker
          </h1>
          <p className="text-muted-foreground mt-2">
            Track projects, manage issues, collaborate with your team.
          </p>
          <div className="mt-8">
            <Button onClick={handleStart}>Get started</Button>
          </div>
        </div>
      </div>
    </div>
  )
}
