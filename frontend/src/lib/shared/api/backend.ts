import { env } from '$env/dynamic/public';
import {
	clearAccessToken,
	readAccessToken,
	writeAccessToken,
	readAuthPayload,
	type AuthPayload
} from '$lib/shared/auth';
import { buildSessionFlow } from '$lib/pages/profile-flow';
import type {
	AgentProfileData,
	KillReport,
	ProfileRequest,
	ProfileRequestStatus,
	RankingEntry,
	SessionFlow,
	SessionUser
} from '$lib/shared/model';

const DEFAULT_BACKEND_URL = 'http://127.0.0.1:3000/api';

type BackendUser = Omit<SessionUser, 'agent_data'> & {
	agent_data?: AgentProfileData | null;
};

type BackendAgentData = {
	agent_data_id: string;
	codename: string | null;
	academic_group: string | null;
	academic_level: 'Bachelor' | 'Master' | null;
	course_number: number | null;
	bachelor_track: 'SWE' | 'AI' | 'BA' | null;
	identification_name: string | null;
	identification_image_id: string | null;
	physical_contact_allowed: boolean;
	hugs_close_proximity_allowed: boolean;
};

type BackendResource = {
	resource_id?: string;
	file_location?: string | null;
	content_type?: string | null;
	filename?: string | null;
	[key: string]: unknown;
};

type BackendProfileRequest = Omit<ProfileRequest, 'requested_profile_data' | 'status'> & {
	status: string;
	requested_profile_data?: AgentProfileData;
};

type BackendKillEvent = {
	kill_event_id: string;
	killer_id: string;
	victim_id: string;
	status: string;
	evidence_url: string | null;
	details: Record<string, unknown> | null;
	moderation_reason: string | null;
	reported_at: string;
	created_at: string;
	updated_at: string | null;
	reviewed_at?: string | null;
};

export type KillTarget = {
	target_id: string;
	identifier: string;
	last_known_location: string;
	status: 'active' | 'eliminated';
};

const backendBaseUrl = () => (env.PUBLIC_BACKEND_URL || DEFAULT_BACKEND_URL).replace(/\/$/, '');

const authHeaders = (token = readAccessToken()): Record<string, string> =>
	token ? { authorization: `Bearer ${token}` } : {};

export async function backendJson<T>(
	path: string,
	options: RequestInit = {},
	customFetch?: typeof fetch
): Promise<T> {
	const fetcher = customFetch ?? fetch;
	const headers = new Headers(options.headers);
	const token = readAccessToken();

	if (token && !headers.has('authorization')) {
		headers.set('authorization', `Bearer ${token}`);
	}

	if (options.body && !(options.body instanceof FormData) && !headers.has('content-type')) {
		headers.set('content-type', 'application/json');
	}

	const response = await fetcher(`${backendBaseUrl()}${path}`, {
		...options,
		headers
	});

	if (!response.ok) {
		let message = `Backend request failed with status ${response.status}`;
		try {
			const payload = (await response.json()) as { error?: string };
			message = payload.error ?? message;
		} catch {
			// ignore non-json backend errors
		}

		throw new Error(message);
	}

	if (response.status === 204) {
		return undefined as T;
	}

	return (await response.json()) as T;
}

export const registerUser = async (
	payload: {
		email?: string | null;
		password?: string | null;
		telegram_id?: number | null;
		agent_name?: string | null;
	},
	customFetch?: typeof fetch
) => {
	const response = await backendJson<{ access_token: string; user: BackendUser }>(
		'/auth/register',
		{
			method: 'POST',
			body: JSON.stringify(payload)
		},
		customFetch
	);

	return {
		access_token: response.access_token,
		user: await normalizeUser(response.user, customFetch)
	};
};

export const getCurrentUser = async (token = readAccessToken(), customFetch?: typeof fetch) => {
	const user = await backendJson<BackendUser>(
		'/auth/me',
		{ headers: authHeaders(token) },
		customFetch
	);
	return normalizeUser(user, customFetch);
};

