import { expect, test } from '@playwright/test';

test('pending user lands on waiting screen and can open game', async ({ page, context }) => {
	await context.addCookies([{ name: 'backend-access-token', value: 'pending-token', url: 'http://127.0.0.1:4173' }]);
	await page.goto('http://127.0.0.1:4173/');

	await expect(page.getByText('Dossier in command queue')).toBeVisible();
	await page.getByRole('link', { name: 'Continue to game' }).click();
	await expect(page).toHaveURL(/target-intel/);
});

test('approved user lands directly on game', async ({ page, context }) => {
	await context.addCookies([{ name: 'backend-access-token', value: 'approved-token', url: 'http://127.0.0.1:4173' }]);
	await page.goto('http://127.0.0.1:4173/');

	await expect(page).toHaveURL(/target-intel/);
	await expect(page.getByText('Current objective')).toBeVisible();
});
