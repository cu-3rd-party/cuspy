import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { listKillReports } from '$lib/shared/api';

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
		reports: await listKillReports().catch(() => [])
	};
};
