<script lang="ts">
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { agentAvatar, dossierNav, roster } from '$lib/prototype/data';
</script>

<TerminalShell topBar={{ title: 'GHOST_SIGNAL_V1.0', icon: 'security', meta: 'AGENT_092', avatar: agentAvatar }} nav={dossierNav}>
	<div class="mx-auto max-w-5xl">
		<section class="relative mb-12 border-l-4 border-secondary-container pl-6">
			<div class="absolute inset-x-0 top-0 h-px bg-primary/10"></div>
			<h1 class="font-headline text-4xl font-bold uppercase tracking-tight sm:text-5xl">Global Rankings</h1>
			<div class="mt-2 flex flex-wrap gap-4 font-label text-[10px] uppercase tracking-[0.2em] text-outline">
				<span>SECTOR: OMEGA-7</span>
				<span class="text-primary">ENCRYPTION: LEVEL_4</span>
				<span>LAST_SYNC: 04:22:11</span>
			</div>
		</section>

		<section class="overflow-hidden bg-surface-container-low">
			<div class="grid grid-cols-12 gap-2 bg-surface-container-highest px-6 py-4 font-label text-[10px] uppercase tracking-[0.2em] text-outline">
				<div class="col-span-2">Rank</div>
				<div class="col-span-4 md:col-span-5">Agent Dossier</div>
				<div class="col-span-3 text-right md:col-span-2">Rating</div>
				<div class="col-span-3 text-right">Discoveries</div>
			</div>

			<div class="divide-y divide-outline-variant/10">
				{#each roster as row}
					<div class={`grid grid-cols-12 items-center gap-2 px-6 py-5 transition-colors ${row.active ? 'border-l-2 border-primary bg-primary-container/10' : 'hover:bg-surface-container'}`}>
						<div class={`col-span-2 font-headline text-2xl font-bold ${row.active ? 'text-primary' : 'text-secondary-container'}`}>{row.rank}</div>
						<div class="col-span-4 flex items-center gap-3 md:col-span-5">
							<div class={`relative flex size-10 items-center justify-center ${row.active ? 'bg-primary-container/20' : 'bg-surface-container-highest'}`}>
								<span class={`material-symbols-outlined ${row.active ? 'text-primary' : 'text-outline'}`} style={row.active ? "font-variation-settings:'FILL' 1" : ''}>account_circle</span>
								<div class="absolute bottom-0 right-0 size-2 bg-primary"></div>
							</div>
							<div>
								<span class={`font-headline font-bold ${row.active ? 'text-primary' : 'text-on-surface'}`}>{row.name}</span>
								<div class={`font-label text-[9px] uppercase ${row.active ? 'text-primary/70' : 'text-outline'}`}>{row.syndicate}</div>
							</div>
						</div>
						<div class="col-span-3 text-right font-headline font-medium md:col-span-2">{row.rating}</div>
						<div class="col-span-3 text-right font-headline">{row.discoveries}</div>
					</div>
				{/each}
			</div>

			<div class="flex items-center justify-between border-t border-outline-variant/30 bg-surface-container-lowest p-4">
				<div class="flex items-center gap-2">
					<div class="signal-dot size-2 bg-primary"></div>
					<span class="font-label text-[9px] uppercase tracking-[0.2em] text-outline">Live stream active: 10,442 users online</span>
				</div>
				<button class="font-label text-[10px] uppercase tracking-[0.2em] text-secondary transition-colors hover:text-on-secondary-container">View Full Database &gt;</button>
			</div>
		</section>

		<div class="mt-8 grid gap-4 md:grid-cols-3">
			<div class="bg-surface-container-high p-6">
				<span class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Global Avg Elo</span>
				<div class="mt-2 font-headline text-3xl font-bold">1,240</div>
				<div class="mt-3 h-1 bg-surface-container-highest"><div class="h-full w-[65%] bg-secondary-container"></div></div>
			</div>
			<div class="bg-surface-container-high p-6">
				<span class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Weekly Discoveries</span>
				<div class="mt-2 font-headline text-3xl font-bold text-primary">128,491</div>
				<div class="mt-2 text-[10px] font-medium text-primary/60">+14.2% FROM LAST PERIOD</div>
			</div>
			<div class="bg-secondary-container p-6 text-black">
				<span class="font-label text-[10px] uppercase tracking-[0.2em]">Target Threshold</span>
				<div class="mt-2 font-headline text-3xl font-bold">2,500</div>
				<div class="mt-2 text-[10px] font-bold">NEXT RANK: ELITE GHOST</div>
			</div>
		</div>
	</div>
</TerminalShell>
