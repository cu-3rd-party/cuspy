import { writable, derived } from 'svelte/store';

export type AgentProfileData = {
	codename?: string;
	academicGroup?: string;
	academicLevel?: string;
	courseNumber?: string;
	bachelorTrack?: string;
	identificationName?: string;
	identificationImage?: string;
	boundaries?: {
		physicalContact?: boolean;
		hugsCloseProximity?: boolean;
	};
	[key: string]: unknown;
};

export type SessionUser = {
	user_id: string;
	telegram_id: number;
	rating: number;
	agent_name: string | null;
	agent_data: AgentProfileData;
	is_admin: boolean;
	created_at: string;
	updated_at: string | null;
};

export type ProfileRequestStatus = 'sent' | 'confirmed' | 'rejected';

export type ProfileRequest = {
	profile_creation_request_id: string;
	user_id: string;
	requested_profile_data: AgentProfileData;
	status: ProfileRequestStatus;
	reviewer_note: string | null;
	reviewed_at: string | null;
	created_at: string;
	updated_at: string;
};

export type SessionFlowStatus = 'guest' | 'no_profile' | 'pending' | 'rejected' | 'approved';

export type SessionFlow = {
	status: SessionFlowStatus;
	user: SessionUser | null;
	latestProfileRequest: ProfileRequest | null;
	canPlay: boolean;
	needsRegistration: boolean;
	needsProfileEdit: boolean;
};

export type RankingEntry = {
	rank: number;
	user_id: string;
	agent_name: string | null;
	rating: number;
	approved_kills: number;
	approved_deaths: number;
};

export type UserStats = {
	user_id: string;
	rating: number;
	approved_kills: number;
	approved_deaths: number;
	pending_kills: number;
};

export const sessionUser = writable<SessionUser | null>(null);

export const canAccessGameplay = (flow: SessionFlow): boolean => flow.canPlay;

export const canAccessEnlistment = (flow: SessionFlow): boolean =>
	flow.status === 'guest' || flow.status === 'no_profile' || flow.status === 'rejected';

export const getLatestProfileRequest = (flow: SessionFlow): ProfileRequest | null =>
	flow.latestProfileRequest;
