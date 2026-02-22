import type { TodoListResponse, CreateTodoRequest } from './types/generated';

const BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:3001';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...((init?.headers as Record<string, string>) ?? {}),
    },
  });
  if (!res.ok) {
    const body = await res.json();
    throw new Error(body.error || res.statusText);
  }
  return res.json();
}

export const todosApi = {
  list: (filter?: string): Promise<TodoListResponse> => {
    const params = filter && filter !== 'all' ? `?filter=${filter}` : '';
    return request(`/api/todos${params}`);
  },

  create: (req: CreateTodoRequest): Promise<void> =>
    request('/api/todos', {
      method: 'POST',
      body: JSON.stringify(req),
    }),

  update: (id: string, req: { title?: string; completed?: boolean }): Promise<void> =>
    request(`/api/todos/${id}`, {
      method: 'PATCH',
      body: JSON.stringify({
        title: req.title ?? null,
        completed: req.completed ?? null,
      }),
    }),

  remove: (id: string): Promise<void> =>
    request(`/api/todos/${id}`, { method: 'DELETE' }),

  toggleAll: (): Promise<void> =>
    request('/api/todos/toggle-all', { method: 'POST' }),

  clearCompleted: (): Promise<void> =>
    request('/api/todos/completed', { method: 'DELETE' }),
};
