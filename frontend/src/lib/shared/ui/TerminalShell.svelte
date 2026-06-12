<script lang="ts">
	import BottomNav from './BottomNav.svelte';
	import TopBar from './TopBar.svelte';
	import type { BottomNavItem, TopBarConfig } from '$lib/shared/config';
	import type {SessionFlow} from "$lib/shared/model";

	let { topBar, flow = undefined, nav, children } = $props<{
		topBar: TopBarConfig;
		flow?: SessionFlow;
		nav?: BottomNavItem[];
		children: () => unknown;
	}>();
</script>

<div class="min-h-screen bg-background text-on-surface">
	<TopBar config={topBar} flow={flow} />
	<main class={`mx-auto w-full max-w-6xl px-4 pt-16 ${nav ? 'pb-24' : 'pb-10'} sm:px-6`}>
		{@render children()}
	</main>
	{#if nav}
		<BottomNav items={nav} />
	{/if}
</div>
