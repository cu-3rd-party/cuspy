import { browser } from '$app/environment';

export type DossierDraft = {
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
	unlockedStep: 1 | 2 | 3;
};

const STORAGE_KEY = 'dossier-draft';

export const createDefaultDossierDraft = (): DossierDraft => ({
	agentId: {
		codename: '',
		academicGroup: '',
		academicLevel: '',
		courseNumber: '',
		bachelorTrack: '',
		identificationName: '',
		identificationImage: ''
	},
	boundaries: {
		physicalContact: true,
		hugsCloseProximity: false
	},
	unlockedStep: 1
});

export const loadDossierDraft = (): DossierDraft => {
	if (!browser) {
		return createDefaultDossierDraft();
	}

	const stored = window.localStorage.getItem(STORAGE_KEY);
	if (!stored) {
		return createDefaultDossierDraft();
	}

	try {
		const parsed = JSON.parse(stored) as Partial<DossierDraft>;
		const defaults = createDefaultDossierDraft();

		return {
			...defaults,
			...parsed,
			agentId: {
				...defaults.agentId,
				...parsed.agentId
			},
			boundaries: {
				...defaults.boundaries,
				...parsed.boundaries
			}
		};
	} catch {
		return createDefaultDossierDraft();
	}
};

export const saveDossierDraft = (draft: DossierDraft) => {
	if (!browser) {
		return;
	}

	window.localStorage.setItem(STORAGE_KEY, JSON.stringify(draft));
};

export const clearDossierDraft = () => {
	if (!browser) {
		return;
	}

	window.localStorage.removeItem(STORAGE_KEY);
};

export const isAgentIdComplete = (draft: DossierDraft) =>
	draft.agentId.codename.trim().length > 0 &&
	draft.agentId.academicGroup.trim().length > 0 &&
	draft.agentId.academicLevel !== '' &&
	(draft.agentId.academicLevel === 'bachelor'
		? ['1', '2', '3', '4'].includes(draft.agentId.courseNumber)
		: true) &&
	(draft.agentId.academicLevel === 'master'
		? ['1', '2'].includes(draft.agentId.courseNumber)
		: true) &&
	(draft.agentId.academicLevel === 'other' ? draft.agentId.courseNumber === '' : true) &&
	(draft.agentId.academicLevel === 'bachelor' && Number(draft.agentId.courseNumber) >= 2
		? draft.agentId.bachelorTrack !== ''
		: true) &&
	(draft.agentId.academicLevel === 'bachelor' || draft.agentId.academicLevel === 'master'
		? draft.agentId.courseNumber !== ''
		: draft.agentId.academicLevel === 'other') &&
	(draft.agentId.academicLevel !== 'bachelor' ? draft.agentId.bachelorTrack === '' : true) &&
	draft.agentId.identificationName.trim().length > 0 &&
	draft.agentId.identificationImage.trim().length > 0;

export const isBoundariesComplete = (_draft: DossierDraft) => true;

export const canAccessStep = (draft: DossierDraft, step: 1 | 2 | 3) => draft.unlockedStep >= step;
