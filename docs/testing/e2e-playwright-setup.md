# Playwright E2E Setup

## Prerequisites

- Backend running on `http://localhost:3000`
- Frontend running on `http://localhost:5173`
- Node.js 18+
- Empty database

## Install

```bash
cd apps/web
npm install -D @playwright/test
npx playwright install chromium
```

## Configuration

Create `apps/web/playwright.config.ts`:

```typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30000,
  retries: 1,
  use: {
    baseURL: 'http://localhost:5173',
    extraHTTPHeaders: {
      'Content-Type': 'application/json',
    },
  },
  webServer: [
    {
      command: 'cd apps/server && cargo run',
      port: 3000,
      reuseExistingServer: true,
    },
    {
      command: 'cd apps/web && npm run dev',
      port: 5173,
      reuseExistingServer: true,
    },
  ],
});
```

## Running Tests

```bash
cd apps/web
npx playwright test
```

For headed mode (see the browser):

```bash
npx playwright test --headed
```

For UI mode:

```bash
npx playwright test --ui
```
