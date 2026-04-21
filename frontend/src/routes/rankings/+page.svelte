<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { agentAvatar, dossierNav, roster } from '$lib/prototype/data';
</script>

<TerminalShell
	topBar={{ title: m.dossier_topbar_title(), icon: 'security', meta: m.rankings_topbar_meta(), avatar: agentAvatar }}
	nav={dossierNav}
>
	<div class="mx-auto max-w-5xl">
		<section class="relative mb-12 border-l-4 border-secondary-container pl-6">
			<div class="absolute inset-x-0 top-0 h-px bg-primary/10"></div>
			<h1 class="font-headline text-4xl font-bold tracking-tight uppercase sm:text-5xl">
				{m.rankings_global_rankings()}
			</h1>
			<div
				class="mt-2 flex flex-wrap gap-4 font-label text-[10px] tracking-[0.2em] text-outline uppercase"
			>
				<span>{m.rankings_sector_omega_7()}</span>
				<span class="text-primary">{m.rankings_encryption_level_4()}</span>
				<span>{m.rankings_last_sync()}</span>
			</div>
		</section>

		<section class="overflow-hidden bg-surface-container-low">
			<div
				class="grid grid-cols-12 gap-2 bg-surface-container-highest px-6 py-4 font-label text-[10px] tracking-[0.2em] text-outline uppercase"
			>
				<div class="col-span-2">{m.rankings_rank()}</div>
				<div class="col-span-4 md:col-span-5">{m.rankings_agent_dossier()}</div>
				<div class="col-span-3 text-right md:col-span-2">{m.rankings_rating()}</div>
				<div class="col-span-3 text-right">{m.rankings_discoveries()}</div>
			</div>

			<div class="divide-y divide-outline-variant/10">
				{#each roster as row}
					<div
						class={`grid grid-cols-12 items-center gap-2 px-6 py-5 transition-colors ${row.active ? 'border-l-2 border-primary bg-primary-container/10' : 'hover:bg-surface-container'}`}
					>
						<div
							class={`col-span-2 font-headline text-2xl font-bold ${row.active ? 'text-primary' : 'text-secondary-container'}`}
						>
							{row.rank}
						</div>
						<div class="col-span-4 flex items-center gap-3 md:col-span-5">
							<div
								class={`relative flex size-10 items-center justify-center ${row.active ? 'bg-primary-container/20' : 'bg-surface-container-highest'}`}
							>
								<span
									class={`material-symbols-outlined ${row.active ? 'text-primary' : 'text-outline'}`}
									style={row.active ? "font-variation-settings:'FILL' 1" : ''}>account_circle</span
								>
								<div class="absolute right-0 bottom-0 size-2 bg-primary"></div>
							</div>
							<div>
								<span
									class={`font-headline font-bold ${row.active ? 'text-primary' : 'text-on-surface'}`}
									>{row.name}</span
								>
								<div
									class={`font-label text-[9px] uppercase ${row.active ? 'text-primary/70' : 'text-outline'}`}
								>
									{row.active ? m.rankings_active_session() : row.syndicate}
								</div>
							</div>
						</div>
						<div class="col-span-3 text-right font-headline font-medium md:col-span-2">
							{row.rating}
						</div>
						<div class="col-span-3 text-right font-headline">{row.discoveries}</div>
					</div>
				{/each}
			</div>

			<div
				class="flex items-center justify-between border-t border-outline-variant/30 bg-surface-container-lowest p-4"
			>
				<div class="flex items-center gap-2">
					<div class="signal-dot size-2 bg-primary"></div>
					<span class="font-label text-[9px] tracking-[0.2em] text-outline uppercase"
						>{m.rankings_live_stream_active()}</span
					>
				</div>
				<button
					class="font-label text-[10px] tracking-[0.2em] text-secondary uppercase transition-colors hover:text-on-secondary-container"
					>{m.rankings_view_full_database()}</button
				>
			</div>
		</section>

		<div class="mt-8 grid gap-4 md:grid-cols-3">
			<div class="bg-surface-container-high p-6">
				<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase"
					>{m.rankings_global_avg_elo()}</span
				>
				<div class="mt-2 font-headline text-3xl font-bold">1,240</div>
				<div class="mt-3 h-1 bg-surface-container-highest">
					<div class="h-full w-[65%] bg-secondary-container"></div>
				</div>
			</div>
			<div class="bg-surface-container-high p-6">
				<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase"
					>{m.rankings_weekly_discoveries()}</span
				>
				<div class="mt-2 font-headline text-3xl font-bold text-primary">128,491</div>
				<div class="mt-2 text-[10px] font-medium text-primary/60">{m.rankings_from_last_period()}</div>
			</div>
			<div class="bg-secondary-container p-6 text-black">
				<span class="font-label text-[10px] tracking-[0.2em] uppercase">{m.rankings_target_threshold()}</span>
				<div class="mt-2 font-headline text-3xl font-bold">2,500</div>
				<div class="mt-2 text-[10px] font-bold">{m.rankings_next_rank()}</div>
			</div>
		</div>
	</div>
</TerminalShell>
