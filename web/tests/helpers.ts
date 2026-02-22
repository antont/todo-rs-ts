import type { Todo, TodoListResponse } from '../src/types/generated';

const BASE = process.env.API_URL ?? 'http://localhost:3001';

export async function api<T>(path: string, init?: RequestInit): Promise<{ status: number; body: T }> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...init,
  });
  const body = await res.json() as T;
  return { status: res.status, body };
}

export async function apiStatus(path: string, init?: RequestInit): Promise<number> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { 'Content-Type': 'application/json' },
    ...init,
  });
  return res.status;
}

export async function clearTodos(): Promise<void> {
  await fetch(`${BASE}/api/todos`, { method: 'DELETE' });
}

export async function createTodo(title: string): Promise<Todo> {
  const { body } = await api<Todo>('/api/todos', {
    method: 'POST',
    body: JSON.stringify({ title }),
  });
  return body;
}

export async function listTodos(filter?: string): Promise<TodoListResponse> {
  const params = filter && filter !== 'all' ? `?filter=${filter}` : '';
  const { body } = await api<TodoListResponse>(`/api/todos${params}`);
  return body;
}
