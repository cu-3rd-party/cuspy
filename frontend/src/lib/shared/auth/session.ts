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
