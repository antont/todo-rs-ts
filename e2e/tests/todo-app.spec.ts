import { test, expect, addTodo, toggleItem } from './fixtures';

test.describe('Todo App — core CRUD & UI', () => {
  test('shows empty state', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.new-todo')).toBeVisible();
    await expect(page.locator('.main')).not.toBeVisible();
    await expect(page.locator('.footer')).not.toBeVisible();
  });

  test('creates a todo', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Buy milk');

    const items = page.locator('.todo-list li');
    await expect(items).toHaveCount(1);
    await expect(items.first().locator('label')).toHaveText('Buy milk');
    await expect(page.locator('.new-todo')).toHaveValue('');
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });

  test('creates multiple todos', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'First');
    await addTodo(page, 'Second');
    await addTodo(page, 'Third');

    const items = page.locator('.todo-list li');
    await expect(items).toHaveCount(3);
    await expect(items.nth(0).locator('label')).toHaveText('First');
    await expect(items.nth(1).locator('label')).toHaveText('Second');
    await expect(items.nth(2).locator('label')).toHaveText('Third');
    await expect(page.locator('.todo-count')).toContainText('3 items left');
  });

  test('trims whitespace on create', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, '  foo  ');

    await expect(page.locator('.todo-list li label').first()).toHaveText('foo');
  });

  test('ignores empty input on Enter', async ({ page }) => {
    await page.goto('/');
    await page.locator('.new-todo').press('Enter');
    await page.locator('.new-todo').fill('   ');
    await page.locator('.new-todo').press('Enter');

    await expect(page.locator('.todo-list li')).toHaveCount(0);
    await expect(page.locator('.main')).not.toBeVisible();
  });

  test('toggles a todo completed', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Toggle me');

    await toggleItem(page, 0);

    const item = page.locator('.todo-list li').first();
    await expect(item).toHaveClass(/completed/);
    await expect(item.locator('.toggle')).toBeChecked();
  });

  test('toggles a todo back to active', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Toggle me');

    await toggleItem(page, 0);
    const item = page.locator('.todo-list li').first();
    await expect(item).toHaveClass(/completed/);

    await toggleItem(page, 0);
    await expect(item).not.toHaveClass(/completed/);
    await expect(item.locator('.toggle')).not.toBeChecked();
  });

  test('deletes a todo', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Delete me');
    await addTodo(page, 'Keep me');

    const firstItem = page.locator('.todo-list li').first();
    await firstItem.hover();
    await firstItem.locator('.destroy').click();

    await expect(page.locator('.todo-list li')).toHaveCount(1);
    await expect(page.locator('.todo-list li label').first()).toHaveText('Keep me');
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });

  test('edits a todo (double-click + Enter)', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Original');

    const item = page.locator('.todo-list li').first();
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');
    await expect(editInput).toBeVisible();

    await editInput.fill('Updated');
    await editInput.press('Enter');

    await expect(item.locator('label')).toHaveText('Updated');
    await expect(editInput).not.toBeVisible();
  });

  test('cancels edit on Escape', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Original');

    const item = page.locator('.todo-list li').first();
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');

    await editInput.fill('Changed');
    await editInput.press('Escape');

    await expect(item.locator('label')).toHaveText('Original');
    await expect(editInput).not.toBeVisible();
  });

  test('saves edit on blur', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Original');

    const item = page.locator('.todo-list li').first();
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');

    await editInput.fill('Blurred');
    await page.locator('.new-todo').click();

    await expect(item.locator('label')).toHaveText('Blurred');
  });

  test('deletes todo if edit cleared', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'To be deleted');

    const item = page.locator('.todo-list li').first();
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');

    await editInput.fill('');
    await editInput.press('Enter');

    await expect(page.locator('.todo-list li')).toHaveCount(0);
  });
});
