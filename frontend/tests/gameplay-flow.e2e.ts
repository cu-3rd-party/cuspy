import { expect, test } from '@playwright/test';

test('approved user can submit kill report and admin can moderate it', async ({ browser }) => {
	const clientContext = await browser.newContext();
	const adminContext = await browser.newContext();
	const clientPage = await clientContext.newPage();
	const adminPage = await adminContext.newPage();

	await clientContext.addInitScript(() => {
		window.localStorage.setItem('backend-access-token', 'approved-token');
	});

	await clientContext.addCookies([{ name: 'backend-access-token', value: 'approved-token', url: 'http://127.0.0.1:4173' }]);
	await adminContext.addCookies([{ name: 'backend-access-token', value: 'admin-token', url: 'http://127.0.0.1:4173' }]);

	await clientPage.goto('http://127.0.0.1:4173/report-kill');
	await expect(clientPage.getByLabel('TARGET_IDENTIFIER')).toBeVisible();
	await clientPage.getByLabel('TARGET_IDENTIFIER').selectOption({ label: 'RAZOR_WIND_ENFORCER' });
	await clientPage.getByLabel('MODUS_OPERANDI').fill('Shadowed target through Sector 7 and confirmed clean elimination.');
	await clientPage.getByRole('button', { name: 'FINALIZE_REPORT' }).click();
	await expect(clientPage.getByText('Kill report submitted for RAZOR_WIND_ENFORCER.')).toBeVisible();

	await adminPage.goto('http://127.0.0.1:4173/admin/events');
	await expect(adminPage.getByRole('button', { name: /RAZOR_WIND_ENFORCER/ })).toBeVisible();
	await adminPage.getByRole('button', { name: /RAZOR_WIND_ENFORCER/ }).click();
	await adminPage.getByLabel('Reviewer Note').fill('Trajectory and timing match target telemetry.');
	await adminPage.getByRole('button', { name: 'CONFIRM_KILL' }).click();
	await expect(adminPage.getByRole('button', { name: /RAZOR_WIND_ENFORCER/ })).toContainText('confirmed');

	await clientPage.goto('http://127.0.0.1:4173/rankings');
	await expect(clientPage.getByText('Approved Agent')).toBeVisible();
	await expect(clientPage.getByText('2', { exact: true }).first()).toBeVisible();

	await clientContext.close();
	await adminContext.close();
});
