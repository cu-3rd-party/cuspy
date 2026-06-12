<script lang="ts">
	import Icon from './Icon.svelte';
	import type { TopBarConfig } from '$lib/shared/config';
	import type { SessionFlow } from '$lib/shared/model';

	let { config, flow = undefined }: { config: TopBarConfig; flow?: SessionFlow } = $props();
	let codename = $derived(
		(flow?.user?.agent_data?.codename as string | undefined) ?? flow?.user?.agent_name ?? 'guest'
	);
	let status = $derived(
		flow?.status === 'approved'
			? 'OPERATIVE ACTIVE'
			: flow?.status === 'pending'
				? 'WAITING CLEARANCE'
				: flow?.status === 'rejected'
					? 'REVISION REQUIRED'
					: flow?.status === 'no_profile'
						? 'PROFILE REQUIRED'
						: 'GUEST SESSION'
	);
</script>

<header
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between bg-surface-container-lowest px-6 py-4 text-primary shadow-[0_0_15px_rgba(0,122,27,0.1)]"
>
	<div class="flex items-center gap-3">
		{#if config.backHref}
			<a
				href={config.backHref}
				class="rounded-full p-2 text-outline transition-colors hover:bg-surface-container-low hover:text-primary"
			>
				<Icon name="arrow_back" />
			</a>
		{/if}
		<a href="/" class="group flex items-center gap-3">
			<Icon name="terminal" class="text-xl transition-transform group-hover:scale-110" />
			<h1 class="font-headline text-sm font-bold tracking-[0.2em] uppercase sm:text-base">
				{config.title || 'CUKILLER // PROTOCOL'}
			</h1>
		</a>
	</div>

	<div class="flex items-center gap-6">
		<div class="hidden min-w-0 md:block">
			<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">{status}</div>
			<div class="truncate font-headline text-sm font-bold text-on-surface uppercase">
				{codename}
			</div>
		</div>
		<!--		<div class="flex items-center gap-2 bg-surface-container px-3 py-1 border border-outline-variant/10">-->
		<!--			<Icon name="payments" class="text-sm text-primary" />-->
		<!--			<span class="font-label text-sm font-bold tracking-widest text-primary">45,200G</span>-->
		<!--		</div>-->
		<!--		<button class="p-1 hover:bg-surface-container rounded transition-colors">-->
		<!--			<Icon name="settings" class="text-xl" />-->
		<!--		</button>-->
	</div>
</header>
