import { test, expect, addTodo, toggleItem } from './fixtures';

test.describe('Item Counter', () => {
  test('shows correct count after create', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'One');
    await expect(page.locator('.todo-count')).toContainText('1 item left');

    await addTodo(page, 'Two');
    await expect(page.locator('.todo-count')).toContainText('2 items left');

    await addTodo(page, 'Three');
    await expect(page.locator('.todo-count')).toContainText('3 items left');
  });

  test('singular "item" for count of 1', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Only one');

    await expect(page.locator('.todo-count')).toContainText('1 item left');
    // Verify it does NOT say "items" (plural)
    await expect(page.locator('.todo-count')).not.toContainText('items');
  });

  test('updates on toggle and delete', async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'First');
    await addTodo(page, 'Second');
    await addTodo(page, 'Third');
    await expect(page.locator('.todo-count')).toContainText('3 items left');

    // Toggle one completed — active count drops
    await toggleItem(page, 0);
    await expect(page.locator('.todo-count')).toContainText('2 items left');

    // Delete one — active count drops again
    const secondItem = page.locator('.todo-list li').nth(1);
    await secondItem.hover();
    await secondItem.locator('.destroy').click();
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });
});
