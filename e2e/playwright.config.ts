import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  outputDir: './test-results',
  workers: 1,
  reporter: process.env.CI ? [['html', { outputFolder: './playwright-report' }]] : 'list',
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
  },
  webServer: [
    {
      command: 'cargo run --no-default-features --features sqlite,test-helpers --bin todo-api',
      port: 3001,
      cwd: '..',
      reuseExistingServer: !process.env.CI,
    },
    {
      command: 'npm run dev',
      port: 5173,
      cwd: '../web',
      reuseExistingServer: !process.env.CI,
    },
  ],
});
