import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { listAdminProfileRequests } from '$lib/shared/api';

export const load: PageLoad = async ({ parent }) => {
	const { sessionFlow, sessionUser } = await parent();

	if (!sessionFlow?.user) {
		redirect(307, '/');
	}

	if (!sessionFlow.user.is_admin) {
		redirect(307, '/target-intel');
	}

	return {
		sessionFlow,
		sessionUser,
		requests: await listAdminProfileRequests().catch(() => [])
	};
};
