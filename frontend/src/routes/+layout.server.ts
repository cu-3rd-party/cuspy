import type { LayoutServerLoad } from './$types';
import { backendJson } from '$lib/server/backend';
import type { SessionUser } from '$lib/stores/session';

export const load: LayoutServerLoad = async ({ cookies, request }) => {
	const accessToken = cookies.get('backend-access-token');
	let sessionUser: SessionUser | null = null;

	if (accessToken) {
		try {
			sessionUser = await backendJson<SessionUser>('/auth/me', {
				token: accessToken,
				request
			});
		} catch (e) {
			// if auth fails, we might want to clear the token
			cookies.delete('backend-access-token', { path: '/' });
		}
	}

	return {
		accessToken: accessToken ?? null,
		sessionUser
	};
};
