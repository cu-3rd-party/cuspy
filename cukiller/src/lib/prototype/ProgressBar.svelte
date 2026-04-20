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
		<span class="font-label text-xs uppercase tracking-[0.2em] text-primary">{label}</span>
		<span class="font-label text-xs uppercase tracking-[0.2em] text-outline">{status}</span>
	</div>
	<div class="segment-bar h-3" style={`grid-template-columns: repeat(${total}, minmax(0, 1fr));`}>
		{#each Array.from({ length: total }, (_, index) => index) as index}
			<span class:active={index < current}></span>
		{/each}
	</div>
</div>
