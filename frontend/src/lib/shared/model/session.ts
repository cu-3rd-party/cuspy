import { writable, derived } from 'svelte/store';

export type AgentProfileData = {
	codename?: string;
	academicGroup?: string;
	academicLevel?: string;
	courseNumber?: string;
	bachelorTrack?: string;
	identificationName?: string;
	identificationImage?: string;
	identificationImageResourceId?: string;
	agentDataId?: string;
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
	username: string | null;
	agent_data_id: string | null;
	agent_data: AgentProfileData | null;
	is_admin: boolean;
	created_at: string;
	updated_at: string | null;
};

export type ProfileRequestStatus = 'pending' | 'approved' | 'rejected';

export type ProfileRequest = {
	profile_request_id: string;
	user_id: string;
	requested_profile_data_id: string;
	requested_profile_data: AgentProfileData;
	status: ProfileRequestStatus;
	reviewer_note: string | null;
	reviewed_at: string | null;
	created_at: string;
	updated_at: string;
};

export type SessionFlowStatus = 'guest' | 'no_profile' | 'pending' | 'rejected' | 'approved';

export type RequestState =
	| { type: 'only-pending' }
	| { type: 'approved-pending' }
	| { type: 'approved-rejected' }
	| { type: 'approved-multiple' };

export type SessionFlow = {
	status: SessionFlowStatus;
	user: SessionUser | null;
	latestProfileRequest: ProfileRequest | null;
	allRequests: ProfileRequest[];
	canPlay: boolean;
	needsRegistration: boolean;
	needsProfileEdit: boolean;
};

export type RankingEntry = {
	rank: number;
	user_id: string;
	username: string | null;
	rating: number;
	approved_kills: number;
	approved_deaths: number;
};

export type KillReportStatus = 'pending' | 'confirmed' | 'rejected';

export type KillReport = {
	kill_report_id: string;
	reporter_user_id: string;
	reporter_codename: string;
	target_id: string;
	target_identifier: string;
	modus_operandi: string;
	witness_present: boolean;
	status: KillReportStatus;
	reviewer_note: string | null;
	created_at: string;
	updated_at: string;
	reviewed_at: string | null;
};

export type UserStats = {
	user_id: string;
	rating: number;
	approved_kills: number;
	approved_deaths: number;
	pending_kills: number;
};

export const deriveRequestState = (flow: SessionFlow): RequestState => {
	const all = flow.allRequests;
	const hasApproved = all.some((r) => r.status === 'approved');
	const hasRejected = all.some((r) => r.status === 'rejected');
	const extra = all.filter((r) => r.status !== 'approved');

	if (!hasApproved) return { type: 'only-pending' };
	if (hasRejected && extra.length > 1) return { type: 'approved-multiple' };
	if (hasRejected) return { type: 'approved-rejected' };
	if (extra.length > 1) return { type: 'approved-multiple' };
	if (extra.length === 1) return { type: 'approved-pending' };
	return { type: 'approved-pending' };
};

export const sessionUser = writable<SessionUser | null>(null);

export const canAccessGameplay = (flow: SessionFlow): boolean => flow.canPlay;

export const canAccessEnlistment = (flow: SessionFlow): boolean =>
	flow.status === 'guest' || flow.status === 'no_profile' || flow.status === 'rejected';

export const getLatestProfileRequest = (flow: SessionFlow): ProfileRequest | null =>
	flow.latestProfileRequest;
