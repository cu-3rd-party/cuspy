import { browser } from '$app/environment';

export type DossierDraft = {
	agentId: {
		codename: string;
		academicGroup: string;
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
	draft.agentId.identificationName.trim().length > 0 &&
	draft.agentId.identificationImage.trim().length > 0;

export const isBoundariesComplete = (_draft: DossierDraft) => true;

export const canAccessStep = (draft: DossierDraft, step: 1 | 2 | 3) => draft.unlockedStep >= step;
