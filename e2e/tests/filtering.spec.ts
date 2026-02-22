import { test, expect, addTodo, toggleItem } from './fixtures';

test.describe('Filtering', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await addTodo(page, 'Active todo');
    await addTodo(page, 'Completed todo');
    await toggleItem(page, 1);
    await expect(page.locator('.todo-list li').nth(1)).toHaveClass(/completed/);
  });

  test('filters active todos', async ({ page }) => {
    await page.locator('.filters a', { hasText: 'Active' }).click();

    const items = page.locator('.todo-list li');
    await expect(items).toHaveCount(1);
    await expect(items.first().locator('label')).toHaveText('Active todo');
  });

  test('filters completed todos', async ({ page }) => {
    await page.locator('.filters a', { hasText: 'Completed' }).click();

    const items = page.locator('.todo-list li');
    await expect(items).toHaveCount(1);
    await expect(items.first().locator('label')).toHaveText('Completed todo');
  });

  test('"All" shows everything', async ({ page }) => {
    await page.locator('.filters a', { hasText: 'Active' }).click();
    await expect(page.locator('.todo-list li')).toHaveCount(1);

    await page.locator('.filters a', { hasText: 'All' }).click();
    await expect(page.locator('.todo-list li')).toHaveCount(2);
  });

  test('filter links highlight correctly', async ({ page }) => {
    await expect(page.locator('.filters a.selected')).toHaveText('All');

    await page.locator('.filters a', { hasText: 'Active' }).click();
    await expect(page.locator('.filters a.selected')).toHaveText('Active');

    await page.locator('.filters a', { hasText: 'Completed' }).click();
    await expect(page.locator('.filters a.selected')).toHaveText('Completed');
  });

  test('URL hash updates on filter click', async ({ page }) => {
    await page.locator('.filters a', { hasText: 'Active' }).click();
    expect(new URL(page.url()).hash).toBe('#/active');

    await page.locator('.filters a', { hasText: 'Completed' }).click();
    expect(new URL(page.url()).hash).toBe('#/completed');
  });

  test('direct hash navigation works', async ({ page }) => {
    await page.goto('/#/active');

    const items = page.locator('.todo-list li');
    await expect(items).toHaveCount(1);
    await expect(items.first().locator('label')).toHaveText('Active todo');
  });
});
