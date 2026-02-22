import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { TodoApp } from './components/TodoApp'

const queryClient = new QueryClient()

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <TodoApp />
    </QueryClientProvider>
  )
}
