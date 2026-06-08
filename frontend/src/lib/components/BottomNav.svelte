<script lang="ts">
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { loadDossierDraft } from '$lib/prototype/dossierDraft';
	import Icon from '$lib/components/Icon.svelte';
	import type { BottomNavItem } from '$lib/prototype/data';

	let { items } = $props<{ items: BottomNavItem[] }>();
	let unlockedStep = $state<1 | 2 | 3>(1);

	const isActive = (item: BottomNavItem) => {
		const current = page.url.pathname;
		return item.match === 'prefix' ? current.startsWith(item.href) : current === item.href;
	};

	const readUnlockedStep = () => {
		unlockedStep = loadDossierDraft().unlockedStep;
	};

	const getRequiredStep = (item: BottomNavItem): 1 | 2 | 3 => {
		if (item.href === '/operational-boundaries') {
			return 2;
		}

		if (item.href === '/dossier-verification') {
			return 3;
		}

		return 1;
	};

	const isLocked = (item: BottomNavItem) =>
		item.group === 'enlist' && unlockedStep < getRequiredStep(item);

	onMount(() => {
		if (!browser) {
			return;
		}

		readUnlockedStep();

		const handleStorage = (event: StorageEvent) => {
			if (event.key === 'dossier-draft') {
				readUnlockedStep();
			}
		};

		window.addEventListener('storage', handleStorage);

		return () => window.removeEventListener('storage', handleStorage);
	});

	$effect(() => {
		page.url.pathname;
		if (browser) {
			readUnlockedStep();
		}
	});
</script>

<nav
	class="fixed inset-x-0 bottom-0 z-50 flex h-20 items-stretch justify-around bg-surface-container-lowest border-t border-outline-variant/15 shadow-[0_-4px_20px_rgba(0,122,27,0.05)]"
>
	{#each items as item}
		<a
			href={isLocked(item) ? undefined : resolve(item.href)}
			aria-disabled={isLocked(item)}
			class={isActive(item) ? 'text-emerald-400 flex flex-col items-center justify-center p-2 transition-all active:scale-95 duration-75 w-full' : isLocked(item) ? ' w-full flex flex-col items-center justify-center p-2 transition-all text-outline pointer-events-none cursor-not-allowed saturate-0' : ' w-full text-emerald-100 flex flex-col items-center justify-center p-2 transition-all active:scale-95 duration-75'}
		>
			<Icon name={item.icon} filled={isActive(item) || item.fill} class="text-xl" />
			<span class="font-headline text-[10px] font-bold tracking-widest uppercase mt-1">{item.label}</span>
		</a>
	{/each}
</nav>
