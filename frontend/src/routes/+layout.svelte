<script lang="ts">
	import type { Pathname } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { locales, localizeHref } from '$lib/paraglide/runtime';
	import BottomNav from '$lib/components/BottomNav.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import TopBar from '$lib/components/TopBar.svelte';
	import favicon from '$lib/assets/favicon.svg';
	import { profileFlowTarget } from '$lib/profile-flow';
	import { gameplayNav, dossierNav, enlistNav } from '$lib/prototype/data';
	import { sessionUser, canAccessGameplay } from '$lib/stores/session';
	import './layout.css';

	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();

	const sessionFlow = $derived(data.sessionFlow);
	const navConfig = $derived.by(() => {
		if (!sessionFlow?.user) return enlistNav;
		if (canAccessGameplay(sessionFlow)) return gameplayNav;
		return dossierNav;
	});
	const chromeTarget = $derived(sessionFlow?.user ? profileFlowTarget(sessionFlow) : '/');

	const pageRef = (locale: 'en' | 'ru' | undefined) => {
		return localizeHref(page.url.pathname, { locale }) as Pathname;
	};

	$effect(() => {
		sessionUser.set(data.sessionUser ?? null);
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<TopBar config={{ title: 'CUKILLER // PROTOCOL', icon: 'terminal' }} flow={sessionFlow} />
<Sidebar flow={sessionFlow} homeHref={chromeTarget} />

<main class="min-h-screen pt-16 lg:pl-80">
	{@render children()}
</main>

<BottomNav items={navConfig} />

<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(pageRef(locale))}>{locale}</a>
	{/each}
</div>
