import { test, expect, addTodo, toggleItem } from './fixtures';

test.describe('Clear Completed', () => {
  test('button hidden when no completed', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Active item');

    await expect(page.locator('.clear-completed')).not.toBeVisible();
  });

  test('button visible when completed exist', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Complete me');
    await toggleItem(page, 0);

    await expect(page.locator('.clear-completed')).toBeVisible();
  });

  test('clicking clears completed todos', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Keep me');
    await addTodo(page, 'Clear me');
    await toggleItem(page, 1);
    await expect(page.locator('.todo-list li').nth(1)).toHaveClass(/completed/);

    await page.locator('.clear-completed').click();

    await expect(page.locator('.todo-list li')).toHaveCount(1);
    await expect(page.locator('.todo-list li label').first()).toHaveText('Keep me');
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });
});
