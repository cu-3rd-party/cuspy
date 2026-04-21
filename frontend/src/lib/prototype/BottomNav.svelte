<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import Icon from '$lib/prototype/Icon.svelte';
	import type { BottomNavItem } from '$lib/prototype/data';

	let { items } = $props<{ items: BottomNavItem[] }>();

	const isActive = (item: BottomNavItem) => {
		const current = page.url.pathname;
		return item.match === 'prefix' ? current.startsWith(item.href) : current === item.href;
	};
</script>

<nav
	class="fixed inset-x-0 bottom-0 z-50 flex h-16 items-stretch justify-around bg-surface-container-lowest shadow-[0_-4px_20px_rgba(0,122,27,0.1)]"
>
	{#each items as item}
		<a
			href={resolve(item.href)}
			class={`flex flex-1 flex-col items-center justify-center gap-1 p-2 text-[10px] font-bold tracking-tight transition-colors ${isActive(item) ? 'bg-primary-container text-black' : 'text-outline hover:bg-surface-container-low hover:text-primary'}`}
		>
			<Icon name={item.icon} filled={isActive(item) || item.fill} />
			<span class="font-headline uppercase">{item.label}</span>
		</a>
	{/each}
</nav>