export const loginUser = async (payload: AuthPayload, customFetch?: typeof fetch) => {
	const response = await backendJson<{ access_token: string; user: BackendUser }>(
		'/auth/login',
		{
			method: 'POST',
			body: JSON.stringify(payload)
		},
		customFetch
	);

	return {
		access_token: response.access_token,
		user: await normalizeUser(response.user, customFetch)
	};
};

export const getSessionFlow = async (
	token = readAccessToken(),
	customFetch?: typeof fetch
): Promise<SessionFlow> => {
	if (!token) {
		return buildSessionFlow(null, null);
	}

	try {
		const user = await getCurrentUser(token, customFetch);
		const requests = await listProfileRequests(token, customFetch);
		return buildSessionFlow(user, requests[0] ?? null, requests);
	} catch (error) {
		// Network error — backend unreachable, don't touch the token
		if (error instanceof TypeError) {
			throw error;
		}

		// Token expired or invalid — try to re-login with stored telegram_id
		const authPayload = readAuthPayload();

		if (authPayload != null) {
			try {
				const loginPayload = await loginUser(authPayload, customFetch);
				writeAccessToken(loginPayload.access_token);
				const user = loginPayload.user;
				const requests = await listProfileRequests(loginPayload.access_token, customFetch);
				return buildSessionFlow(user, requests[0] ?? null, requests);
			} catch {
				// re-login also failed
			}
		}

		clearAccessToken();
		return buildSessionFlow(null, null);
	}
};

export const createAgentData = async (
	profileData: AgentProfileData,
	customFetch?: typeof fetch
) => {
	const formData = new FormData();
	formData.set('data', JSON.stringify(toAgentDataMetadata(profileData)));

	const imageFile = await dataUrlToFile(
		profileData.identificationImage,
		profileData.identificationName || 'identification.png'
	);
	if (imageFile) {
		formData.set('image', imageFile);
	}

	const created = await backendJson<BackendAgentData>(
		'/agent-data',
		{
			method: 'POST',
			body: formData
		},
		customFetch
	);

	return normalizeAgentData(created, profileData.identificationImage, customFetch);
};

const agentDataCache = new Map<string, AgentProfileData>();

export const getAgentData = async (agentDataId: string, customFetch?: typeof fetch) => {
	const cached = agentDataCache.get(agentDataId);
	if (cached) return cached;

	const data = await backendJson<BackendAgentData>(`/agent-data/${agentDataId}`, {}, customFetch);
	const profile = await normalizeAgentData(data, undefined, customFetch);
	agentDataCache.set(agentDataId, profile);
	return profile;
};

export const getResourceMetadata = (resourceId: string, customFetch?: typeof fetch) =>
	backendJson<BackendResource>(`/resource/${encodeURIComponent(resourceId)}`, {}, customFetch);

export const deleteProfileRequest = async (requestId: string, token = readAccessToken()) =>
	backendJson<undefined>(`/profile-requests/${encodeURIComponent(requestId)}`, {
		method: 'DELETE',
		headers: authHeaders(token)
	});

export const createProfileRequest = async (
	agentDataId: string,
	token = readAccessToken(),
	customFetch?: typeof fetch
) =>
	normalizeProfileRequest(
		await backendJson<BackendProfileRequest>(
			'/profile-requests',
			{
				method: 'POST',
				headers: authHeaders(token),
				body: JSON.stringify({ agent_data_id: agentDataId })
			},
			customFetch
		),
		customFetch
	);

export const listProfileRequests = async (
	token = readAccessToken(),
	customFetch?: typeof fetch
) => {
	const requests = await backendJson<BackendProfileRequest[]>(
		'/profile-requests',
		{
			headers: authHeaders(token)
		},
		customFetch
	);

	return hydrateProfileRequests(requests, customFetch);
};

export const listAdminProfileRequests = async (token = readAccessToken()) => {
	const requests = await backendJson<BackendProfileRequest[]>('/admin/profile-requests/', {
		headers: authHeaders(token)
	});

	return hydrateProfileRequests(requests);
};

