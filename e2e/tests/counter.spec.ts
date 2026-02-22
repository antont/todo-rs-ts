import { test, expect, addTodo, todoItem, toggleItemByText } from './fixtures';

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
    await toggleItemByText(page, 'First');
    await expect(page.locator('.todo-count')).toContainText('2 items left');

    // Delete an active one — active count drops again
    const target = todoItem(page, 'Second');
    await target.hover();
    await target.locator('.destroy').click();
    await expect(page.locator('.todo-count')).toContainText('1 item left');
  });
});
