import { browser } from '$app/environment';

const TOKEN_KEY = 'backend-access-token';

export const readAccessToken = () => {
	if (!browser) return null;
	return window.localStorage.getItem(TOKEN_KEY);
};

export const writeAccessToken = (token: string) => {
	if (!browser) return;
	window.localStorage.setItem(TOKEN_KEY, token);
};

export const clearAccessToken = () => {
	if (!browser) return;
	window.localStorage.removeItem(TOKEN_KEY);
};
