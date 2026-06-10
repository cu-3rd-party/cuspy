<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { readAccessToken } from '$lib/auth/session';
	import { m } from '$lib/paraglide/messages.js';
	import {
		canAccessStep,
		loadDossierDraft,
		saveDossierDraft,
		type DossierDraft
	} from '$lib/prototype/dossierDraft';
	import TerminalShell from '$lib/components/TerminalShell.svelte';
	import { enlistNav, verificationImage } from '$lib/prototype/data';
	import { sessionUser } from '$lib/stores/session';
	import type { AgentProfileData } from '$lib/stores/session';
	import type { SessionUser } from '$lib/stores/session';

	let draft = $state<DossierDraft>(loadDossierDraft());
	let profileImage = $derived(draft.agentId.identificationImage || verificationImage);
	let submitError = $state('');
	let isSubmitting = $state(false);

	const statusTone = (active: boolean) => (active ? 'primary' : 'secondary');

	let boundaries = $derived([
		[
			m.dossier_verification_physical_contact(),
			draft.boundaries.physicalContact ? m.common_authorized() : m.common_restricted(),
			statusTone(draft.boundaries.physicalContact)
		],
		[
			m.dossier_verification_hugs(),
			draft.boundaries.hugsCloseProximity ? m.common_authorized() : m.common_restricted(),
			statusTone(draft.boundaries.hugsCloseProximity)
		],
	] as const);

	onMount(() => {
		draft = loadDossierDraft();
		if (!canAccessStep(draft, 3)) {
			goto(resolve('/agent-id'), { replaceState: true });
		}
	});

	const buildRequestedProfileData = (): AgentProfileData => ({
		codename: draft.agentId.codename,
		academicGroup: draft.agentId.academicGroup,
		academicLevel: draft.agentId.academicLevel,
		courseNumber: draft.agentId.courseNumber,
		bachelorTrack: draft.agentId.bachelorTrack,
		identificationName: draft.agentId.identificationName,
		identificationImage: draft.agentId.identificationImage,
		boundaries: {
			physicalContact: draft.boundaries.physicalContact,
			hugsCloseProximity: draft.boundaries.hugsCloseProximity
		}
	});

	const handleSubmit = async () => {
		const token = readAccessToken();
		if (!token) {
			submitError = 'Missing session token';
			return;
		}

		isSubmitting = true;
		submitError = '';

		try {
			const requestedProfileData = buildRequestedProfileData();
			const response = await fetch('/profile-creation-requests', {
				method: 'POST',
				headers: {
					'content-type': 'application/json',
					authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					requested_profile_data: requestedProfileData
				})
			});

			if (!response.ok) {
				const payload = await response.json().catch(() => ({ error: 'Request failed' }));
				throw new Error(payload.error ?? 'Request failed');
			}

			const payload = (await response.json().catch(() => ({}))) as {
				user?: SessionUser;
			};

			draft.registrationCompleted = true;
			draft.unlockedStep = 3;
			saveDossierDraft(draft);

			if (payload.user) {
				sessionUser.set({
					...payload.user,
					agent_name: payload.user.agent_name ?? draft.agentId.codename,
					agent_data: {
						...requestedProfileData,
						...payload.user.agent_data,
						boundaries: requestedProfileData.boundaries
					}
				});
			} else {
				sessionUser.update((current) =>
					current
						? {
							...current,
							agent_name: draft.agentId.codename,
							agent_data: {
								...current.agent_data,
								...requestedProfileData,
								boundaries: requestedProfileData.boundaries
							}
						}
						: current
				);
			}

			await goto(resolve('/dossier'), { invalidateAll: true });
		} catch (error) {
			submitError = error instanceof Error ? error.message : 'Failed to submit profile';
		} finally {
			isSubmitting = false;
		}
	};
</script>

<TerminalShell
	topBar={{ title: m.dossier_verification_topbar_title(), icon: 'security' }}
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
					<p class="font-label text-[10px] text-outline uppercase">
						{m.dossier_verification_agent_id()}
					</p>
					<p class="font-headline text-sm font-bold text-secondary">AG-004-FOX-SIGMA</p>
				</div>
			</div>
		</section>

		<div class="mb-4 grid gap-4 md:grid-cols-3">
			<div class="relative overflow-hidden bg-surface-container md:col-span-1">
				<img
					src={profileImage}
					alt={m.dossier_verification_image_alt()}
					class="hover-unmask h-64 w-full object-cover opacity-60 grayscale md:h-full"
				/>
				<div
					class="absolute inset-0 bg-linear-to-t from-black via-transparent to-transparent"
				></div>
				<div class="absolute bottom-2 left-2 flex items-center gap-2 text-primary">
					<span class="material-symbols-outlined text-xs">smart_card_reader</span>
					<span class="font-label text-[10px]">{m.dossier_verification_bio_link_active()}</span>
				</div>
				<div class="scan-border-pulse absolute inset-0 border-t border-primary/20"></div>
			</div>

			<div class="space-y-4 md:col-span-2">
				<div class="bg-surface-container p-6">
					<p class="mb-1 font-label text-[10px] text-outline uppercase">
						{m.dossier_verification_assigned_codename()}
					</p>
					<h2 class="font-headline text-4xl font-extrabold tracking-tight">
						{draft.agentId.codename || 'NEON_FOX'}
					</h2>
				</div>
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="bg-surface-container p-6">
						<p class="mb-1 font-label text-[10px] text-outline uppercase">
							{m.dossier_verification_academic_group()}
						</p>
						<h3 class="font-headline text-xl font-bold text-secondary">
							{draft.agentId.academicGroup || 'SIGMA-091'}
						</h3>
					</div>
					<div class="flex items-center justify-center bg-surface-container p-6 text-center">
						<div>
							<span
								class="material-symbols-outlined mb-2 text-3xl text-primary"
								style="font-variation-settings:'FILL' 1">shield_with_heart</span
							>
							<p class="font-label text-[10px] text-outline uppercase">
								{m.dossier_verification_clearance_level_4()}
							</p>
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
				{#each boundaries as [label, status, tone] (label)}
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
			class="mt-4 border border-dashed border-error/30 bg-surface-container-low p-6 text-center"
		>
			<p class="mb-2 font-label text-[10px] font-bold tracking-[0.3em] text-error uppercase">
				{m.dossier_verification_legal_notice()}
			</p>
			<p class="mx-auto max-w-lg text-xs leading-relaxed text-on-surface-variant">
				{m.dossier_verification_legal_copy()}
			</p>
		</footer>

		{#if submitError}
			<div class="mb-6 bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase">
				{submitError}
			</div>
		{/if}

		<div
			class="inset-x-0 mt-4 bg-linear-to-t from-background via-background to-transparent"
		>
			<div class="mx-auto flex justify-end md:pr-0">
				<button
					type="button"
					onclick={handleSubmit}
					disabled={isSubmitting}
					class="press-shift tactical-button flex h-16 w-full items-center justify-center gap-3 font-headline font-bold tracking-[0.2em] uppercase shadow-[0_0_20px_rgba(0,122,27,0.4)] transition-transform"
				>
					{isSubmitting ? 'Submitting' : m.dossier_verification_submit_for_approval()}
					<span class="material-symbols-outlined">arrow_forward</span>
				</button>
			</div>
		</div>
	</div>
</TerminalShell>
