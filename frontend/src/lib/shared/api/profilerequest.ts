import { createClient } from '@connectrpc/connect';
import { ProfileRequest, type SubscribeRequest } from '$lib/proto/profilerequest/profilerequest_pb';
import { transport } from './transport';

export const profileRequestClient = createClient(ProfileRequest, transport);

export function subscribeToProfileRequests(userId: string) {
	return profileRequestClient.subscribe({ userId });
}
