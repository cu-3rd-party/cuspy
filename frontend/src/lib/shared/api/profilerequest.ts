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

export function profileRequestEventToUpdate(
	event: ProfileRequestEvent
): Partial<ModelProfileRequest> {
	const status = event.status.toLowerCase() as ProfileRequestStatus;
	return {
		profile_request_id: event.profileRequestId,
		user_id: event.userId,
		status,
		reviewer_note: event.reviewerNote || null,
		reviewed_at: event.updatedAt || null,
		updated_at: event.updatedAt
	};
}
