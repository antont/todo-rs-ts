import { useState, useEffect } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { todosApi } from '../api'
import { TodoItem } from './TodoItem'
import { TodoFooter } from './TodoFooter'

function filterFromHash(): string {
  const hash = window.location.hash.replace('#/', '')
  return ['all', 'active', 'completed'].includes(hash) ? hash : 'all'
}

export function TodoApp() {
  const [newTitle, setNewTitle] = useState('')
  const [filter, setFilter] = useState(filterFromHash)
  const queryClient = useQueryClient()

  useEffect(() => {
    const onHashChange = () => setFilter(filterFromHash())
    window.addEventListener('hashchange', onHashChange)
    return () => window.removeEventListener('hashchange', onHashChange)
  }, [])

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['todos', filter],
    queryFn: () => todosApi.list(filter),
  })

  const invalidate = () => queryClient.invalidateQueries({ queryKey: ['todos'] })

  const createMutation = useMutation({
    mutationFn: todosApi.create,
    onSuccess: invalidate,
  })

  const updateMutation = useMutation({
    mutationFn: ({ id, ...req }: { id: string; title?: string; completed?: boolean }) =>
      todosApi.update(id, req),
    onSuccess: invalidate,
  })

  const deleteMutation = useMutation({
    mutationFn: todosApi.remove,
    onSuccess: invalidate,
  })

  const toggleAllMutation = useMutation({
    mutationFn: todosApi.toggleAll,
    onSuccess: invalidate,
  })

  const clearCompletedMutation = useMutation({
    mutationFn: todosApi.clearCompleted,
    onSuccess: invalidate,
  })

  const handleNewTodoKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key !== 'Enter') return
    e.preventDefault()
    const title = newTitle.trim()
    if (!title) return
    createMutation.mutate({ title })
    setNewTitle('')
  }

  if (isLoading) return <p>Loading...</p>
  if (isError) return <p>Error: {(error as Error).message}</p>

  const todos = data?.todos ?? []
  const activeCount = data?.activeCount ?? 0
  const completedCount = data?.completedCount ?? 0
  // totalCount uses unfiltered counts (activeCount + completedCount) from the API,
  // so it reflects the total regardless of the current filter.
  const totalCount = activeCount + completedCount

  return (
    <section className="todoapp">
      <header className="header">
        <h1>todos</h1>
        <input
          className="new-todo"
          placeholder="What needs to be done?"
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          onKeyDown={handleNewTodoKeyDown}
          autoFocus
        />
      </header>

      {totalCount > 0 && (
        <section className="main">
          <input
            id="toggle-all"
            className="toggle-all"
            type="checkbox"
            checked={activeCount === 0}
            onChange={() => toggleAllMutation.mutate()}
          />
          <label htmlFor="toggle-all">Mark all as complete</label>
          <ul className="todo-list">
            {todos.map((todo) => (
              <TodoItem
                key={todo.id}
                todo={todo}
                onToggle={(completed) =>
                  updateMutation.mutate({ id: todo.id, completed })
                }
                onUpdate={(title) =>
                  updateMutation.mutate({ id: todo.id, title })
                }
                onDelete={() => deleteMutation.mutate(todo.id)}
              />
            ))}
          </ul>
        </section>
      )}

      {totalCount > 0 && (
        <TodoFooter
          activeCount={activeCount}
          completedCount={completedCount}
          filter={filter}
          onFilterChange={setFilter}
          onClearCompleted={() => clearCompletedMutation.mutate()}
        />
      )}
    </section>
  )
}
