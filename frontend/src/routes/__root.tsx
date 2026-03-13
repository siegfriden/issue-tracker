import { QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { Link, Outlet, createRootRoute } from '@tanstack/react-router'

import { Toaster } from '@/components/ui/sonner'
import { queryClient } from '@/lib/query-client'

// Root route — wraps every page with the app layout and global providers
export const Route = createRootRoute({
  component: RootLayout,
})

function RootLayout() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="bg-background text-foreground min-h-screen font-sans antialiased">
        <header className="border-b">
          <nav className="container mx-auto flex items-center gap-4 px-4 py-3">
            <span className="text-lg font-semibold">Issue Tracker</span>
            <Link
              to="/"
              className="text-muted-foreground hover:text-foreground [&.active]:text-foreground text-sm transition-colors"
            >
              Home
            </Link>
            <Link
              to="/about"
              className="text-muted-foreground hover:text-foreground [&.active]:text-foreground text-sm transition-colors"
            >
              About
            </Link>
          </nav>
        </header>

        <main className="container mx-auto px-4 py-8">
          {/* Outlet renders the matched child route */}
          <Outlet />
        </main>
      </div>

      <Toaster />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}
