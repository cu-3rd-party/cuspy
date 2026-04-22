import { backendJson } from '$lib/server/backend';
import { buildSessionFlow } from '$lib/profile-flow';
import type { ProfileRequest, SessionFlow, SessionUser } from '$lib/stores/session';

type LoadSessionFlowArgs = {
	accessToken: string | null;
	request: Request;
	cookies: { delete: (name: string, options: { path: string }) => void };
};

export const loadSessionFlow = async ({
	accessToken,
	request,
	cookies
}: LoadSessionFlowArgs): Promise<SessionFlow> => {
	if (!accessToken) {
		return buildSessionFlow(null, null);
	}

	try {
		const user = await backendJson<SessionUser>('/auth/me', {
			token: accessToken,
			request
		});
		const profileRequests = await backendJson<ProfileRequest[]>('/profile-creation-requests', {
			token: accessToken,
			request
		});

		return buildSessionFlow(user, profileRequests[0] ?? null);
	} catch {
		cookies.delete('backend-access-token', { path: '/' });
		return buildSessionFlow(null, null);
	}
};
