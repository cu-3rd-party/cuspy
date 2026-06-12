<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import type { Pathname } from '$app/types';
	import { appPathFromView, getAppContext } from '$lib/shared/providers';
	import type { AppContext } from '$lib/shared/providers';
	import type { SessionFlow } from '$lib/shared/model';

	let {
		flow = undefined,
		homeHref = '/'
	}: {
		flow?: SessionFlow;
		homeHref?: string;
	} = $props();
	let app: AppContext | null = null;

	try {
		app = getAppContext();
	} catch {
		app = null;
	}

	let codename = $derived(
		(flow?.user?.agent_data?.codename as string | undefined) ??
			flow?.user?.agent_name ??
			'OPERATIVE_X'
	);
	let clearance = $derived(
		flow?.status === 'approved'
			? 'PHANTOM'
			: flow?.status === 'pending'
				? 'QUEUE'
				: flow?.status === 'rejected'
					? 'REVIEW'
					: 'SHELL'
	);
	let links = $derived([
		{ href: homeHref, label: 'HOME_VECTOR', icon: 'home' },
		{ href: '/waiting-clearance', label: 'QUEUE_STATE', icon: 'hourglass_top' },
		{ href: '/surveillance', label: 'DATABASE', icon: 'group' },
		{ href: '/rankings', label: 'RANKINGS', icon: 'leaderboard' }
	]);
	let activePath = $derived(app ? appPathFromView(app.view) : page.url.pathname);
</script>

<aside
	class="fixed top-16 left-0 z-40 hidden h-[calc(100vh-64px)] w-80 flex-col space-y-4 border-r border-outline-variant/10 bg-background p-8 shadow-[20px_0_60px_rgba(129,0,143,0.08)] lg:flex"
>
	<div class="mb-10 flex flex-col space-y-2 border-b border-outline-variant/15 pb-6">
		<div class="flex items-center gap-3">
			<div class="flex h-12 w-12 items-center justify-center bg-surface-container-high">
				<span class="material-symbols-outlined text-3xl text-secondary-container">shield</span>
			</div>
			<div>
				<p class="font-headline font-black tracking-widest text-secondary-container uppercase">
					{codename}
				</p>
				<p class="font-headline text-[10px] text-on-surface-variant uppercase">
					CLEARANCE: {clearance}
				</p>
			</div>
		</div>
		<p class="mt-2 font-label text-[9px] tracking-widest text-secondary-container/60 uppercase">
			ENCRYPTION ACTIVE
		</p>
	</div>
	<nav class="flex flex-col gap-2">
		{#each links as link (link.href)}
			<a
				href={resolve(link.href as Pathname)}
				class="group flex items-center gap-4 p-3 font-headline font-medium uppercase transition-all {activePath ===
					link.href
					? 'bg-surface-container pl-6 text-secondary'
					: 'text-on-surface opacity-70 hover:bg-surface-container-high hover:pl-6'}"
			>
				<span class="material-symbols-outlined">{link.icon}</span>
				<span>{link.label}</span>
			</a>
		{/each}
	</nav>
	<div class="mt-auto border-l-2 border-secondary bg-secondary-container/10 p-4">
		<p class="mb-1 font-headline text-[11px] font-bold text-secondary uppercase">
			Transmission Warning
		</p>
		<p class="text-[10px] leading-relaxed text-on-surface-variant/80">
			All data packets are logged. Misreporting is a terminal offense.
		</p>
	</div>
</aside>
