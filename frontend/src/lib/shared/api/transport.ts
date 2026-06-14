import { type Interceptor } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';
import { readAccessToken } from '$lib/shared/auth';

export const BACKEND_URL = import.meta.env.PUBLIC_BACKEND_URL ?? 'http://localhost:3000';

const authInterceptor: Interceptor = (next) => async (req) => {
	const token = readAccessToken();
	if (token) {
		req.header.set('authorization', `Bearer ${token}`);
	}
	return await next(req);
};

export const transport = createGrpcWebTransport({
	baseUrl: BACKEND_URL + '/api',
	interceptors: [authInterceptor]
});
