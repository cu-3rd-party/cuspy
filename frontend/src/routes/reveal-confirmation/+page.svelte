<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
</script>

<TerminalShell
	topBar={{
		title: m.reveal_topbar_title(),
		icon: 'warning',
		subtitle: m.reveal_priority_one_alert()
	}}
>
	<div class="pointer-events-none fixed inset-0 overflow-hidden">
		<div
			class="absolute -top-[10%] -left-[10%] h-[40%] w-[40%] rounded-full bg-error-container/10 blur-[120px]"
		></div>
		<div
			class="absolute top-[20%] right-[-5%] h-[50%] w-[30%] rounded-full bg-secondary-container/10 blur-[100px]"
		></div>
	</div>

	<div class="relative z-10 mx-auto flex max-w-2xl flex-col space-y-12 py-10">
		<header class="space-y-4">
			<div class="flex items-center gap-3 text-error">
				<span class="material-symbols-outlined text-3xl" style="font-variation-settings:'FILL' 1"
					>warning</span
				>
				<span class="font-label text-sm font-bold tracking-[0.3em] uppercase"
					>{m.reveal_priority_one_alert()}</span
				>
			</div>
			<h1
				class="font-headline text-5xl leading-none font-bold tracking-tight uppercase sm:text-7xl"
			>
				{m.reveal_security()} <br />
				<span class="text-error">{m.reveal_breach()}</span>
			</h1>
			<div class="h-1 w-24 bg-error"></div>
		</header>

		<section class="relative overflow-hidden bg-surface-container-low p-1">
			<div
				class="absolute top-0 right-0 p-4 font-label text-[10px] text-outline uppercase opacity-40"
			>
				{m.reveal_reference()}
			</div>
			<div class="border-l-4 border-error bg-surface-container p-6 md:p-10">
				<div class="space-y-6">
					<div class="flex items-start justify-between gap-4">
						<p class="max-w-md font-headline text-xl leading-relaxed md:text-2xl">
							<span class="bg-secondary-container/20 px-1 text-secondary">{m.reveal_agent_x()}</span
							>
							{m.reveal_claims_revealed_you()}
						</p>
						<span class="material-symbols-outlined text-outline-variant">fingerprint</span>
					</div>
					<div class="grid gap-8 border-y border-outline-variant/20 py-6 md:grid-cols-2">
						<div>
							<p class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
								{m.reveal_observation_status()}
							</p>
							<p class="mt-2 text-sm font-medium text-on-surface-variant">
								{m.reveal_observation_question()}
							</p>
						</div>
						<div>
							<p class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
								{m.reveal_physical_engagement()}
							</p>
							<p class="mt-2 text-sm font-medium text-on-surface-variant">
								{m.reveal_physical_question()}
							</p>
						</div>
					</div>
					<div class="flex items-center gap-4 bg-surface-container-highest/50 p-4">
						<span
							class="material-symbols-outlined text-secondary"
							style="font-variation-settings:'FILL' 1">lock_person</span
						>
						<p class="font-label text-[11px] leading-tight text-outline uppercase">
							{m.reveal_immediate_action_prefix()}
							<span class="font-bold text-error">{m.reveal_automatic_network_expulsion()}</span>.
						</p>
					</div>
				</div>
			</div>
		</section>

		<div class="grid gap-4 md:grid-cols-2">
			<button
				class="group press-down flex flex-col bg-error-container p-6 text-left transition-all hover:bg-[#a5000b]"
			>
				<div class="mb-8 flex items-center justify-between">
					<span class="material-symbols-outlined group-hover-grow text-on-error-container"
						>emergency_home</span
					>
					<span class="font-label text-[10px] text-on-error-container/60 uppercase"
						>{m.reveal_code_redact()}</span
					>
				</div>
				<span class="font-headline text-xl font-bold text-on-error-container uppercase"
					>{m.reveal_i_am_compromised()}</span
				>
				<p class="mt-2 font-label text-[10px] text-on-error-container/70 uppercase">
					{m.reveal_submit_and_accept_loss()}
				</p>
			</button>

			<button
				class="group flex flex-col border-t-2 border-transparent bg-surface-container-highest p-6 text-left transition-all hover:bg-outline-variant active:border-secondary"
			>
				<div class="mb-8 flex items-center justify-between">
					<span class="material-symbols-outlined group-hover-tilt text-secondary">gavel</span>
					<span class="font-label text-[10px] text-outline uppercase"
						>{m.reveal_code_contest()}</span
					>
				</div>
				<span class="font-headline text-xl font-bold uppercase">{m.reveal_dispute_claim()}</span>
				<p class="mt-2 font-label text-[10px] text-on-surface-variant uppercase">
					{m.reveal_initiate_mediation()}
				</p>
			</button>
		</div>

		<footer class="pt-12 text-center">
			<div class="flex justify-center gap-8">
				{#each [[m.reveal_latency(), '24MS'], [m.reveal_node(), 'HK_09'], [m.reveal_signal(), m.reveal_stable()]] as [label, value]}
					<div class="opacity-30">
						<div class="font-label text-[10px] uppercase">{label}</div>
						<div class="font-headline text-xs font-bold">{value}</div>
					</div>
				{/each}
			</div>
			<p class="mt-4 font-label text-[9px] tracking-[0.4em] text-outline uppercase">
				{m.reveal_terminal_version()}
			</p>
		</footer>
	</div>

	<div class="fixed top-0 left-0 z-50 h-1 w-full bg-surface-container-highest">
		<div class="live-bar h-full w-1/3 bg-error"></div>
	</div>
</TerminalShell>
