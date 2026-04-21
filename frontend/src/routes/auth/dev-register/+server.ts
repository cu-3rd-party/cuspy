import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { backendJson } from '$lib/server/backend';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const payload = await request.json();
	const response = await backendJson<{
		access_token: string;
		user: Record<string, unknown>;
	}>('/auth/register', {
		method: 'POST',
		request,
		body: JSON.stringify(payload)
	});

	cookies.set('backend-access-token', response.access_token, {
		path: '/',
		httpOnly: false,
		sameSite: 'none',
		secure: true
	});
	cookies.set('session-user', JSON.stringify(response.user), {
		path: '/',
		httpOnly: false,
		sameSite: 'none',
		secure: true
	});

	return json(response);
};
