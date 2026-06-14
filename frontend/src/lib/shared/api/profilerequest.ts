import { createClient } from '@connectrpc/connect';
import {
	ProfileRequest,
	type ProfileRequestEvent
} from '$lib/proto/profilerequest/profilerequest_pb';
import { transport } from './transport';
import type {
	ProfileRequest as ModelProfileRequest,
	ProfileRequestStatus
} from '$lib/shared/model';

export const profileRequestClient = createClient(ProfileRequest, transport);

export function subscribeToProfileRequests(userId: string) {
	return profileRequestClient.subscribe({ userId });
}

const normalizeStatus = (status: string): ProfileRequestStatus => {
	const s = status.toLowerCase();
	if (s === 'confirmed' || s === 'approved') return 'approved';
	if (s === 'rejected') return 'rejected';
	return 'pending';
};

export function profileRequestEventToUpdate(
	event: ProfileRequestEvent
): Partial<ModelProfileRequest> {
	return {
		profile_request_id: event.profileRequestId,
		user_id: event.userId,
		status: normalizeStatus(event.status),
		reviewer_note: event.reviewerNote || null,
		reviewed_at: event.updatedAt || null,
		updated_at: event.updatedAt
	};
}
