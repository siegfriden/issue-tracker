import { createFileRoute, useNavigate } from '@tanstack/react-router'

import { AuthLayout } from '@/components/layouts/auth-layout'
import { RegisterForm } from '@/features/auth/components/register-form'

export const Route = createFileRoute('/auth/register')({
  component: RegisterPage,
})

function RegisterPage() {
  const navigate = useNavigate()
  const { redirectTo } = Route.useSearch() as { redirectTo?: string }

  return (
    <AuthLayout title="Register">
      <RegisterForm
        onSuccess={() => {
          navigate({ to: redirectTo ?? '/app', replace: true })
        }}
      />
    </AuthLayout>
  )
}
