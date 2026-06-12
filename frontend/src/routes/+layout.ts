import type { LayoutLoad } from './$types';
import { getSessionFlow } from '$lib/shared/api';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
	const sessionFlow = await getSessionFlow(undefined, fetch);

	return {
		sessionFlow,
		sessionUser: sessionFlow.user
	};
};
