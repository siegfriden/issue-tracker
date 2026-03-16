import { Link } from '@tanstack/react-router'
import { useState } from 'react'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useRegister } from '@/features/auth/api/register'
import { registerInputSchema } from '@/features/auth/schemas'

type RegisterFormProps = {
  onSuccess: () => void
}

export const RegisterForm = ({ onSuccess }: RegisterFormProps) => {
  const register = useRegister({ mutationConfig: { onSuccess } })
  const [errors, setErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)
    const values = {
      display_name: formData.get('display_name') as string,
      email: formData.get('email') as string,
      password: formData.get('password') as string,
    }

    const result = registerInputSchema.safeParse(values)
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
    register.mutate(result.data)
  }

  return (
    <div className="bg-card w-[380px] rounded-lg border shadow-sm">
      <div className="p-6 pb-0">
        <h1 className="text-foreground text-base font-semibold">
          Create Account
        </h1>
        <p className="text-muted-foreground mt-1 text-[13px]">
          Register for a new account to get started
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-3 p-6">
        <div className="space-y-1.5">
          <Label htmlFor="display_name">Display Name</Label>
          <Input
            id="display_name"
            name="display_name"
            type="text"
            placeholder="John Doe"
          />
          {errors.display_name && (
            <p className="text-destructive text-xs">{errors.display_name}</p>
          )}
        </div>

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
            placeholder="Min. 8 characters"
          />
          {errors.password && (
            <p className="text-destructive text-xs">{errors.password}</p>
          )}
        </div>

        <Button type="submit" className="w-full" disabled={register.isPending}>
          {register.isPending ? 'Creating account...' : 'Create account'}
        </Button>
      </form>

      <div className="px-6 pb-6 text-center">
        <p className="text-muted-foreground text-[13px]">
          Already have an account?{' '}
          <Link
            to="/auth/login"
            className="text-foreground font-medium hover:underline"
          >
            Sign in
          </Link>
        </p>
      </div>
    </div>
  )
}
