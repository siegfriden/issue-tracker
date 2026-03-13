import { useQuery } from '@tanstack/react-query'

// Example demonstrating a basic TanStack Query hook pattern.
// Replace the fetcher with a real API call when ready.
export function useExampleQuery() {
  return useQuery({
    // Unique cache key — queries with the same key share cached data
    queryKey: ['example'],
    queryFn: async () => {
      // Simulate an API call
      await new Promise((resolve) => setTimeout(resolve, 500))
      return {
        title: 'Issue Tracker',
        description: 'A project management tool for tracking issues and tasks.',
        version: '0.1.0',
      }
    },
  })
}
