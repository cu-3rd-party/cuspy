<script lang="ts">
	let { current, total, label, status } = $props<{
		current: number;
		total: number;
		label: string;
		status: string;
	}>();
</script>

<div class="w-full">
	<div class="mb-2 flex items-end justify-between gap-4">
		<span class="font-label text-xs tracking-[0.2em] text-primary uppercase">{label}</span>
		<span class="font-label text-xs tracking-[0.2em] text-outline uppercase">{status}</span>
	</div>
	<div class="segment-bar h-3" style={`grid-template-columns: repeat(${total}, minmax(0, 1fr));`}>
		{#each Array.from({ length: total }, (_, index) => index) as index}
			<span class:active={index < current} class:pulsing-element={index === current - 1}></span>
		{/each}
	</div>
</div>

<style>
	@keyframes completion-pulse {
		0%,
		100% {
			opacity: 1;
			box-shadow: 0 0 8px var(--color-primary-fixed);
		}
		50% {
			opacity: 0.82;
			box-shadow: 0 0 18px var(--color-primary-container);
		}
	}

	.pulsing-element {
		animation: completion-pulse 1.5s ease-in-out infinite;
	}
</style>
