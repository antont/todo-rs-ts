import { describe, beforeEach, test, expect } from 'vitest';
import type { Todo, TodoListResponse } from '../src/types/generated';
import { api, apiStatus, clearTodos, createTodo, listTodos } from './helpers';

describe('Todos API', () => {
  beforeEach(async () => {
    await clearTodos();
  });

  test('list returns empty initially', async () => {
    const data = await listTodos();
    expect(data.todos).toEqual([]);
    expect(data.activeCount).toBe(0);
    expect(data.completedCount).toBe(0);
  });

  test('create a todo', async () => {
    const todo = await createTodo('Buy milk');
    expect(todo.title).toBe('Buy milk');
    expect(todo.completed).toBe(false);
    expect(todo.id).toBeTruthy();
    expect(todo.createdAt).toBeTruthy();
    expect(todo.updatedAt).toBeTruthy();
  });

  test('create rejects empty title', async () => {
    const status = await apiStatus('/api/todos', {
      method: 'POST',
      body: JSON.stringify({ title: '   ' }),
    });
    expect(status).toBe(400);
  });

  test('list returns created todos', async () => {
    await createTodo('First');
    await createTodo('Second');
    const data = await listTodos();
    expect(data.todos).toHaveLength(2);
    expect(data.activeCount).toBe(2);
    expect(data.completedCount).toBe(0);
  });

  test('update title', async () => {
    const todo = await createTodo('Original');
    const { body: updated } = await api<Todo>(`/api/todos/${todo.id}`, {
      method: 'PATCH',
      body: JSON.stringify({ title: 'Updated', completed: null }),
    });
    expect(updated.title).toBe('Updated');
    expect(updated.completed).toBe(false);
  });

  test('toggle completed', async () => {
    const todo = await createTodo('Do laundry');
    const { body: updated } = await api<Todo>(`/api/todos/${todo.id}`, {
      method: 'PATCH',
      body: JSON.stringify({ title: null, completed: true }),
    });
    expect(updated.completed).toBe(true);
    expect(updated.title).toBe('Do laundry');
  });

  test('delete a todo', async () => {
    const todo = await createTodo('Temporary');
    const status = await apiStatus(`/api/todos/${todo.id}`, { method: 'DELETE' });
    expect(status).toBe(200);

    const data = await listTodos();
    expect(data.todos).toHaveLength(0);
  });

  test('delete non-existent returns 404', async () => {
    const fakeId = '00000000-0000-0000-0000-000000000000';
    const status = await apiStatus(`/api/todos/${fakeId}`, { method: 'DELETE' });
    expect(status).toBe(404);
  });

  test('filter active and completed', async () => {
    const todo1 = await createTodo('Active task');
    await createTodo('Will complete');
    await api(`/api/todos/${todo1.id}`, {
      method: 'PATCH',
      body: JSON.stringify({ title: null, completed: true }),
    });

    const active = await listTodos('active');
    expect(active.todos).toHaveLength(1);
    expect(active.todos[0].title).toBe('Will complete');

    const completed = await listTodos('completed');
    expect(completed.todos).toHaveLength(1);
    expect(completed.todos[0].title).toBe('Active task');
  });

  test('toggle all', async () => {
    await createTodo('Task A');
    await createTodo('Task B');

    // Toggle all to completed
    await apiStatus('/api/todos/toggle-all', { method: 'POST' });
    let data = await listTodos();
    expect(data.todos.every((t) => t.completed)).toBe(true);
    expect(data.completedCount).toBe(2);

    // Toggle all back to active
    await apiStatus('/api/todos/toggle-all', { method: 'POST' });
    data = await listTodos();
    expect(data.todos.every((t) => !t.completed)).toBe(true);
    expect(data.activeCount).toBe(2);
  });

  test('clear completed', async () => {
    const todo1 = await createTodo('Keep this');
    await createTodo('Remove this');
    await api(`/api/todos/${todo1.id}`, {
      method: 'PATCH',
      body: JSON.stringify({ title: null, completed: true }),
    });

    await apiStatus('/api/todos/completed', { method: 'DELETE' });

    const data = await listTodos();
    expect(data.todos).toHaveLength(1);
    expect(data.todos[0].title).toBe('Remove this');
    expect(data.activeCount).toBe(1);
    expect(data.completedCount).toBe(0);
  });
});
