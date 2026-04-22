import { describe, expect, it, vi } from 'vitest';

vi.mock('$lib/server/backend', () => ({
	backendJson: vi.fn(async () => [
		{ rank: 1, user_id: 'user-1', agent_name: 'Alpha', rating: 1200, approved_kills: 2, approved_deaths: 1 }
	])
}));

describe('rankings page server load', () => {
	it('loads rankings for gameplay-ready users', async () => {
		const { load } = await import('../../routes/rankings/+page.server');
		const result = (await load({
			parent: async () => ({
				sessionFlow: {
					status: 'pending',
					user: { user_id: 'user-1' },
					latestProfileRequest: null,
					canPlay: true,
					needsRegistration: false,
					needsProfileEdit: false
				},
				sessionUser: null
			}),
			request: new Request('http://localhost/rankings'),
			cookies: { get: () => 'token' }
		} as never)) as unknown as { rankings: Array<unknown>; sessionFlow: { status: string } };

		expect(result.rankings).toHaveLength(1);
		expect(result.sessionFlow.status).toBe('pending');
	});
});
