import type { PageServerLoad } from './$types';
import { backendJson } from '$lib/server/backend';
import { requireGameplayFlow } from '$lib/server/flow-guards';
import type { RankingEntry } from '$lib/stores/session';

export const load: PageServerLoad = async ({ parent, request, cookies }) => {
	const { sessionFlow, sessionUser } = await parent();
	const flow = requireGameplayFlow(sessionFlow);
	const token = cookies.get('backend-access-token');

	const rankings = token
		? await backendJson<RankingEntry[]>('/rankings', {
				request,
				token
			})
		: [];

	return {
		sessionFlow: flow,
		sessionUser,
		rankings
	};
};
