import { defineConfig } from '@playwright/test';

export default defineConfig({
	use: {
		baseURL: 'http://127.0.0.1:4173'
	},
	webServer: [
		{ command: 'node tests/mock-backend.mjs', port: 3001, reuseExistingServer: !process.env.CI },
		{
			command: 'BACKEND_URL=http://127.0.0.1:3001 npm run build && BACKEND_URL=http://127.0.0.1:3001 npm run preview',
			port: 4173,
			reuseExistingServer: !process.env.CI
		}
	],
	testMatch: '**/*.e2e.{ts,js}'
});
