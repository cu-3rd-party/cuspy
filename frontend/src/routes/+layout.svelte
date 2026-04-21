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

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}

<div style="display:none">
	{#each locales as locale (locale)}
		<a href={resolve(pageRef(locale))}>{locale}</a>
	{/each}
</div>
