import { test, expect, addTodo, todoItem, toggleItemByText } from './fixtures';

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
    await expect(todoItem(page, 'First')).toBeVisible();
    await expect(todoItem(page, 'Second')).toBeVisible();
    await expect(todoItem(page, 'Third')).toBeVisible();
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

    await toggleItemByText(page, 'Toggle me');

    const item = todoItem(page, 'Toggle me');
    await expect(item).toHaveClass(/completed/);
    await expect(item.locator('.toggle')).toBeChecked();
  });

  test('toggles a todo back to active', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Toggle me');

    await toggleItemByText(page, 'Toggle me');
    const item = todoItem(page, 'Toggle me');
    await expect(item).toHaveClass(/completed/);

    await toggleItemByText(page, 'Toggle me');
    await expect(item).not.toHaveClass(/completed/);
    await expect(item.locator('.toggle')).not.toBeChecked();
  });

  test('deletes a todo', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Delete me');
    await addTodo(page, 'Keep me');

    const target = todoItem(page, 'Delete me');
    await target.hover();
    await target.locator('.destroy').click();

    await expect(page.locator('.todo-list li')).toHaveCount(1);
    await expect(todoItem(page, 'Keep me')).toBeVisible();
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });

  test('edits a todo (double-click + Enter)', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Original');

    const item = todoItem(page, 'Original');
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');
    await expect(editInput).toBeVisible();

    await editInput.fill('Updated');
    await editInput.press('Enter');

    await expect(todoItem(page, 'Updated')).toBeVisible();
    await expect(editInput).not.toBeVisible();
  });

  test('cancels edit on Escape', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Original');

    const item = todoItem(page, 'Original');
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

    const item = todoItem(page, 'Original');
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');

    await editInput.fill('Blurred');
    await page.locator('.new-todo').click();

    await expect(todoItem(page, 'Blurred')).toBeVisible();
  });

  test('deletes todo if edit cleared', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'To be deleted');

    const item = todoItem(page, 'To be deleted');
    await item.locator('label').dblclick();
    const editInput = item.locator('.edit');

    await editInput.fill('');
    await editInput.press('Enter');

    await expect(page.locator('.todo-list li')).toHaveCount(0);
  });
});
