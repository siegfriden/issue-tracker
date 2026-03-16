import { Link, useNavigate } from '@tanstack/react-router'
import { LogOut, User2 } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { useUser } from '@/features/auth/api/get-user'
import { useLogout } from '@/features/auth/api/logout'

export const Navbar = () => {
  const user = useUser()
  const navigate = useNavigate()
  const logout = useLogout({
    mutationConfig: { onSuccess: () => navigate({ to: '/auth/login' }) },
  })

  return (
    <header className="bg-card flex h-12 items-center justify-between border-b px-8">
      <Link
        to={user.data ? '/app' : '/'}
        className="text-foreground text-sm font-semibold no-underline"
      >
        Issue Tracker
      </Link>

      {user.data && (
        <div className="relative">
          <Button
            variant="ghost"
            size="sm"
            className="text-muted-foreground gap-2"
            onClick={() => logout.mutate()}
          >
            <User2 className="size-4" />
            <span className="text-sm">{user.data.display_name}</span>
            <LogOut className="size-3.5" />
          </Button>
        </div>
      )}
    </header>
  )
}
