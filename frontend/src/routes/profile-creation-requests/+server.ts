import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { backendJson } from '$lib/server/backend';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const payload = await request.json();
	const token = cookies.get('backend-access-token') ?? request.headers.get('authorization')?.replace(/^Bearer\s+/, '');

	const response = await backendJson<Record<string, unknown>>('/profile-creation-requests', {
		method: 'POST',
		request,
		token: token ?? undefined,
		body: JSON.stringify(payload)
	});

	return json(response);
};
