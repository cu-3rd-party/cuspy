import type {
	AgentProfileData,
	ProfileRequest,
	SessionFlow,
	SessionFlowStatus,
	SessionUser
} from '$lib/shared/model';

export const buildSessionFlow = (
	user: SessionUser | null,
	latestProfileRequest: ProfileRequest | null,
	allRequests: ProfileRequest[] = []
): SessionFlow => {
	const status = deriveSessionFlowStatus(user, latestProfileRequest);
	const anyApproved = allRequests.length > 0
		? allRequests.some((r) => r.status === 'approved')
		: status === 'approved';

	return {
		status,
		user,
		latestProfileRequest,
		allRequests,
		canPlay: anyApproved,
		needsRegistration: status === 'no_profile' || status === 'guest',
		needsProfileEdit: status === 'rejected'
	};
};

export const deriveSessionFlowStatus = (
	user: SessionUser | null,
	latestProfileRequest: ProfileRequest | null
): SessionFlowStatus => {
	if (!user) {
		return 'guest';
	}

	if (!latestProfileRequest) {
		return 'no_profile';
	}

	if (latestProfileRequest.status === 'approved') {
		return 'approved';
	}

	if (latestProfileRequest.status === 'rejected') {
		return 'rejected';
	}

	return 'pending';
};

export const profileFlowTarget = (flow: SessionFlow): string => {
	if (flow.status === 'guest') {
		return '/';
	}

	if (flow.status === 'no_profile') {
		return '/agent-id';
	}

	if (flow.status === 'rejected') {
		return '/agent-id?mode=edit';
	}

	if (flow.status === 'pending') {
		return '/waiting-clearance';
	}

	return '/target-intel';
};

export const canAccessGameplay = (flow: SessionFlow) => flow.canPlay;

export const canAccessEnlistment = (flow: SessionFlow) =>
	flow.status === 'guest' || flow.status === 'no_profile' || flow.status === 'rejected';

type DossierDraftShape = {
	agentId: {
		codename: string;
		academicGroup: string;
		academicLevel: 'bachelor' | 'master' | 'other' | '';
		courseNumber: '1' | '2' | '3' | '4' | '';
		bachelorTrack: 'development' | 'ai' | 'business' | '';
		identificationName: string;
		identificationImage: string;
	};
	boundaries: {
		physicalContact: boolean;
		hugsCloseProximity: boolean;
	};
	registrationCompleted: boolean;
	unlockedStep: 1 | 2 | 3;
};

export const applyProfileDataToDraft = (
	draft: DossierDraftShape,
	profileData: AgentProfileData
): DossierDraftShape => ({
	...draft,
	agentId: {
		...draft.agentId,
		codename: asString(profileData.codename),
		academicGroup: asString(profileData.academicGroup),
		academicLevel: asAcademicLevel(profileData.academicLevel),
		courseNumber: asCourseNumber(profileData.courseNumber),
		bachelorTrack: asBachelorTrack(profileData.bachelorTrack),
		identificationName: asString(profileData.identificationName),
		identificationImage: asString(profileData.identificationImage)
	},
	boundaries: {
		physicalContact:
			typeof profileData.boundaries?.physicalContact === 'boolean'
				? profileData.boundaries.physicalContact
				: draft.boundaries.physicalContact,
		hugsCloseProximity:
			typeof profileData.boundaries?.hugsCloseProximity === 'boolean'
				? profileData.boundaries.hugsCloseProximity
				: draft.boundaries.hugsCloseProximity
	},
	registrationCompleted: true,
	unlockedStep: 3
});

const asString = (value: unknown) => (typeof value === 'string' ? value : '');

const asAcademicLevel = (value: unknown): DossierDraftShape['agentId']['academicLevel'] =>
	value === 'bachelor' || value === 'master' || value === 'other'
		? value
		: value === 'Bachelor'
			? 'bachelor'
			: value === 'Master'
				? 'master'
				: '';

const asCourseNumber = (value: unknown): DossierDraftShape['agentId']['courseNumber'] =>
	value === '1' || value === '2' || value === '3' || value === '4' ? value : '';

const asBachelorTrack = (value: unknown): DossierDraftShape['agentId']['bachelorTrack'] =>
	value === 'development' || value === 'ai' || value === 'business'
		? value
		: value === 'SWE'
			? 'development'
			: value === 'AI'
				? 'ai'
				: value === 'BA'
					? 'business'
					: '';
