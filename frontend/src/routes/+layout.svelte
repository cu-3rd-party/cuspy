<script lang="ts">
	import type { Pathname } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import { sessionUser } from '$lib/stores/session';
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { data, children } = $props();

	const pageRef = (locale: "en" | "ru" | undefined) => { // TODO(@pxc1984) 22 Apr 2026: unhardcode the languages
		return localizeHref(page.url.pathname, { locale }) as Pathname;
	}

	onMount(() => {
		sessionUser.set(data.sessionUser ?? null);
	});
</script>

<script lang="ts">
	import type { Pathname } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import { sessionUser } from '$lib/stores/session';
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import TopBar from '$lib/components/TopBar.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import BottomNav from '$lib/components/BottomNav.svelte';
	import { gameplayNav } from '$lib/prototype/data';

	let { data, children } = $props();

	const pageRef = (locale: "en" | "ru" | undefined) => { // TODO(@pxc1984) 22 Apr 2026: unhardcode the languages
		return localizeHref(page.url.pathname, { locale }) as Pathname;
	}

	onMount(() => {
		sessionUser.set(data.sessionUser ?? null);
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<TopBar config={{ title: 'CUKILLER // PROTOCOL', icon: 'terminal' }} />
<Sidebar />
<main class="pt-16 lg:pl-80 min-h-screen">
	{@render children()}
</main>
<BottomNav items={gameplayNav} />

<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(pageRef(locale))}>{locale}</a>
	{/each}
</div>
