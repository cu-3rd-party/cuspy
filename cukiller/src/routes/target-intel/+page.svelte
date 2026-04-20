<script lang="ts">
	import { resolve } from '$app/paths';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { agentAvatar, dossierNav, targetImage } from '$lib/prototype/data';
</script>

<TerminalShell topBar={{ title: 'GHOST_SIGNAL_V1.0', icon: 'security', avatar: agentAvatar }} nav={dossierNav}>
	<div class="fixed inset-0 pointer-events-none opacity-5 scanning-grid"></div>
	<div class="relative mx-auto max-w-2xl">
		<div class="mb-10">
			<span class="font-label text-[10px] uppercase tracking-[0.3em] text-primary">Status: In Progress</span>
			<h2 class="mt-1 font-headline text-4xl font-bold uppercase tracking-tight">Current Objective</h2>
			<div class="mt-2 h-1 w-12 bg-primary"></div>
		</div>

		<div class="scan-sweep relative mb-8 overflow-hidden border-l-4 border-primary-container bg-surface-container-low">
			<div class="absolute inset-x-0 top-0 h-0.5 bg-primary/30"></div>
			<div class="flex flex-col md:flex-row">
				<div class="relative h-64 w-full overflow-hidden bg-surface-container-highest md:h-auto md:w-48">
					<img src={targetImage} alt="Target portrait" class="size-full object-cover grayscale contrast-125" />
					<div class="absolute inset-0 bg-primary-container/20 mix-blend-overlay"></div>
					<div class="absolute bottom-2 left-2 flex items-center gap-2 bg-black/80 px-2 py-1">
						<span class="signal-dot size-1.5 rounded-full bg-error"></span>
						<span class="font-label text-[10px] uppercase tracking-[0.2em] text-white">Target_Live</span>
					</div>
				</div>
				<div class="flex-1 p-6">
					<div class="grid gap-6">
						<div>
							<span class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Subject Identifier</span>
							<div class="font-headline text-2xl font-bold uppercase tracking-tight">Agent Raven</div>
						</div>
						<div class="grid gap-4 sm:grid-cols-2">
							<div>
								<span class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Operational Group</span>
								<div class="text-sm font-medium text-primary">B21-DS-01</div>
							</div>
							<div>
								<span class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Last Sighting</span>
								<div class="text-sm font-medium">Campus Central</div>
							</div>
						</div>
						<div class="border-t border-outline-variant/30 pt-4 font-label text-[10px] uppercase text-outline">
							<div class="mb-2 flex items-center gap-2"><span class="material-symbols-outlined text-[14px] text-primary">fingerprint</span>Biometric Match: 98.4%</div>
							<div class="flex items-center gap-2"><span class="material-symbols-outlined text-[14px] text-primary">distance</span>Proximity: 1.2km</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="space-y-6">
			<a href={resolve('/reveal-confirmation')} class="press-down group relative flex items-center justify-between bg-primary-container px-8 py-6 font-headline text-xl font-extrabold uppercase tracking-[0.2em] text-black transition-all hover:bg-primary">
				<span>REPORT DISCOVERY</span>
				<span class="material-symbols-outlined group-active-slide text-3xl">target</span>
			</a>

			<div class="bg-surface-container p-4">
				<div class="flex gap-3">
					<span class="material-symbols-outlined text-xl text-error">warning</span>
					<div>
						<p class="font-label text-[11px] font-bold uppercase text-error">Operational Protocol: No Witnesses</p>
						<p class="mt-1 text-xs leading-relaxed text-on-surface-variant">Engagement must remain covert. Do not expose your presence to civilian assets or non-participating entities.</p>
					</div>
				</div>
				<div class="mt-3 border-t border-outline-variant/20 pt-3">
					<div class="flex gap-3">
						<span class="material-symbols-outlined text-xl text-outline">gavel</span>
						<div>
							<p class="font-label text-[11px] font-bold uppercase">Code of Conduct: Respect Boundaries</p>
							<p class="mt-1 text-xs leading-relaxed text-on-surface-variant">Operation is strictly confined to designated sector limits. Physical contact results in immediate extraction and de-ranking.</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="mt-12 space-y-2 font-label text-[9px] uppercase tracking-[0.2em] text-on-surface/40">
			<div class="flex justify-between"><span>&gt; system_handshake_established</span><span>[ok]</span></div>
			<div class="flex justify-between"><span>&gt; scanning_encryption_layers...</span><span>[complete]</span></div>
			<div class="flex justify-between"><span>&gt; location_triangulated_campus_grid_7</span><span>12:44:02</span></div>
		</div>
	</div>
</TerminalShell>
