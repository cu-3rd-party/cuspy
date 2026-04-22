import { error, redirect } from '@sveltejs/kit';
import type { SessionFlow } from '$lib/stores/session';

export const requireAdminFlow = (flow: SessionFlow | undefined) => {
	if (!flow || flow.status === 'guest') {
		redirect(307, '/');
	}

	if (!flow.user?.is_admin) {
		throw error(403, 'Admin access required');
	}

	return flow;
};
