<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';

	type AgentIdState = {
		codename: string;
		identificationName: string;
		identificationImage: string;
	};

	let {
		agentId,
		uploadError = false,
		handleIdentificationChange
	}: {
		agentId: AgentIdState;
		uploadError?: boolean;
		handleIdentificationChange: (event: Event) => Promise<void>;
	} = $props();
</script>

<div class="space-y-2">
	<div class="flex items-center justify-between">
		<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
			{m.agent_id_codename_label()}
		</p>
		<span class="font-label text-[10px] text-primary/40 uppercase">{m.common_required()}</span>
	</div>
	<input
		class="w-full border-0 border-b-2 border-outline-variant bg-transparent px-0 py-3 font-label text-lg tracking-[0.2em] text-primary transition-all placeholder:text-outline/30 focus:border-primary focus:ring-0"
		placeholder={m.agent_id_codename_placeholder()}
		bind:value={agentId.codename}
	/>
</div>

<div class="space-y-4">
	<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
		{m.agent_id_upload_label()}
	</p>
	<div
		class={`group relative flex aspect-[4/3] w-full cursor-pointer flex-col items-center justify-center overflow-hidden border-2 border-dashed bg-surface-container-high transition-colors hover:bg-surface-container-highest ${uploadError ? 'border-error' : 'border-outline-variant'}`}
	>
		{#if agentId.identificationImage}
			<img
				src={agentId.identificationImage}
				alt={agentId.identificationName}
				class="absolute inset-0 size-full min-w-full object-cover object-center"
			/>
			<div class="absolute inset-0 bg-black/25"></div>
		{/if}

		{#if uploadError}
			<div
				class="pointer-events-none absolute inset-3 animate-[spin_3s_linear_infinite] rounded-full border border-error/60"
			></div>
			<div
				class="pointer-events-none absolute inset-6 animate-[spin_1.8s_linear_infinite] rounded-full border-2 border-transparent border-t-error"
			></div>
		{/if}

		<span
			class={`material-symbols-outlined mb-2 text-4xl transition-colors group-hover:text-primary ${agentId.identificationImage ? 'relative z-10 text-white' : 'text-outline'}`}
			>add_a_photo</span
		>
		<p
			class={`font-label text-[10px] tracking-tight uppercase transition-colors group-hover:text-on-surface ${agentId.identificationImage ? 'relative z-10 text-white' : 'text-outline'}`}
		>
			{m.agent_id_upload_hint()}
		</p>
		<input type="file" class="absolute inset-0 opacity-0" onchange={handleIdentificationChange} />
	</div>
	{#if agentId.identificationName}
		<p class="font-label text-[10px] text-primary uppercase">
			{agentId.identificationName}
		</p>
	{/if}
</div>
