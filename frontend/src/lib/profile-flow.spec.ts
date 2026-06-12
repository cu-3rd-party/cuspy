import { describe, expect, it } from 'vitest';
import { buildSessionFlow, profileFlowTarget } from '$lib/pages/profile-flow';
import type { ProfileRequest, SessionUser } from '$lib/shared/model';

const makeUser = (): SessionUser => ({
	user_id: 'user-1',
	telegram_id: 1,
	rating: 1000,
	agent_name: 'Alpha',
	agent_data_id: null,
	agent_data: {},
	is_admin: false,
	created_at: '1',
	updated_at: null
});

const makeRequest = (status: ProfileRequest['status']): ProfileRequest => ({
	profile_request_id: 'request-1',
	user_id: 'user-1',
	requested_profile_data_id: 'agent-data-1',
	requested_profile_data: {},
	status,
	reviewer_note: null,
	reviewed_at: null,
	created_at: '1',
	updated_at: '1'
});

describe('profile flow', () => {
	it('routes pending users to waiting screen without gameplay access', () => {
		const request = makeRequest('pending');
		const flow = buildSessionFlow(makeUser(), request, [request]);

		expect(flow.status).toBe('pending');
		expect(flow.canPlay).toBe(false);
		expect(profileFlowTarget(flow)).toBe('/waiting-clearance');
	});

	it('grants gameplay access when at least one request is approved', () => {
		const approved = makeRequest('approved');
		const pending = makeRequest('pending');
		const flow = buildSessionFlow(makeUser(), pending, [pending, approved]);

		expect(flow.status).toBe('pending');
		expect(flow.canPlay).toBe(true);
	});

	it('routes rejected users to edit mode without gameplay access', () => {
		const request = makeRequest('rejected');
		const flow = buildSessionFlow(makeUser(), request, [request]);

		expect(flow.needsProfileEdit).toBe(true);
		expect(flow.canPlay).toBe(false);
		expect(profileFlowTarget(flow)).toBe('/agent-id?mode=edit');
	});
});
