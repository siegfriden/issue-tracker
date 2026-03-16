import { useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'

import { useUser } from '@/features/auth/api/get-user'

import { Head } from '../head'
import { Navbar } from './navbar'

type LayoutProps = {
  children: React.ReactNode
  title: string
}

export const AuthLayout = ({ children, title }: LayoutProps) => {
  const user = useUser()
  const navigate = useNavigate()

  useEffect(() => {
    if (user.data) {
      navigate({ to: '/app', replace: true })
    }
  }, [user.data, navigate])

  return (
    <div className="bg-background flex min-h-screen flex-col">
      <Head title={title} />
      <Navbar />
      <div className="flex flex-1 items-center justify-center p-4">
        {children}
      </div>
    </div>
  )
}
