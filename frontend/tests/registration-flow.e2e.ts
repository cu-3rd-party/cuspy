import { expect, test } from '@playwright/test';
import { Buffer } from 'buffer';

test('full registration flow: fill dossier, submit, admin approves, game unlocks', async ({
	page
}) => {
	const uniqueId = Date.now();
	const codename = `TEST_E2E_${uniqueId}`;
	const adminSecret = 'dev-admin-secret';

	// 1. Go to home page
	await page.goto('/');

	// 2. Wait for terminal verification to finish (3s delay in +page.ts)
	await expect(page.getByRole('link', { name: 'Start registration' })).toBeEnabled({
		timeout: 15000
	});

	// 3. Click CTA to start registration
	await page.getByRole('link', { name: 'Start registration' }).click();
	await expect(page.getByPlaceholder('e.g. NEON_FOX')).toBeVisible();

	// 4. Fill Agent ID form
	await page.getByPlaceholder('e.g. NEON_FOX').fill(codename);

	// Select academic level: Bachelor
	await page.getByText('Bachelor').first().click();

	// Select course number 3 (wait for select to appear)
	await expect(page.locator('select')).toBeVisible();
	await page.locator('select').selectOption('3');

	// Select bachelor track: Dev (wait for radios to appear)
	await expect(page.getByText('Dev')).toBeVisible();
	await page.getByText('Dev').click();

	// Upload identification image (minimal 1x1 PNG)
	const imageBuffer = Buffer.from(
		'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==',
		'base64'
	);
	await page.locator('input[type="file"]').setInputFiles({
		name: 'identification.png',
		mimeType: 'image/png',
		buffer: imageBuffer
	});
	await expect(page.getByText('identification.png')).toBeVisible();

	// 5. Click Proceed — this registers the user and navigates to Operational Boundaries
	await page.getByRole('button', { name: 'Proceed' }).click();

	// 6. On Operational Boundaries — click Finalize dossier
	await expect(page.getByText('Finalize dossier')).toBeVisible({ timeout: 10000 });
	await page.getByText('Finalize dossier').click();

	// 7. On Dossier Verification — verify entered codename is displayed
	await expect(page.getByText(codename)).toBeVisible();

	// 8. Submit for admin approval and capture profile_request_id from the network
	const [response] = await Promise.all([
		page.waitForResponse(
			(resp) =>
				resp.url().includes('/profile-requests') &&
				resp.request().method() === 'POST' &&
				resp.status() === 201
		),
		page.getByRole('button', { name: /Submit for admin approval/ }).click()
	]);

	const profileRequest = await response.json();
	expect(profileRequest).toHaveProperty('profile_request_id');
	const requestId = profileRequest.profile_request_id;

	// 9. Wait until the waiting-clearance view renders
	await expect(page.getByText('Dossier in command queue')).toBeVisible({ timeout: 10000 });

	// 10. Approve the request via admin API
	const approveRes = await page.request.fetch(
		`http://127.0.0.1:3000/admin/profile-requests/${requestId}`,
		{
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json',
				'x-admin-secret': adminSecret
			},
			data: {
				status: 'confirmed',
				reviewer_note: 'Approved by E2E test'
			}
		}
	);
	expect(approveRes.ok()).toBe(true);

	// 11. Wait for frontend polling (5s interval) to detect status change and navigate to game view
	await expect(
		page.getByText('Current objective').or(page.getByText('OPERATIVE ACTIVE'))
	).toBeVisible({ timeout: 10000 });
});
