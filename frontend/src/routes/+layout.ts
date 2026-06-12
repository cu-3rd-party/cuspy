import type { LayoutLoad } from './$types';
import { getSessionFlow } from '$lib/shared/api';

export const ssr = false;

export const load: LayoutLoad = async () => {
	const sessionFlow = await getSessionFlow();

	return {
		sessionFlow,
		sessionUser: sessionFlow.user
	};
};
