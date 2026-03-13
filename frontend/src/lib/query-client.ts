import {
  QueryClient,
  type UseMutationOptions,
  type UseQueryOptions,
} from '@tanstack/react-query'

// Shared QueryClient instance with sensible defaults for the app
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Data stays fresh for 30 seconds before a background refetch triggers
      staleTime: 30 * 1000,
      // Only retry failed requests once to avoid hammering a failing API
      retry: 1,
      // Don't refetch when the browser tab regains focus — reduces unnecessary API calls
      refetchOnWindowFocus: false,
    },
  },
})

/* eslint-disable @typescript-eslint/no-explicit-any */

// Utility types for query/mutation hooks. Accept a fetch function and infer
// the options type, omitting keys the hook owns (queryKey/queryFn/mutationFn).
// This lets callers pass overrides like { enabled, onSuccess } without
// re-specifying the fetch logic.

export type QueryConfig<
  QueryFnType extends (...args: any) => Promise<unknown>,
> = Omit<
  UseQueryOptions<
    Awaited<ReturnType<QueryFnType>>,
    Error,
    Awaited<ReturnType<QueryFnType>>,
    any[]
  >,
  'queryKey' | 'queryFn'
>

export type MutationConfig<
  MutationFnType extends (...args: any) => Promise<unknown>,
> = Omit<
  UseMutationOptions<
    Awaited<ReturnType<MutationFnType>>,
    Error,
    Parameters<MutationFnType>[0]
  >,
  'mutationFn'
>

/* eslint-enable @typescript-eslint/no-explicit-any */
