<script lang="ts">
	import {browser} from '$app/environment';
	import {replaceState} from '$app/navigation';
	import type {Snippet} from 'svelte';
	import {onMount} from 'svelte';
	import {
		getSessionFlow,
		type KillTarget,
		listAdminProfileRequests,
		listKillReports,
		listKillTargets,
		listRankings
	} from '$lib/shared/api';
	import { buildSessionFlow, profileFlowTarget } from '$lib/pages/profile-flow';
	import type {KillReport, ProfileRequest, RankingEntry, SessionFlow, SessionUser} from '$lib/shared/model';
	import {sessionUser as sessionUserStore} from '$lib/shared/model';
	import {type AppContext, type AppView, appViewFromPath, setAppContext} from './app-context';

	type Props = {
		initialSessionFlow?: SessionFlow | null;
		initialSessionUser?: SessionUser | null;
		children: Snippet;
	};

	let { initialSessionFlow = null, initialSessionUser = null, children }: Props = $props();
	const getInitialSessionFlow = () => initialSessionFlow;
	const getInitialSessionUser = () => initialSessionUser ?? initialSessionFlow?.user ?? null;

	let view = $state<AppView>(browser ? appViewFromPath(window.location.pathname) : 'home');
	let sessionFlow = $state<SessionFlow>(getInitialSessionFlow() ?? buildSessionFlow(null, null));
	let sessionUser = $state<SessionUser | null>(getInitialSessionUser());
	let rankings = $state<RankingEntry[]>([]);
	let killTargets = $state<KillTarget[]>([]);
	let adminProfileRequests = $state<ProfileRequest[]>([]);
	let killReports = $state<KillReport[]>([]);

	const verification = Promise.resolve({
		refId: 'SPA',
		clearance: 'LOCAL',
		grantedAt: '1970-01-01T00:00:00.000Z'
	});

	sessionUserStore.set(getInitialSessionUser());

	onMount(() => {
		if (!browser) return;
		void refreshSession();

		// TODO: заменить это на gRPC server-side streaming и на бекенде пушить события в вебсокет, а тут просто слушать
		const pollTimer = setInterval(() => {
			void refreshSession();
		}, 5000);

		return () => clearInterval(pollTimer);
	});

	const adminViews: AppView[] = ['admin-moderation', 'admin-events'];
	const protectedViews: AppView[] = [
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
	];
	const userViews: AppView[] = ['dossier', 'waiting-clearance', 'profile-request-moderation'];

	const guardedView = (targetView: AppView): AppView => {
		if (userViews.includes(targetView) && !sessionUser) {
			return 'home';
		}

		if (protectedViews.includes(targetView) && !sessionFlow?.canPlay) {
			return sessionFlow ? appViewFromPath(profileFlowTarget(sessionFlow)) : 'home';
		}

		if (adminViews.includes(targetView) && !sessionUser?.is_admin) {
			return sessionFlow ? appViewFromPath(profileFlowTarget(sessionFlow)) : 'home';
		}

		return targetView;
	};

	const navigate = (target: AppView | string) => {
		view = guardedView(appViewFromPath(target));

		if (browser) {
			window.scrollTo(0, 0);

			try {
				replaceState('/', '');
			} catch {
				// Router not yet initialized — URL will be updated on next navigation
			}
		}
	};

	const setSessionUser = (user: SessionUser | null) => {
		sessionUser = user;
		sessionUserStore.set(user);
		sessionFlow = buildSessionFlow(user, sessionFlow?.latestProfileRequest ?? null);
	};

	const refreshSession = async () => {
		try {
			const nextFlow = await getSessionFlow();
			sessionFlow = nextFlow;
			sessionUser = nextFlow.user;
			sessionUserStore.set(nextFlow.user);

			if (guardedView(view) !== view) {
				navigate(profileFlowTarget(nextFlow));
			}
		} catch {
			// Network error — backend unreachable, keep current session
		}
	};

	const loadRankings = async () => {
		rankings = await listRankings().catch(() => []);
	};

	const loadKillTargets = async () => {
		killTargets = await listKillTargets().catch(() => []);
	};

	const loadAdminProfileRequests = async () => {
		adminProfileRequests = await listAdminProfileRequests().catch(() => []);
	};

	const loadKillReports = async () => {
		killReports = await listKillReports().catch(() => []);
	};

	const handleClick = (event: MouseEvent) => {
		if (
			event.defaultPrevented ||
			event.button !== 0 ||
			event.metaKey ||
			event.ctrlKey ||
			event.shiftKey ||
			event.altKey
		) {
			return;
		}

		const anchor = (event.target as Element | null)?.closest('a[href]') as HTMLAnchorElement | null;

		if (!anchor || anchor.target || anchor.hasAttribute('download')) {
			return;
		}

		const url = new URL(anchor.href);

		if (url.origin !== window.location.origin) {
			return;
		}

		event.preventDefault();
		navigate(`${url.pathname}${url.search}`);
	};

	const handleKeydown = () => {};

	const app: AppContext = {
		get view() {
			return view;
		},
		set view(v: AppView) {
			view = v;
		},
		get sessionFlow() {
			return sessionFlow;
		},
		get sessionUser() {
			return sessionUser;
		},
		get rankings() {
			return rankings;
		},
		get killTargets() {
			return killTargets;
		},
		get adminProfileRequests() {
			return adminProfileRequests;
		},
		get killReports() {
			return killReports;
		},
		verification,
		navigate,
		refreshSession,
		setSessionUser,
		loadRankings,
		loadKillTargets,
		loadAdminProfileRequests,
		loadKillReports
	};

	setAppContext(app);
</script>

<div role="presentation" onclick={handleClick} onkeydown={handleKeydown}>
	{@render children()}
</div>
