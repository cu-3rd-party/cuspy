import { expect, test } from '@playwright/test';

test('admin can review and approve profile requests', async ({ page, context }) => {
	await context.addCookies([{ name: 'backend-access-token', value: 'admin-token', url: 'http://127.0.0.1:4173' }]);
	await page.goto('http://127.0.0.1:4173/admin/moderation');

	await expect(page.getByRole('heading', { name: 'PROFILE_CLEARANCE_QUEUE' })).toBeVisible();
	await expect(page.getByRole('button', { name: /PENDING_AGENT/ })).toBeVisible();
	await page.getByRole('button', { name: 'PENDING_AGENT B21-DS-01 sent' }).click();
	await page.getByLabel('Reviewer Note').fill('Looks consistent.');
	await page.getByRole('button', { name: 'APPROVE_PROFILE' }).click();
	await expect(page.getByText('confirmed')).toHaveCount(1);
});
