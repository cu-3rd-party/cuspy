import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { profileFlowTarget } from '$lib/pages/profile-flow';

export const load: PageLoad = async ({ parent }) => {
	const { sessionFlow, sessionUser } = await parent();

	if (!sessionFlow?.canPlay) {
		redirect(307, sessionFlow ? profileFlowTarget(sessionFlow) : '/');
	}

	return { sessionFlow, sessionUser };
};
