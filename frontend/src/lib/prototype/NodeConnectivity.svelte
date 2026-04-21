<script lang="ts">
	import type { Snippet } from 'svelte';
	import { m } from '$lib/paraglide/messages.js';

	let { children }: { children: Snippet } = $props();
</script>

<div class="border-l-2 border-secondary bg-surface-container-low p-4">
	<p class="mb-1 font-label text-[10px] text-outline uppercase">
		{m.agent_id_node_connectivity()}
	</p>
	<!--suppress JSValidateTypes -->
	<p class="font-headline font-bold">
		{@render children()}
	</p>
</div>
