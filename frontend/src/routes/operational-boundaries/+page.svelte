<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '$lib/paraglide/messages.js';
	import { getAppContext } from '$lib/shared/providers';
	import {
		canAccessStep,
		loadDossierDraft,
		saveDossierDraft,
		type DossierDraft
	} from '$lib/pages/profile-flow';
	import { ProgressBar } from '$lib/shared/ui';
	import { TerminalShell } from '$lib/shared/ui';
	import { boundariesImage, enlistNav } from '$lib/shared/config';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
	const app = getAppContext();

	let draft = $state<DossierDraft>(loadDossierDraft());
	let toggles = $state([
		{
			icon: 'front_hand',
			code: 'SEC_LEVEL_01',
			title: m.boundaries_physical_contact_title(),
			copy: m.boundaries_physical_contact_copy(),
			status_authorized: m.common_authorized(),
			status_restricted: m.common_restricted(),
			active: true
		},
		{
			icon: 'groups',
			code: 'SEC_LEVEL_02',
			title: m.boundaries_hugs_title(),
			copy: m.boundaries_hugs_copy(),
			status_authorized: m.common_authorized(),
			status_restricted: m.common_restricted(),
			active: false
		}
	]);

	let bioStage = $state([0, 1]);
	let bioStageLen = 5;

	const toggleBoundary = (index: number) => {
		toggles[index].active = !toggles[index].active;
	};

	onMount(() => {
		draft = loadDossierDraft();
		if (!canAccessStep(draft, 2)) {
			app.navigate('/agent-id');
			return;
		}

		toggles[0].active = draft.boundaries.physicalContact;
		toggles[1].active = draft.boundaries.hugsCloseProximity;

		const interval = window.setInterval(() => {
			bioStage = bioStage.map((value) => (value + 1) % bioStageLen);
		}, 100);

		return () => window.clearInterval(interval);
	});

	$effect(() => {
		draft.boundaries.physicalContact = toggles[0].active;
		draft.boundaries.hugsCloseProximity = toggles[1].active;
		saveDossierDraft(draft);
	});

	const handleSubmit = () => {
		draft.unlockedStep = Math.max(draft.unlockedStep, 3) as DossierDraft['unlockedStep'];
		saveDossierDraft(draft);
		app.navigate('/dossier-verification');
	};
</script>

<TerminalShell topBar={{ title: m.home_topbar_title(), icon: 'terminal' }} nav={enlistNav}>
	<div class="mx-auto max-w-4xl">
		<ProgressBar
			current={2}
			total={2}
			label={m.boundaries_progress_label()}
			status={m.boundaries_progress_status()}
		/>

		<section class="mt-12 mb-10">
			<h1 class="font-headline text-4xl font-bold tracking-tight uppercase sm:text-6xl">
				{m.boundaries_title()}
			</h1>
			<p class="mt-4 max-w-xl border-l-2 border-secondary-container pl-4 text-outline italic">
				{m.boundaries_intro()}
			</p>
		</section>

		<div class="grid gap-4 md:grid-cols-2">
			{#each toggles as toggle, index (toggle.code)}
				<button
					type="button"
					onclick={() => toggleBoundary(index)}
					aria-pressed={toggle.active}
					class="flex flex-col justify-between bg-surface-container p-6 text-left transition-colors hover:bg-surface-container-high"
				>
					<span class="mb-8">
						<span class="mb-4 flex items-start justify-between">
							<span
								class="material-symbols-outlined text-3xl text-secondary"
								style={toggle.active ? "font-variation-settings:'FILL' 1" : ''}>{toggle.icon}</span
							>
							<span
								class="bg-surface-container-low px-2 py-1 font-label text-[10px] tracking-[0.2em] text-outline uppercase"
								>{toggle.code}</span
							>
						</span>
						<span class="mb-2 font-headline text-xl font-bold uppercase">{toggle.title}</span>
						<span class="text-sm text-on-surface-variant">{toggle.copy}</span>
					</span>
					<span class="flex items-center justify-between">
						<span class="font-label text-xs tracking-[0.2em] text-outline uppercase"
							>{m.common_status()}:
							<span class={toggle.active ? 'text-primary' : 'text-error'}
								>{toggle.active ? toggle.status_authorized : toggle.status_restricted}</span
							></span
						>
						<span
							class={`relative h-6 w-12 ${toggle.active ? 'bg-primary-container' : 'bg-surface-container-highest'} ring-1 ring-outline-variant`}
						>
							<span
								class={`absolute top-0.5 size-5 bg-on-surface transition-all ${toggle.active ? 'left-6.5' : 'left-0.5'}`}
							></span>
						</span>
					</span>
				</button>
			{/each}

			<div
				class="flex items-center gap-6 border-l-4 border-secondary bg-secondary-container/10 p-6 md:col-span-2"
			>
				<span class="material-symbols-outlined signal-dot text-4xl text-secondary">warning</span>
				<div>
					<h4 class="font-headline font-bold text-secondary uppercase">
						{m.boundaries_critical_protocol()}
					</h4>
					<p class="text-sm text-on-surface-variant">
						{m.boundaries_critical_copy()}
					</p>
				</div>
			</div>

			<div class="bg-surface-container-lowest p-6">
				{#each [[m.boundaries_agent_id(), 'RX-9042-ALFA', 'primary'], [m.boundaries_consent_hash(), 'E92_D77_01X', 'secondary'], [m.common_timestamp(), '2024-05-21 14:02:11', 'on-surface']] as [label, value, tone] (label)}
					<div
						class="flex justify-between border-b border-outline-variant/30 py-3 first:pt-0 last:border-b-0 last:pb-0"
					>
						<span class="font-label text-[10px] text-outline uppercase">{label}</span>
						<span
							class={`font-label text-[10px] uppercase ${tone === 'primary' ? 'text-primary' : tone === 'secondary' ? 'text-secondary' : 'text-on-surface'}`}
							>{value}</span
						>
					</div>
				{/each}
			</div>

			<div class="relative min-h-40 overflow-hidden bg-surface-container">
				<img
					src={boundariesImage}
					alt={m.boundaries_image_alt()}
					class="absolute inset-0 size-full object-cover opacity-40 mix-blend-overlay"
				/>
				<div class="absolute inset-0 bg-linear-to-t from-surface-container to-transparent"></div>
				<div class="absolute bottom-4 left-4">
					<span class="mb-1 block font-label text-[10px] text-primary-fixed"
						>{m.boundaries_scanning_bio_rhythms()}</span
					>
					<div class="animated-stage-grid">
						{#each Array.from({ length: bioStageLen }, (_, index) => index) as index (index)}
							<span class:active={bioStage.includes(index)}></span>
						{/each}
					</div>
				</div>
			</div>
		</div>

		<div class="mt-8 flex flex-col gap-4 md:flex-row md:items-center">
			<button
				type="button"
				onclick={handleSubmit}
				class="glitch-burst press-scale tactical-button group flex flex-1 items-center justify-center gap-4 px-10 py-5 font-headline text-xl font-extrabold uppercase shadow-[0_0_20px_rgba(0,122,27,0.2)] transition-all hover:bg-primary"
			>
				{m.boundaries_finalize_dossier()}
				<span class="material-symbols-outlined group-hover-slide">double_arrow</span>
			</button>
			<!--			<a href={resolve('/')} class="px-8 py-5 font-label text-sm uppercase tracking-[0.3em] text-outline transition-colors hover:text-on-surface">RETRACT APPLICATION</a>-->
		</div>
	</div>
</TerminalShell>
