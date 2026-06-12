<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import { AppProvider } from '$lib/shared/providers';
	import './layout.css';

	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>
<AppProvider initialSessionFlow={data.sessionFlow} initialSessionUser={data.sessionUser}>
	<main class="min-h-screen">
		{@render children()}
	</main>
</AppProvider>
