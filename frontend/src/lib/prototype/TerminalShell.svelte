<script lang="ts">
	import BottomNav from '$lib/prototype/BottomNav.svelte';
	import TopBar from '$lib/prototype/TopBar.svelte';
	import type { BottomNavItem, TopBarConfig } from '$lib/prototype/data';

	let { topBar, nav, children } = $props<{
		topBar: TopBarConfig;
		nav?: BottomNavItem[];
		children: () => unknown;
	}>();
</script>

<div class="min-h-screen bg-background text-on-surface">
	<TopBar config={topBar} />
	<main class={`mx-auto w-full max-w-6xl px-4 pt-24 ${nav ? 'pb-24' : 'pb-10'} sm:px-6`}>
		{@render children()}
	</main>
	{#if nav}
		<BottomNav items={nav} />
	{/if}
</div>
