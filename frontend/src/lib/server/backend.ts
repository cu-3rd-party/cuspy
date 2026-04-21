import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';

const DEFAULT_BACKEND_URL = 'http://127.0.0.1:3000';

export const backendBaseUrl = () => env.BACKEND_URL || DEFAULT_BACKEND_URL;

export const telegramInitDataHeader = (request: Request) => {
	const initData = request.headers.get('x-telegram-init-data');
	return initData ? { 'x-telegram-init-data': initData } : {};
};

export async function backendJson<T>(
	path: string,
	options: RequestInit & { request?: Request; token?: string } = {}
): Promise<T> {
	const headers = new Headers(options.headers);
	headers.set('content-type', 'application/json');

	if (options.token) {
		headers.set('authorization', `Bearer ${options.token}`);
	}

	if (options.request) {
		for (const [key, value] of Object.entries(telegramInitDataHeader(options.request))) {
			headers.set(key, value);
		}
	}

	const response = await fetch(`${backendBaseUrl()}${path}`, {
		...options,
		headers
	});

	if (!response.ok) {
		let message = `Backend request failed with status ${response.status}`;
		try {
			const payload = (await response.json()) as { error?: string };
			if (payload.error) message = payload.error;
		} catch {
			// ignore non-json backend errors
		}

		throw error(response.status, message);
	}

	return (await response.json()) as T;
}