export const moderateProfileRequest = async ({
	requestId,
	decision,
	reviewerNote,
	token = readAccessToken()
}: {
	requestId: string;
	decision: 'approved' | 'rejected';
	reviewerNote: string;
	token?: string | null;
}) =>
	normalizeProfileRequest(
		await backendJson<BackendProfileRequest>(`/admin/profile-requests/${requestId}`, {
			method: 'PATCH',
			headers: authHeaders(token),
			body: JSON.stringify({
				status: decision === 'approved' ? 'confirmed' : 'rejected',
				reviewer_note: reviewerNote || null
			})
		})
	);

export const listRankings = (token = readAccessToken()) =>
	backendJson<RankingEntry[]>('/stats/rankings', { headers: authHeaders(token) });

export const listKillTargets = async (token = readAccessToken()): Promise<KillTarget[]> => {
	const rankings = await listRankings(token);
	return rankings.map((entry) => ({
		target_id: entry.user_id,
		identifier: entry.agent_name ?? `AGENT_${entry.user_id.slice(0, 4).toUpperCase()}`,
		last_known_location: 'CLASSIFIED',
		status: 'active'
	}));
};

export const reportKill = ({
	victimId,
	modusOperandi,
	witnessPresent,
	token = readAccessToken()
}: {
	victimId: string;
	modusOperandi: string;
	witnessPresent: boolean;
	token?: string | null;
}) =>
	backendJson<BackendKillEvent>('/kill/', {
		method: 'POST',
		headers: authHeaders(token),
		body: JSON.stringify({
			victim_id: victimId,
			details: {
				modus_operandi: modusOperandi,
				witness_present: witnessPresent
			}
		})
	});

export const listKillReports = async (token = readAccessToken()): Promise<KillReport[]> => {
	const reports = await backendJson<BackendKillEvent[]>('/kill/', { headers: authHeaders(token) });
	return reports.map(normalizeKillReport);
};

export const moderateKillReport = ({
	reportId,
	decision,
	reviewerNote,
	token = readAccessToken()
}: {
	reportId: string;
	decision: 'confirmed' | 'rejected';
	reviewerNote: string;
	token?: string | null;
}) =>
	backendJson<BackendKillEvent>(`/kill/${reportId}/moderate`, {
		method: 'POST',
		headers: authHeaders(token),
		body: JSON.stringify({
			action: decision === 'confirmed' ? 'APPROVE' : 'REJECT',
			reason: reviewerNote || null
		})
	});

const hydrateProfileRequests = async (
	requests: BackendProfileRequest[],
	customFetch?: typeof fetch
) => Promise.all(requests.map((request) => normalizeProfileRequest(request, customFetch)));

const normalizeUser = async (
	user: BackendUser,
	customFetch?: typeof fetch
): Promise<SessionUser> => ({
	...user,
	agent_data: user.agent_data_id
		? await getAgentData(user.agent_data_id, customFetch)
		: (user.agent_data ?? null)
});

const normalizeProfileRequest = async (
	request: BackendProfileRequest,
	customFetch?: typeof fetch
): Promise<ProfileRequest> => ({
	...request,
	status: normalizeProfileStatus(request.status),
	requested_profile_data:
		request.requested_profile_data ??
		(await getAgentData(request.requested_profile_data_id, customFetch))
});

const normalizeProfileStatus = (status: string): ProfileRequestStatus => {
	const normalized = status.toLowerCase();
	if (normalized === 'confirmed' || normalized === 'approved') return 'approved';
	if (normalized === 'rejected') return 'rejected';
	return 'pending';
};

const normalizeAgentData = async (
	data: BackendAgentData,
	identificationImage?: string | null,
	customFetch?: typeof fetch
): Promise<AgentProfileData> => {
	const identificationImageResourceId = data.identification_image_id ?? undefined;

	return {
		agentDataId: data.agent_data_id,
		codename: data.codename ?? undefined,
		academicGroup: data.academic_group ?? undefined,
		academicLevel:
			data.academic_level === 'Bachelor'
				? 'bachelor'
				: data.academic_level === 'Master'
					? 'master'
					: undefined,
		courseNumber: data.course_number == null ? undefined : String(data.course_number),
		bachelorTrack:
			data.bachelor_track === 'SWE'
				? 'development'
				: data.bachelor_track === 'AI'
					? 'ai'
					: data.bachelor_track === 'BA'
						? 'business'
						: undefined,
		identificationName: data.identification_name ?? undefined,
		identificationImageResourceId,
		identificationImage: identificationImage ?? undefined,
		boundaries: {
			physicalContact: data.physical_contact_allowed,
			hugsCloseProximity: data.hugs_close_proximity_allowed
		}
	};
};

