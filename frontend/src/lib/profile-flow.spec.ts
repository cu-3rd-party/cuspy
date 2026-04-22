import { describe, expect, it } from 'vitest';
import { buildSessionFlow, profileFlowTarget } from '$lib/profile-flow';
import type { ProfileRequest, SessionUser } from '$lib/stores/session';

const makeUser = (): SessionUser => ({
	user_id: 'user-1',
	telegram_id: 1,
	rating: 1000,
	agent_name: 'Alpha',
	agent_data: {},
	is_admin: false,
	created_at: '1',
	updated_at: null
});

const makeRequest = (status: ProfileRequest['status']): ProfileRequest => ({
	profile_creation_request_id: 'request-1',
	user_id: 'user-1',
	requested_profile_data: {},
	status,
	reviewer_note: null,
	reviewed_at: null,
	created_at: '1',
	updated_at: '1'
});

describe('profile flow', () => {
	it('routes pending users to waiting screen and keeps gameplay access', () => {
		const flow = buildSessionFlow(makeUser(), makeRequest('sent'));

		expect(flow.status).toBe('pending');
		expect(flow.canPlay).toBe(true);
		expect(profileFlowTarget(flow)).toBe('/waiting-clearance');
	});

	it('routes rejected users to edit mode', () => {
		const flow = buildSessionFlow(makeUser(), makeRequest('rejected'));

		expect(flow.needsProfileEdit).toBe(true);
		expect(profileFlowTarget(flow)).toBe('/agent-id?mode=edit');
	});
});
