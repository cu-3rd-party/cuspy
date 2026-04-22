<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import Icon from '$lib/components/Icon.svelte';
	import type { TopBarConfig } from '$lib/prototype/data';

	let { config } = $props<{ config: TopBarConfig }>();
</script>

<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import Icon from '$lib/components/Icon.svelte';
	import type { TopBarConfig } from '$lib/prototype/data';

	let { config } = $props<{ config: TopBarConfig }>();
</script>

<header
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between bg-surface-container-lowest px-6 py-4 text-primary shadow-[0_0_15px_rgba(0,122,27,0.1)]"
>
	<div class="flex items-center gap-3">
		{#if config.backHref}
			<a
				href={resolve(config.backHref)}
				class="rounded-full p-2 text-outline transition-colors hover:bg-surface-container-low hover:text-primary"
			>
				<Icon name="arrow_back" />
			</a>
		{/if}
		<a href="/" class="flex items-center gap-3 group">
			<Icon name="terminal" class="text-xl transition-transform group-hover:scale-110" />
			<h1 class="font-headline text-sm font-bold tracking-[0.2em] uppercase sm:text-base">
				{config.title || 'CUKILLER // PROTOCOL'}
			</h1>
		</a>
	</div>

	<div class="flex items-center gap-6">
		<nav class="hidden md:flex gap-8 text-[12px] font-bold tracking-[0.1em] uppercase">
			<a href="/admin/moderation" class="text-primary hover:bg-surface-container transition-colors p-1">ENCRYPTED_COMMS</a>
			<a href="/admin/events" class="text-on-surface hover:bg-surface-container transition-colors p-1">NETWORK_STATUS</a>
		</nav>
		<div class="flex items-center gap-2 bg-surface-container px-3 py-1 border border-outline-variant/10">
			<Icon name="payments" class="text-sm text-primary" />
			<span class="font-label text-sm font-bold tracking-widest text-primary">45,200G</span>
		</div>
		<button class="p-1 hover:bg-surface-container rounded transition-colors">
			<Icon name="settings" class="text-xl" />
		</button>
	</div>
</header>
