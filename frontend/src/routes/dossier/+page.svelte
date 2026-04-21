<script lang="ts">
	import { resolve } from '$app/paths';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { agentAvatar, dossierNav } from '$lib/prototype/data';
</script>

<TerminalShell
	topBar={{
		title: 'GHOST_SIGNAL_V1.0',
		icon: 'security',
		meta: 'ACTIVE_SESSION_092',
		avatar: agentAvatar
	}}
	nav={dossierNav}
>
	<section class="mb-12 border-b border-outline-variant/10 pb-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
			<div>
				<span class="mb-2 block font-headline text-xs tracking-[0.3em] text-primary/60 uppercase"
					>Subject Dossier</span
				>
				<h2 class="font-headline text-5xl font-bold tracking-tight sm:text-7xl">Agent Sokolov</h2>
				<div class="mt-2 flex items-center gap-4">
					<span
						class="bg-primary-container px-3 py-1 font-headline text-xs font-bold tracking-[0.2em] text-on-primary-container"
						>ID: 8829-X</span
					>
					<span class="font-headline text-xs font-bold tracking-[0.2em] text-outline"
						>LOC: [REDACTED]</span
					>
				</div>
			</div>
			<div class="text-right">
				<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">
					Current Authorization
				</div>
				<div class="font-headline text-2xl font-bold tracking-tight text-secondary">
					CLEARANCE LEVEL 4
				</div>
			</div>
		</div>
	</section>

	<div class="grid gap-1 md:grid-cols-12">
		<div
			class="relative flex min-h-80 flex-col justify-between overflow-hidden bg-surface-container-low p-8 md:col-span-8"
		>
			<div class="absolute top-0 right-0 p-4 opacity-10">
				<span class="material-symbols-outlined text-9xl" style="font-variation-settings:'FILL' 1"
					>shield</span
				>
			</div>
			<div class="relative z-10">
				<span class="mb-6 block font-headline text-[10px] tracking-[0.2em] text-outline uppercase"
					>Combat Performance</span
				>
				<div class="mb-8 flex items-center gap-8">
					<div>
						<div class="mb-1 font-headline text-xs tracking-[0.2em] text-on-surface/60 uppercase">
							Current Tier
						</div>
						<div class="font-headline text-4xl font-bold">Silver IV</div>
					</div>
					<div class="h-12 w-px bg-outline-variant/30"></div>
					<div>
						<div class="mb-1 font-headline text-xs tracking-[0.2em] text-on-surface/60 uppercase">
							Elo Rating
						</div>
						<div class="font-headline text-4xl font-bold text-primary">1450</div>
					</div>
				</div>
			</div>
			<div class="relative z-10">
				<div class="mb-4 flex items-end justify-between">
					<span class="font-headline text-[10px] tracking-[0.2em] uppercase"
						>Experience Progress</span
					>
					<span class="font-headline text-xs font-bold text-primary">78% TO GOLD</span>
				</div>
				<div class="segment-bar grid-cols-6">
					<span class="active"></span><span class="active"></span><span class="active"></span><span
						class="active"
					></span><span></span><span></span>
				</div>
			</div>
		</div>

		<div
			class="scan-sweep relative flex min-h-80 flex-col justify-between bg-secondary-container p-8 md:col-span-4"
		>
			<div class="scanline absolute inset-0 opacity-20"></div>
			<div class="relative z-10">
				<span
					class="mb-6 block font-headline text-[10px] tracking-[0.2em] text-on-secondary-container/80 uppercase"
					>System Status</span
				>
				<div class="space-y-6">
					<div class="flex gap-4">
						<span class="material-symbols-outlined text-2xl text-on-secondary-container"
							>target</span
						>
						<div>
							<div
								class="font-headline text-xs font-bold tracking-[0.2em] text-on-secondary-container uppercase"
							>
								HUNTING
							</div>
							<div class="text-sm text-on-secondary-container/90">Target: "The Broker"</div>
						</div>
					</div>
					<div class="flex gap-4">
						<span class="material-symbols-outlined text-2xl text-error">warning</span>
						<div>
							<div class="font-headline text-xs font-bold tracking-[0.2em] text-error uppercase">
								UNDER PURSUIT
							</div>
							<div class="text-sm text-error/90">Threat: Unknown Agent</div>
						</div>
					</div>
				</div>
			</div>
			<a
				href={resolve('/target-intel')}
				class="press-scale relative z-10 border border-secondary/30 bg-black px-4 py-4 text-center font-headline text-xs font-bold tracking-[0.2em] text-secondary uppercase transition-colors hover:bg-secondary hover:text-black"
			>
				View Target Intel
			</a>
		</div>

		<div class="grid gap-1 md:col-span-12 md:grid-cols-3">
			<div class="border-l-4 border-primary bg-surface-container p-6">
				<div class="mb-3 font-headline text-[10px] tracking-[0.2em] text-primary uppercase">
					Recent Activity
				</div>
				<p class="text-sm leading-relaxed text-on-surface/80">
					Intelligence intercepted in Sector 7G. Surveillance logs uploaded to cloud relay.
					Encryption sequence verified.
				</p>
			</div>
			<div class="border-l-4 border-secondary bg-surface-container p-6">
				<div class="mb-3 font-headline text-[10px] tracking-[0.2em] text-secondary uppercase">
					Priority Objectives
				</div>
				<ul class="space-y-2 text-sm text-on-surface/80">
					<li class="flex items-center gap-2">
						<span class="size-1.5 bg-secondary"></span>Infiltrate Tokyo Hub
					</li>
					<li class="flex items-center gap-2 opacity-50">
						<span class="size-1.5 bg-outline"></span>Recover Hard Drive (Complete)
					</li>
				</ul>
			</div>
			<div
				class="scanning-grid relative flex items-center justify-center overflow-hidden bg-surface-container-high p-6 text-center"
			>
				<div class="relative z-10">
					<span class="material-symbols-outlined mb-2 text-3xl text-outline">add_circle</span>
					<div class="font-headline text-[10px] font-bold tracking-[0.2em] text-outline uppercase">
						New Mission Available
					</div>
				</div>
			</div>
		</div>
	</div>
</TerminalShell>
