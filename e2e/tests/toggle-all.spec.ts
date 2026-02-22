import { test, expect, addTodo, toggleItemByText, toggleAll } from './fixtures';

test.describe('Toggle All', () => {
  test('toggle-all marks all completed', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'One');
    await addTodo(page, 'Two');
    await addTodo(page, 'Three');

    await toggleAll(page);

    const items = page.locator('.todo-list li');
    const count = await items.count();
    for (let i = 0; i < count; i++) {
      await expect(items.nth(i)).toHaveClass(/completed/);
      await expect(items.nth(i).locator('.toggle')).toBeChecked();
    }
  });

  test('toggle-all marks all active', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'One');
    await addTodo(page, 'Two');

    // Complete all first
    await toggleAll(page);
    await expect(page.locator('.todo-list li').first()).toHaveClass(/completed/);

    // Toggle back to active
    await toggleAll(page);

    const items = page.locator('.todo-list li');
    const count = await items.count();
    for (let i = 0; i < count; i++) {
      await expect(items.nth(i)).not.toHaveClass(/completed/);
      await expect(items.nth(i).locator('.toggle')).not.toBeChecked();
    }
  });

  test('toggle-all reflects mixed state', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'One');
    await addTodo(page, 'Two');

    // Complete only one
    await toggleItemByText(page, 'One');

    // toggle-all should be unchecked because some are still active
    await expect(page.locator('.toggle-all')).not.toBeChecked();
  });
});
