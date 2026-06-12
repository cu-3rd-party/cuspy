import type { LayoutLoad } from './$types';
import { getSessionFlow } from '$lib/shared/api';
import { buildSessionFlow } from '$lib/pages/profile-flow';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
	try {
		const sessionFlow = await getSessionFlow(undefined, fetch);

		return {
			sessionFlow,
			sessionUser: sessionFlow.user
		};
	} catch {
		const guestFlow = buildSessionFlow(null, null);

		return {
			sessionFlow: guestFlow,
			sessionUser: null
		};
	}
};
