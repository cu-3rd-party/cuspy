<script lang="ts">
	import { resolve } from '$app/paths';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { enlistNav, verificationImage } from '$lib/prototype/data';

	const boundaries = [
		['Physical Contact', 'AUTHORIZED', 'primary'],
		['Hugs & Close Proximity', 'RESTRICTED', 'secondary'],
		['Combat Simulation', 'AUTHORIZED', 'primary'],
		['Verbal Interrogation', 'AUTHORIZED', 'primary']
	] as const;
</script>

<TerminalShell
	topBar={{ title: 'VERIFY_DOSSIER', icon: 'security', backHref: '/agent-id' }}
	nav={enlistNav}
>
	<div class="mx-auto max-w-4xl">
		<section class="mb-8 border-l-4 border-primary bg-surface-container-lowest p-4">
			<div class="mb-2 flex justify-between">
				<span class="font-headline text-[10px] font-bold tracking-[0.3em] text-primary uppercase"
					>CLASSIFIED_INTEL</span
				>
				<span
					class="font-headline text-[10px] font-bold tracking-[0.2em] text-on-surface-variant uppercase"
					>TS-S/SCI-O-91</span
				>
			</div>
			<div class="grid gap-4 sm:grid-cols-2">
				<div>
					<p class="font-label text-[10px] text-outline uppercase">Timestamp</p>
					<p class="font-headline text-sm font-bold">2024-05-14 // 04:12:09_UTC</p>
				</div>
				<div class="text-right">
					<p class="font-label text-[10px] text-outline uppercase">Agent ID</p>
					<p class="font-headline text-sm font-bold text-secondary">AG-004-FOX-SIGMA</p>
				</div>
			</div>
		</section>

		<div class="mb-8 grid gap-4 md:grid-cols-3">
			<div class="relative overflow-hidden bg-surface-container md:col-span-1">
				<img
					src={verificationImage}
					alt="Biometric face scan"
					class="hover-unmask h-64 w-full object-cover opacity-60 grayscale md:h-full"
				/>
				<div
					class="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent"
				></div>
				<div class="absolute bottom-2 left-2 flex items-center gap-2 text-primary">
					<span class="material-symbols-outlined text-xs">smart_card_reader</span>
					<span class="font-label text-[10px]">BIO_LINK_ACTIVE</span>
				</div>
				<div class="scan-border-pulse absolute inset-0 border-t border-primary/20"></div>
			</div>

			<div class="space-y-4 md:col-span-2">
				<div class="bg-surface-container p-6">
					<p class="mb-1 font-label text-[10px] text-outline uppercase">Assigned Codename</p>
					<h2 class="font-headline text-4xl font-extrabold tracking-tight">NEON_FOX</h2>
				</div>
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="bg-surface-container p-6">
						<p class="mb-1 font-label text-[10px] text-outline uppercase">Academic Group</p>
						<h3 class="font-headline text-xl font-bold text-secondary">SIGMA-091</h3>
					</div>
					<div class="flex items-center justify-center bg-surface-container p-6 text-center">
						<div>
							<span
								class="material-symbols-outlined mb-2 text-3xl text-primary"
								style="font-variation-settings:'FILL' 1">shield_with_heart</span
							>
							<p class="font-label text-[10px] text-outline uppercase">Clearance: Level 4</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<section>
			<h3
				class="mb-4 flex items-center gap-2 font-headline text-sm font-bold tracking-[0.2em] text-primary"
			>
				<span class="material-symbols-outlined text-sm">settings_input_component</span>
				OPERATIONAL_BOUNDARIES
			</h3>
			<div class="grid gap-px bg-outline-variant/20 md:grid-cols-2">
				{#each boundaries as [label, status, tone]}
					<div class="flex items-center justify-between bg-surface-container p-4">
						<span class="text-sm">{label}</span>
						<span
							class={`px-3 py-1 font-label text-[10px] font-bold ${tone === 'primary' ? 'bg-primary-container/20 text-primary' : 'bg-secondary-container/20 text-secondary'}`}
							>{status}</span
						>
					</div>
				{/each}
			</div>
		</section>

		<footer
			class="mt-12 mb-24 border border-dashed border-error/30 bg-surface-container-low p-6 text-center"
		>
			<p class="mb-2 font-label text-[10px] font-bold tracking-[0.3em] text-error uppercase">
				Legal Notice / Article 4.C
			</p>
			<p class="mx-auto max-w-lg text-xs leading-relaxed text-on-surface-variant">
				All data entered into the dossier system is subject to immediate verification. Unauthorized
				access or deceptive entries will result in immediate network expulsion and asset
				termination.
			</p>
		</footer>

		<div
			class="fixed inset-x-0 bottom-15 bg-gradient-to-t from-background via-background to-transparent px-4 pt-12 pb-4"
		>
			<div class="mx-auto max-w-4xl">
				<a
					href={resolve('/dossier')}
					class="press-shift tactical-button flex h-16 items-center justify-center gap-3 font-headline font-bold tracking-[0.2em] uppercase shadow-[0_0_20px_rgba(0,122,27,0.4)] transition-transform"
				>
					SUBMIT FOR ADMIN APPROVAL
					<span class="material-symbols-outlined">arrow_forward</span>
				</a>
			</div>
		</div>
	</div>
</TerminalShell>
