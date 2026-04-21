<script lang="ts">
	import { resolve } from '$app/paths';
	import ProgressBar from '$lib/prototype/ProgressBar.svelte';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { enlistNav } from '$lib/prototype/data';
</script>

<TerminalShell topBar={{ title: 'CLASSIFIED_REGISTRATION', icon: 'terminal' }} nav={enlistNav}>
	<div class="mx-auto max-w-xl">
		<ProgressBar current={1} total={2} label="Status: Initializing Link" status="Phase 01 / 02" />

		<section class="scan-sweep relative mt-12 overflow-hidden bg-surface-container">
			<div class="bg-surface-container-low p-6">
				<div class="flex items-center justify-between">
					<div>
						<h2 class="font-headline text-2xl font-bold uppercase">AGENT_ENROLLMENT</h2>
						<p class="mt-1 font-label text-[10px] tracking-[0.3em] text-outline uppercase">
							Clearance Level: Delta-6
						</p>
					</div>
					<span
						class="material-symbols-outlined text-3xl text-secondary"
						style="font-variation-settings:'FILL' 1">security</span
					>
				</div>
			</div>

			<form class="space-y-10 p-8" action={resolve('/operational-boundaries')}>
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
							Agent Name (Codename)
						</p>
						<span class="font-label text-[10px] text-primary/40 uppercase">Required</span>
					</div>
					<input
						class="w-full border-0 border-b-2 border-outline-variant bg-transparent px-0 py-3 font-label text-lg tracking-[0.2em] text-primary transition-all placeholder:text-outline/30 focus:border-primary focus:ring-0"
						placeholder="e.g. NEON_FOX"
					/>
				</div>

				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
							Academic Group / Sector
						</p>
						<span class="font-label text-[10px] text-primary/40 uppercase">Authentication Link</span
						>
					</div>
					<input
						class="w-full border-0 border-b-2 border-outline-variant bg-transparent px-0 py-3 font-label text-lg tracking-[0.2em] text-primary transition-all placeholder:text-outline/30 focus:border-primary focus:ring-0"
						placeholder="SIGMA-091"
					/>
				</div>

				<div class="space-y-4">
					<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
						Upload Identification (Biometric Scan)
					</p>
					<div
						class="group relative flex aspect-video cursor-pointer flex-col items-center justify-center border-2 border-dashed border-outline-variant bg-surface-container-high transition-colors hover:bg-surface-container-highest"
					>
						<span
							class="material-symbols-outlined mb-2 text-4xl text-outline transition-colors group-hover:text-primary"
							>add_a_photo</span
						>
						<p
							class="font-label text-[10px] tracking-tight text-outline uppercase transition-colors group-hover:text-on-surface"
						>
							Drop visual data or click to browse
						</p>
						<input type="file" class="absolute inset-0 opacity-0" />
					</div>
				</div>

				<button
					class="glitch-burst tactical-button flex w-full items-center justify-center gap-3 px-6 py-5 font-headline font-bold tracking-[0.3em] uppercase transition-[filter,transform,brightness] hover:brightness-110 active:scale-[0.98]"
					type="submit"
				>
					Proceed
					<span class="material-symbols-outlined">arrow_forward</span>
				</button>
			</form>

			<div class="bg-surface-container-lowest p-4">
				<p
					class="flex items-start gap-4 font-label text-[9px] leading-relaxed text-outline uppercase"
				>
					<span class="signal-dot mt-1 size-2 bg-primary"></span>
					<span
						>Any falsification of agent credentials will result in immediate extraction from the
						protocol.</span
					>
				</p>
			</div>
		</section>

		<div class="mt-10 grid gap-4 sm:grid-cols-2">
			<div class="border-l-2 border-secondary bg-surface-container-low p-4">
				<p class="mb-1 font-label text-[10px] text-outline uppercase">Node Connectivity</p>
				<p class="font-headline font-bold">STABLE_ENCRYPTED</p>
			</div>
			<div class="border-l-2 border-primary bg-surface-container-low p-4">
				<p class="mb-1 font-label text-[10px] text-outline uppercase">Time Remaining</p>
				<p class="font-headline font-bold">14:59:59</p>
			</div>
		</div>
	</div>
</TerminalShell>
