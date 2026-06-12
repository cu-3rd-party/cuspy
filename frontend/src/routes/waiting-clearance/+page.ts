import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { profileFlowTarget } from '$lib/pages/profile-flow';

export const load: PageLoad = async ({ parent }) => {
	const { sessionFlow, sessionUser } = await parent();

	if (sessionFlow && sessionFlow.status !== 'pending') {
		redirect(307, profileFlowTarget(sessionFlow));
	}

	return { sessionFlow, sessionUser };
};
