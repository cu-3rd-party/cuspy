<script lang="ts">
	import { page } from '$app/state';
	import type { SessionFlow } from '$lib/stores/session';

	let {
		flow = undefined,
		homeHref = '/'
	}: {
		flow?: SessionFlow;
		homeHref?: string;
	} = $props();

	let codename = $derived(
		(flow?.user?.agent_data?.codename as string | undefined) ?? flow?.user?.agent_name ?? 'OPERATIVE_X'
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
</script>

<aside class="hidden lg:flex flex-col h-[calc(100vh-64px)] w-80 bg-background shadow-[20px_0_60px_rgba(129,0,143,0.08)] p-8 space-y-4 fixed left-0 top-16 z-40 border-r border-outline-variant/10">
	<div class="flex flex-col space-y-2 mb-10 pb-6 border-b border-outline-variant/15">
		<div class="flex items-center gap-3">
			<div class="w-12 h-12 bg-surface-container-high flex items-center justify-center">
				<span class="material-symbols-outlined text-secondary-container text-3xl">shield</span>
			</div>
			<div>
				<p class="text-secondary-container font-black tracking-widest font-headline uppercase">{codename}</p>
				<p class="text-[10px] text-on-surface-variant font-headline uppercase">CLEARANCE: {clearance}</p>
			</div>
		</div>
		<p class="text-[9px] tracking-widest text-secondary-container/60 mt-2 uppercase font-label">ENCRYPTION ACTIVE</p>
	</div>
	<nav class="flex flex-col gap-2">
		{#each links as link (link.href)}
			<a href={link.href} class="group flex items-center gap-4 p-3 font-headline uppercase font-medium transition-all {page.url.pathname === link.href ? 'bg-surface-container text-secondary pl-6' : 'text-on-surface opacity-70 hover:bg-surface-container-high hover:pl-6'}">
				<span class="material-symbols-outlined">{link.icon}</span>
				<span>{link.label}</span>
			</a>
		{/each}
	</nav>
	<div class="mt-auto bg-secondary-container/10 p-4 border-l-2 border-secondary">
		<p class="text-[11px] font-headline text-secondary font-bold mb-1 uppercase">Transmission Warning</p>
		<p class="text-[10px] leading-relaxed text-on-surface-variant/80">All data packets are logged. Misreporting is a terminal offense.</p>
	</div>
</aside>
