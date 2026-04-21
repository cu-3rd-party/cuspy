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
	class="fixed inset-x-0 bottom-0 z-50 flex h-16 items-stretch justify-around bg-surface-container-lowest shadow-[0_-4px_20px_rgba(0,122,27,0.1)]"
>
	{#each items as item}
		<a
			href={isLocked(item) ? undefined : resolve(item.href)}
			aria-disabled={isLocked(item)}
			class={`flex flex-1 flex-col items-center justify-center gap-1 p-2 text-[10px] font-bold tracking-tight transition-colors ${isActive(item) ? 'bg-primary-container text-black' : isLocked(item) ? 'pointer-events-none cursor-not-allowed text-outline/30 saturate-0' : 'text-outline hover:bg-surface-container-low hover:text-primary'}`}
		>
			<Icon name={item.icon} filled={isActive(item) || item.fill} />
			<span class="font-headline uppercase">{item.label}</span>
		</a>
	{/each}
</nav>
