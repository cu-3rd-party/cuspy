<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import Icon from '$lib/components/Icon.svelte';
	import type { TopBarConfig } from '$lib/prototype/data';

	let { config } = $props<{ config: TopBarConfig }>();
</script>

<header
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between bg-surface-container-lowest px-6 py-4 text-primary"
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
		<Icon name={config.icon} class="text-xl" />
		<div>
			<h1 class="font-headline text-sm font-bold tracking-[0.25em] uppercase sm:text-base">
				{config.title}
			</h1>
			{#if config.subtitle}
				<p class="mt-1 font-label text-[10px] tracking-[0.3em] text-outline uppercase">
					{config.subtitle}
				</p>
			{/if}
		</div>
	</div>

	<div class="flex items-center gap-4">
		{#if config.meta}
			<div class="hidden text-right sm:block">
				<p class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">{config.meta}</p>
			</div>
		{/if}
		{#if config.avatar}
			<div class="size-10 overflow-hidden bg-surface-container-highest">
				<img
					src={config.avatar}
					alt={m.top_bar_agent_avatar_alt()}
					class="size-full object-cover contrast-125 grayscale"
				/>
			</div>
		{/if}
	</div>
</header>
