<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '$lib/paraglide/messages.js';

	let { timeRemaining }: { timeRemaining: Date } = $props();
	let seconds = $derived(timeRemaining.getMinutes() * 60 + timeRemaining.getSeconds());

	onMount(() => {
		const timer = window.setInterval(() => {
			if (seconds <= 0) {
				return;
			}
			seconds -= 1;
		}, 1000);

		return () => window.clearInterval(timer);
	});
</script>

<div class="border-l-2 border-primary bg-surface-container-low p-4" class:finish={seconds === 0}>
	<p class="mb-1 font-label text-[10px] text-outline uppercase">
		{m.agent_id_time_remaining()}
	</p>
	<p class="font-headline font-bold">
		{Math.floor(seconds / 60)
			.toString()
			.padStart(2, '0')}:{(seconds % 60).toString().padStart(2, '0')}
	</p>
</div>

<style>
	@keyframes completion-pulse {
		0%,
		100% {
			opacity: 1;
			box-shadow: 0 0 8px var(--color-on-error);
		}
		50% {
			opacity: 0.82;
			box-shadow: 0 0 18px var(--color-on-error);
		}
	}

	.finish {
		animation: completion-pulse 1s ease-in-out infinite;
	}
</style>
