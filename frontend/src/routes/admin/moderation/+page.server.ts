import type { Actions, PageServerLoad } from './$types';
import { fail } from '@sveltejs/kit';
import { backendJson } from '$lib/server/backend';
import { requireAdminFlow } from '$lib/server/admin';
import type { ProfileRequest } from '$lib/stores/session';

export const load: PageServerLoad = async ({ parent, request, cookies }) => {
	const { sessionFlow, sessionUser } = await parent();
	const flow = requireAdminFlow(sessionFlow);
	const token = cookies.get('backend-access-token');

	const requests = token
		? await backendJson<ProfileRequest[]>('/admin/profile-creation-requests', {
				request,
				token
			})
		: [];

	return {
		sessionFlow: flow,
		requests,
		sessionUser
	};
};

export const actions: Actions = {
	moderate: async ({ request, cookies }) => {
		const formData = await request.formData();
		const requestId = String(formData.get('requestId') ?? '');
		const decision = String(formData.get('decision') ?? '');
		const reviewerNote = String(formData.get('reviewerNote') ?? '').trim();
		const token = cookies.get('backend-access-token');

		if (!requestId || !token) {
			return fail(400, { error: 'Missing moderation context.' });
		}

		if (decision !== 'confirmed' && decision !== 'rejected') {
			return fail(400, { error: 'Invalid moderation decision.' });
		}

		try {
			await backendJson(`/admin/profile-creation-requests/${requestId}`, {
				method: 'PATCH',
				request,
				token,
				body: JSON.stringify({
					status: decision,
					reviewer_note: reviewerNote || null
				})
			});
		} catch (cause) {
			return fail(400, {
				error: cause instanceof Error ? cause.message : 'Moderation request failed.'
			});
		}

		return { success: true, requestId, decision };
	}
};
