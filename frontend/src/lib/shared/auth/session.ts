import { browser } from '$app/environment';

const TOKEN_KEY = 'backend-access-token';

const readCookieToken = () => {
	const cookie = document.cookie
		.split('; ')
		.find((entry) => entry.startsWith(`${TOKEN_KEY}=`))
		?.slice(TOKEN_KEY.length + 1);

	return cookie ? decodeURIComponent(cookie) : null;
};

export const readAccessToken = () => {
	if (!browser) return null;
	return window.localStorage.getItem(TOKEN_KEY) || readCookieToken();
};

export const writeAccessToken = (token: string) => {
	if (!browser) return;
	window.localStorage.setItem(TOKEN_KEY, token);
	document.cookie = `${TOKEN_KEY}=${encodeURIComponent(token)}; path=/; SameSite=Lax`;
};

export const clearAccessToken = () => {
	if (!browser) return;
	window.localStorage.removeItem(TOKEN_KEY);
	document.cookie = `${TOKEN_KEY}=; path=/; max-age=0; SameSite=Lax`;
};

const AUTH_PAYLOAD_KEY = 'auth-data';

export interface AuthPayload {
	email: string;
	password: string;
	telegram_id: number;
	agent_name: string;
}

export const readAuthPayload = (): AuthPayload | null => {
	if (!browser) return null;
	const raw = window.localStorage.getItem(AUTH_PAYLOAD_KEY);
	return raw ? JSON.parse(raw) : null;
};

export const writeAuthPayload = (payload: AuthPayload) => {
	if (!browser) return;
	window.localStorage.setItem(AUTH_PAYLOAD_KEY, JSON.stringify(payload));
};
