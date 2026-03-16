import * as z from 'zod'

const schema = z.object({
  API_URL: z.string(),
  APP_TITLE: z.string().optional().default('Issue Tracker'),
})

const envVars = Object.fromEntries(
  Object.entries(import.meta.env)
    .filter(([key]) => key.startsWith('VITE_'))
    .map(([key, value]) => [key.replace('VITE_', ''), value]),
)

const parsed = schema.safeParse(envVars)

if (!parsed.success) {
  const fields = parsed.error.flatten().fieldErrors
  throw new Error(
    `Invalid env provided.\nThe following variables are missing or invalid:\n${Object.entries(
      fields,
    )
      .map(([k, v]) => `- ${k}: ${v}`)
      .join('\n')}`,
  )
}

export const env = parsed.data
