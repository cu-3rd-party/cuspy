import { redirect } from '@sveltejs/kit';
import { profileFlowTarget } from '$lib/profile-flow';
import type { SessionFlow } from '$lib/stores/session';

export const requireAuthenticatedFlow = (flow: SessionFlow | undefined) => {
	if (!flow || flow.status === 'guest') {
		redirect(307, '/');
	}

	return flow;
};

export const requireGameplayFlow = (flow: SessionFlow | undefined) => {
	const authenticatedFlow = requireAuthenticatedFlow(flow);

	if (!authenticatedFlow.canPlay) {
		redirect(307, profileFlowTarget(authenticatedFlow));
	}

	return authenticatedFlow;
};

export const requireRegistrationFlow = (flow: SessionFlow | undefined) => {
	if (!flow || !flow.needsRegistration) {
		redirect(307, flow ? profileFlowTarget(flow) : '/');
	}

	return flow;
};
