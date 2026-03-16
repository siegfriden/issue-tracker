import { createFileRoute, useNavigate } from '@tanstack/react-router'

import { AuthLayout } from '@/components/layouts/auth-layout'
import { LoginForm } from '@/features/auth/components/login-form'

export const Route = createFileRoute('/auth/login')({
  component: LoginPage,
})

function LoginPage() {
  const navigate = useNavigate()
  const { redirectTo } = Route.useSearch() as { redirectTo?: string }

  return (
    <AuthLayout title="Login">
      <LoginForm
        onSuccess={() => {
          navigate({ to: redirectTo ?? '/app', replace: true })
        }}
      />
    </AuthLayout>
  )
}
