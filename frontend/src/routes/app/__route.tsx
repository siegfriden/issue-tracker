import { Outlet, createFileRoute, redirect } from '@tanstack/react-router'

import { DashboardLayout } from '@/components/layouts/dashboard-layout'
import { getToken } from '@/lib/api-client'

export const Route = createFileRoute('/app')({
  beforeLoad: () => {
    if (!getToken()) {
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
