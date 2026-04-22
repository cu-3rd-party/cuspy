import type { PageServerLoad } from './$types';
import { requireGameplayFlow } from '$lib/server/flow-guards';

export const load: PageServerLoad = async ({ parent }) => {
	const { sessionFlow, sessionUser } = await parent();

	return {
		sessionFlow: requireGameplayFlow(sessionFlow),
		sessionUser
	};
};
