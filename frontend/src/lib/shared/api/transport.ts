import { createGrpcWebTransport } from '@connectrpc/connect-web';

export const BACKEND_URL = import.meta.env.VITE_GRPC_HOST ?? 'http://localhost:6969';

export const transport = createGrpcWebTransport({
	baseUrl: BACKEND_URL
});
