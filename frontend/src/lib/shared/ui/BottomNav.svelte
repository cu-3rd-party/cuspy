<script lang="ts">
	import { browser } from '$app/environment';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import Icon from './Icon.svelte';
	import type { BottomNavItem } from '$lib/shared/config';

	const DOSSIER_DRAFT_STORAGE_KEY = 'dossier-draft';
	type StoredDossierDraft = { unlockedStep?: 1 | 2 | 3 };

	let { items } = $props<{ items: BottomNavItem[] }>();
	let unlockedStep = $state<1 | 2 | 3>(1);

	const isActive = (item: BottomNavItem) => {
		const current = page.url.pathname;
		return item.match === 'prefix' ? current.startsWith(item.href) : current === item.href;
	};

	const readUnlockedStep = () => {
		try {
			const stored = window.localStorage.getItem(DOSSIER_DRAFT_STORAGE_KEY);
			const parsed = stored ? (JSON.parse(stored) as StoredDossierDraft) : null;

			unlockedStep =
				parsed?.unlockedStep === 2 || parsed?.unlockedStep === 3 ? parsed.unlockedStep : 1;
		} catch {
			unlockedStep = 1;
		}
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

	afterNavigate(() => {
		if (browser) {
			readUnlockedStep();
		}
	});

	onMount(() => {
		if (!browser) {
			return;
		}

		readUnlockedStep();

		const handleStorage = (event: StorageEvent) => {
			if (event.key === DOSSIER_DRAFT_STORAGE_KEY) {
				readUnlockedStep();
			}
		};

		window.addEventListener('storage', handleStorage);

		return () => window.removeEventListener('storage', handleStorage);
	});
</script>

<nav
	class="fixed inset-x-0 bottom-0 z-50 flex h-20 items-stretch justify-around border-t border-outline-variant/15 bg-surface-container-lowest shadow-[0_-4px_20px_rgba(0,122,27,0.05)]"
>
	{#each items as item (item.href)}
		<a
			href={isLocked(item) ? undefined : resolve(item.href)}
			aria-disabled={isLocked(item)}
			class={isActive(item)
				? 'flex w-full flex-col items-center justify-center p-2 text-emerald-400 transition-all duration-75 active:scale-95'
				: isLocked(item)
					? ' pointer-events-none flex w-full cursor-not-allowed flex-col items-center justify-center p-2 text-outline saturate-0 transition-all'
					: ' flex w-full flex-col items-center justify-center p-2 text-emerald-100 transition-all duration-75 active:scale-95'}
		>
			<Icon name={item.icon} filled={isActive(item) || item.fill} class="text-xl" />
			<span class="mt-1 font-headline text-[10px] font-bold tracking-widest uppercase"
				>{item.label}</span
			>
		</a>
	{/each}
</nav>
