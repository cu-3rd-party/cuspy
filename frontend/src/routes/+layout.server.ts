import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => ({
	accessToken: cookies.get('backend-access-token') ?? null,
	sessionUser: cookies.get('session-user') ? JSON.parse(cookies.get('session-user')!) : null
});
