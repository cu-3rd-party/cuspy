<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
	import { enlistNav, verificationImage } from '$lib/prototype/data';

	const boundaries = [
		[m.dossier_verification_physical_contact(), m.common_authorized(), 'primary'],
		[m.dossier_verification_hugs(), m.common_restricted(), 'secondary'],
		[m.dossier_verification_combat_simulation(), m.common_authorized(), 'primary'],
		[m.dossier_verification_verbal_interrogation(), m.common_authorized(), 'primary']
	] as const;
</script>

<TerminalShell
	topBar={{ title: m.dossier_verification_topbar_title(), icon: 'security', backHref: '/agent-id' }}
	nav={enlistNav}
>
	<div class="mx-auto max-w-4xl">
		<section class="mb-8 border-l-4 border-primary bg-surface-container-lowest p-4">
			<div class="mb-2 flex justify-between">
				<span class="font-headline text-[10px] font-bold tracking-[0.3em] text-primary uppercase"
					>{m.dossier_verification_classified_intel()}</span
				>
				<span
					class="font-headline text-[10px] font-bold tracking-[0.2em] text-on-surface-variant uppercase"
					>TS-S/SCI-O-91</span
				>
			</div>
			<div class="grid gap-4 sm:grid-cols-2">
				<div>
					<p class="font-label text-[10px] text-outline uppercase">{m.common_timestamp()}</p>
					<p class="font-headline text-sm font-bold">2024-05-14 // 04:12:09_UTC</p>
				</div>
				<div class="text-right">
					<p class="font-label text-[10px] text-outline uppercase">{m.dossier_verification_agent_id()}</p>
					<p class="font-headline text-sm font-bold text-secondary">AG-004-FOX-SIGMA</p>
				</div>
			</div>
		</section>

		<div class="mb-8 grid gap-4 md:grid-cols-3">
			<div class="relative overflow-hidden bg-surface-container md:col-span-1">
				<img
					src={verificationImage}
					alt={m.dossier_verification_image_alt()}
					class="hover-unmask h-64 w-full object-cover opacity-60 grayscale md:h-full"
				/>
				<div
					class="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent"
				></div>
				<div class="absolute bottom-2 left-2 flex items-center gap-2 text-primary">
					<span class="material-symbols-outlined text-xs">smart_card_reader</span>
					<span class="font-label text-[10px]">{m.dossier_verification_bio_link_active()}</span>
				</div>
				<div class="scan-border-pulse absolute inset-0 border-t border-primary/20"></div>
			</div>

			<div class="space-y-4 md:col-span-2">
				<div class="bg-surface-container p-6">
						<p class="mb-1 font-label text-[10px] text-outline uppercase">{m.dossier_verification_assigned_codename()}</p>
					<h2 class="font-headline text-4xl font-extrabold tracking-tight">NEON_FOX</h2>
				</div>
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="bg-surface-container p-6">
						<p class="mb-1 font-label text-[10px] text-outline uppercase">{m.dossier_verification_academic_group()}</p>
						<h3 class="font-headline text-xl font-bold text-secondary">SIGMA-091</h3>
					</div>
					<div class="flex items-center justify-center bg-surface-container p-6 text-center">
						<div>
							<span
								class="material-symbols-outlined mb-2 text-3xl text-primary"
								style="font-variation-settings:'FILL' 1">shield_with_heart</span
							>
							<p class="font-label text-[10px] text-outline uppercase">{m.dossier_verification_clearance_level_4()}</p>
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
				{m.dossier_verification_operational_boundaries()}
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
				{m.dossier_verification_legal_notice()}
			</p>
			<p class="mx-auto max-w-lg text-xs leading-relaxed text-on-surface-variant">
				{m.dossier_verification_legal_copy()}
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
					{m.dossier_verification_submit_for_approval()}
					<span class="material-symbols-outlined">arrow_forward</span>
				</a>
			</div>
		</div>
	</div>
</TerminalShell>