const resourceImageCache = new Map<string, string>();

export async function resolveAgentImage(profile: {
	identificationImage?: string;
	identificationImageResourceId?: string;
}): Promise<string | undefined> {
	if (profile.identificationImage) return profile.identificationImage;
	if (!profile.identificationImageResourceId) return undefined;

	const cached = resourceImageCache.get(profile.identificationImageResourceId);
	if (cached) return cached;

	try {
		const metadata = await getResourceMetadata(profile.identificationImageResourceId);
		const url = metadata.file_location ?? undefined;
		if (url) resourceImageCache.set(profile.identificationImageResourceId, url);
		return url;
	} catch {
		return undefined;
	}
}

const resolveResourceFileLocation = async (resourceId?: string, customFetch?: typeof fetch) => {
	if (!resourceId) {
		return undefined;
	}

	if (
		resourceId.startsWith('http://') ||
		resourceId.startsWith('https://') ||
		resourceId.startsWith('data:')
	) {
		return resourceId;
	}

	const metadata = await getResourceMetadata(resourceId, customFetch);
	return metadata.file_location ?? undefined;
};

const toAgentDataMetadata = (profileData: AgentProfileData) => ({
	codename: profileData.codename || null,
	academic_group: profileData.academicGroup || null,
	academic_level:
		profileData.academicLevel === 'bachelor'
			? 'Bachelor'
			: profileData.academicLevel === 'master'
				? 'Master'
				: null,
	course_number: profileData.courseNumber ? Number(profileData.courseNumber) : null,
	bachelor_track:
		profileData.bachelorTrack === 'development'
			? 'SWE'
			: profileData.bachelorTrack === 'ai'
				? 'AI'
				: profileData.bachelorTrack === 'business'
					? 'BA'
					: null,
	identification_name: profileData.identificationName || null,
	physical_contact_allowed: profileData.boundaries?.physicalContact ?? true,
	hugs_close_proximity_allowed: profileData.boundaries?.hugsCloseProximity ?? false
});

const normalizeKillReport = (report: BackendKillEvent): KillReport => {
	const details = report.details ?? {};
	const status = report.status.toUpperCase();

	return {
		kill_report_id: report.kill_event_id,
		reporter_user_id: report.killer_id,
		reporter_codename: `USER_${report.killer_id.slice(0, 4).toUpperCase()}`,
		target_id: report.victim_id,
		target_identifier: `USER_${report.victim_id.slice(0, 4).toUpperCase()}`,
		modus_operandi: String(details.modus_operandi ?? 'No details supplied'),
		witness_present: Boolean(details.witness_present),
		status:
			status === 'ADMIN_APPROVED' || status === 'CONFIRMED'
				? 'confirmed'
				: status === 'REJECTED' || status === 'ADMIN_REJECTED'
					? 'rejected'
					: 'pending',
		reviewer_note: report.moderation_reason,
		created_at: report.created_at,
		updated_at: report.updated_at ?? report.created_at,
		reviewed_at: report.reviewed_at ?? null
	};
};

const dataUrlToFile = async (dataUrl: unknown, fallbackName: string) => {
	if (typeof dataUrl !== 'string' || !dataUrl.startsWith('data:')) {
		return null;
	}

	const response = await fetch(dataUrl);
	const blob = await response.blob();
	const extension = blob.type.split('/')[1] || 'png';
	const safeName = fallbackName.includes('.') ? fallbackName : `${fallbackName}.${extension}`;
	return new File([blob], safeName, { type: blob.type });
};
