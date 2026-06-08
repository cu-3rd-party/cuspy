import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { backendJson, debugAuthEnabled } from '$lib/server/backend';

export const POST: RequestHandler = async ({ request, cookies }) => {
	if (!debugAuthEnabled()) {
		return json({ error: 'Debug auth is disabled' }, { status: 404 });
	}

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
		sameSite: 'lax',
		secure: false
	});

	return json(response);
};
