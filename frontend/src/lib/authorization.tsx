type AuthorizationProps = {
  forbiddenFallback?: React.ReactNode
  children: React.ReactNode
  policyCheck: boolean
}

export const Authorization = ({
  policyCheck,
  forbiddenFallback = null,
  children,
}: AuthorizationProps) => {
  return <>{policyCheck ? children : forbiddenFallback}</>
}
