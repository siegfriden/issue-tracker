import { Link } from '@tanstack/react-router'
import { useState } from 'react'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useLogin } from '@/features/auth/api/login'
import { loginInputSchema } from '@/features/auth/schemas'

type LoginFormProps = {
  onSuccess: () => void
}

export const LoginForm = ({ onSuccess }: LoginFormProps) => {
  const login = useLogin({ mutationConfig: { onSuccess } })
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)
    const values = {
      email: formData.get('email') as string,
      password: formData.get('password') as string,
    }

    const result = loginInputSchema.safeParse(values)
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      for (const issue of result.error.issues) {
        const path = issue.path.join('.')
        if (!fieldErrors[path]) {
          fieldErrors[path] = issue.message
        }
      }
      setErrors(fieldErrors)
      return
    }

    setErrors({})
    login.mutate(result.data)
  }

  return (
    <div className="bg-card w-[380px] rounded-lg border shadow-sm">
      <div className="p-6 pb-0">
        <h1 className="text-foreground text-base font-semibold">Login</h1>
        <p className="text-muted-foreground mt-1 text-[13px]">
          Sign in to your account to continue
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-3 p-6">
        <div className="space-y-1.5">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            name="email"
            type="email"
            placeholder="you@example.com"
          />
          {errors.email && (
            <p className="text-destructive text-xs">{errors.email}</p>
          )}
        </div>

        <div className="space-y-1.5">
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            name="password"
            type="password"
            placeholder="********"
          />
          {errors.password && (
            <p className="text-destructive text-xs">{errors.password}</p>
          )}
        </div>

        <Button type="submit" className="w-full" disabled={login.isPending}>
          {login.isPending ? 'Signing in...' : 'Sign in'}
        </Button>
      </form>

      <div className="px-6 pb-6 text-center">
        <p className="text-muted-foreground text-[13px]">
          Don&apos;t have an account?{' '}
          <Link
            to="/auth/register"
            className="text-foreground font-medium hover:underline"
          >
            Register
          </Link>
        </p>
      </div>
    </div>
  )
}
