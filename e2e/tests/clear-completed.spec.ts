import { test, expect, addTodo, todoItem, toggleItemByText } from './fixtures';

test.describe('Clear Completed', () => {
  test('button hidden when no completed', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Active item');

    await expect(page.locator('.clear-completed')).not.toBeVisible();
  });

  test('button visible when completed exist', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Complete me');
    await toggleItemByText(page, 'Complete me');

    await expect(page.locator('.clear-completed')).toBeVisible();
  });

  test('clicking clears completed todos', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Keep me');
    await addTodo(page, 'Clear me');
    await toggleItemByText(page, 'Clear me');
    await expect(todoItem(page, 'Clear me')).toHaveClass(/completed/);

    await page.locator('.clear-completed').click();

    await expect(page.locator('.todo-list li')).toHaveCount(1);
    await expect(todoItem(page, 'Keep me')).toBeVisible();
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });
});
