import { test as base, expect, type Page } from '@playwright/test';

const API_BASE = 'http://localhost:3001';

async function clearTodos() {
  await fetch(`${API_BASE}/api/test/cleanup`, { method: 'DELETE' });
}

async function createTodoViaAPI(title: string) {
  const res = await fetch(`${API_BASE}/api/todos`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title }),
  });
  return res.json();
}

async function addTodo(page: Page, title: string) {
  const countBefore = await page.locator('.todo-list li').count();
  await page.locator('.new-todo').fill(title);
  await page.locator('.new-todo').press('Enter');
  // Wait for the new todo to appear in the list before continuing
  await expect(page.locator('.todo-list li')).toHaveCount(countBefore + 1);
}

// The TodoMVC toggle-all label (position:absolute, z-index:1) overlaps individual
// toggle checkboxes. Coordinate-based clicks hit the label instead. Use el.click()
// to dispatch directly on the DOM element, bypassing hit-testing.
async function toggleItem(page: Page, index: number) {
  await page.locator('.todo-list li').nth(index).locator('.toggle').evaluate(
    (el: HTMLElement) => el.click()
  );
}

// The toggle-all checkbox is hidden (1x1px, opacity 0) with a label overlay.
// Click the label instead of the checkbox.
async function toggleAll(page: Page) {
  await page.locator('label[for="toggle-all"]').click();
}

export const test = base.extend<{ clearState: void }>({
  clearState: [async ({}, use) => {
    await clearTodos();
    await use();
  }, { auto: true }],
});

export { expect, clearTodos, createTodoViaAPI, addTodo, toggleItem, toggleAll };
