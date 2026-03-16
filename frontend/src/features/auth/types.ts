export type User = {
  id: string
  email: string
  display_name: string
  created_at: string
  updated_at: string
}

export type AuthResponse = {
  access_token: string
  refresh_token: string
}
