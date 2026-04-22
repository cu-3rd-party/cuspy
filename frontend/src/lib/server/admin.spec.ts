import { describe, expect, it } from 'vitest';
import { requireAdminFlow } from '$lib/server/admin';

describe('requireAdminFlow', () => {
	it('returns flow for admins', () => {
		const flow = requireAdminFlow({
			status: 'approved',
			user: { is_admin: true },
			latestProfileRequest: null,
			canPlay: true,
			needsRegistration: false,
			needsProfileEdit: false
		} as never);

		expect(flow.user?.is_admin).toBe(true);
	});

	it('throws for non-admins', () => {
		try {
			requireAdminFlow({
				status: 'approved',
				user: { is_admin: false },
				latestProfileRequest: null,
				canPlay: true,
				needsRegistration: false,
				needsProfileEdit: false
			} as never);
			throw new Error('expected admin guard to throw');
		} catch (error) {
			expect(error).toMatchObject({ status: 403 });
		}
	});
});
