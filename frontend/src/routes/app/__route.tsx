import { Outlet, createFileRoute, redirect } from '@tanstack/react-router'

import { DashboardLayout } from '@/components/layouts/dashboard-layout'
import { getUser } from '@/features/auth/api/get-user'
import { userQueryKey } from '@/features/auth/api/query-keys'

export const Route = createFileRoute('/app')({
  beforeLoad: async ({ context }) => {
    const user = await context.queryClient.ensureQueryData({
      queryKey: userQueryKey,
      queryFn: getUser,
      staleTime: Infinity,
    })
    if (!user) {
      throw redirect({
        to: '/auth/login',
        search: { redirectTo: window.location.pathname },
      })
    }
  },
  component: AppLayout,
})

function AppLayout() {
  return (
    <DashboardLayout>
      <Outlet />
    </DashboardLayout>
  )
}
