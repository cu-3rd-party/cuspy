import { defineConfig } from '@playwright/test';

export default defineConfig({
	use: {
		baseURL: 'http://localhost:5173'
	},
	webServer: [
		{
			command: 'PUBLIC_BACKEND_URL=http://127.0.0.1:3000 npm run dev',
			port: 5173,
			reuseExistingServer: !process.env.CI
		}
	],
	testMatch: '**/*.e2e.{ts,js}'
});
