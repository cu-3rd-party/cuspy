<script lang="ts">
	import { resolveAgentImage } from '$lib/shared/api';
	import type { AgentProfileData } from '$lib/shared/model';

	let {
		profile,
		name,
		class: className = '',
		size = 32
	}: {
		profile: Pick<AgentProfileData, 'identificationImage' | 'identificationImageResourceId'>;
		name: string;
		class?: string;
		size?: number;
	} = $props();

	let src = $state<string | undefined>();

	$effect(() => {
		src = undefined;
		resolveAgentImage(profile).then((url) => {
			src = url;
		});
	});
</script>

{#if src}
	<img
		{src}
		alt=""
		class="shrink-0 rounded-full object-cover {className}"
		style="width: {size}px; height: {size}px;"
	/>
{:else}
	<div
		class="flex shrink-0 items-center justify-center rounded-full bg-surface-container-high font-label text-[10px] text-outline {className}"
		style="width: {size}px; height: {size}px;"
	>
		{name.charAt(0).toUpperCase()}
	</div>
{/if}
