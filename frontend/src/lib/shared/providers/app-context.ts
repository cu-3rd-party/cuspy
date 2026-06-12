import { createContext } from 'svelte';
import type { KillTarget } from '$lib/shared/api';
import type {
	KillReport,
	ProfileRequest,
	RankingEntry,
	SessionFlow,
	SessionUser
} from '$lib/shared/model';

export type AppView =
	| 'home'
	| 'agent-id'
	| 'operational-boundaries'
	| 'dossier-verification'
	| 'dossier'
	| 'waiting-clearance'
	| 'target-intel'
	| 'surveillance'
	| 'missions'
	| 'loot'
	| 'perks'
	| 'report-kill'
	| 'reveal-confirmation'
	| 'rankings'
	| 'admin-moderation'
	| 'admin-events';

export type LandingVerification = Promise<{
	refId: string;
	clearance: string;
	grantedAt: string;
}>;

export type AppContext = {
	view: AppView;
	readonly sessionFlow: SessionFlow | null;
	readonly sessionUser: SessionUser | null;
	readonly rankings: RankingEntry[];
	readonly killTargets: KillTarget[];
	readonly adminProfileRequests: ProfileRequest[];
	readonly killReports: KillReport[];
	readonly verification: LandingVerification;
	navigate: (target: AppView | string) => void;
	refreshSession: () => Promise<void>;
	setSessionUser: (user: SessionUser | null) => void;
	loadRankings: () => Promise<void>;
	loadKillTargets: () => Promise<void>;
	loadAdminProfileRequests: () => Promise<void>;
	loadKillReports: () => Promise<void>;
};

export const [getAppContext, setAppContext] = createContext<AppContext>();

export const appViewFromPath = (path: string): AppView => {
	const normalized = path.split('?')[0].replace(/^\//, '').replace(/\/$/, '');

	if (normalized === '') return 'home';
	if (normalized === 'admin/moderation') return 'admin-moderation';
	if (normalized === 'admin/events') return 'admin-events';

	return isAppView(normalized) ? normalized : 'home';
};

export const appPathFromView = (view: AppView) => {
	if (view === 'home') return '/';
	if (view === 'admin-moderation') return '/admin/moderation';
	if (view === 'admin-events') return '/admin/events';
	return `/${view}`;
};

const isAppView = (value: string): value is AppView =>
	[
		'home',
		'agent-id',
		'operational-boundaries',
		'dossier-verification',
		'dossier',
		'waiting-clearance',
		'target-intel',
		'surveillance',
		'missions',
		'loot',
		'perks',
		'report-kill',
		'reveal-confirmation',
		'rankings',
		'admin-moderation',
		'admin-events'
	].includes(value);
