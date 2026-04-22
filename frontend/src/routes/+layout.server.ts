import type { LayoutServerLoad } from './$types';
import { loadSessionFlow } from '$lib/server/session-flow';

export const load: LayoutServerLoad = async ({ cookies, request }) => {
	const accessToken = cookies.get('backend-access-token');
	const sessionFlow = await loadSessionFlow({ accessToken: accessToken ?? null, request, cookies });

	return {
		accessToken: accessToken ?? null,
		sessionFlow,
		sessionUser: sessionFlow.user
	};
};
