<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import { ProfileStatePanel } from '$lib/shared/ui';
	import { TerminalShell } from '$lib/shared/ui';
	import { agentAvatar, dossierNav } from '$lib/shared/config';
	import { sessionUser, type SessionFlow } from '$lib/shared/model';

	let { data } = $props<{
		data: {
			sessionFlow?: SessionFlow;
			sessionUser?: SessionFlow['user'];
		};
	}>();

	let flow = $derived(data.sessionFlow ?? null);
	let user = $derived(data.sessionUser ?? flow?.user ?? $sessionUser);
	let codename = $derived(
		(user?.agent_data?.codename as string | undefined) || user?.agent_name || 'AGENT'
	);
	let clearance = $derived(user?.rating ? `Rating ${user.rating}` : m.dossier_clearance_level_4());
	let reviewerNote = $derived(
		flow?.latestProfileRequest?.reviewer_note?.trim() ||
			'No reviewer note attached. Reopen profile editor and retransmit corrected dossier intel.'
	);
	let stateLabel = $derived.by(() => {
		if (!flow || flow.status === 'guest') {
			return 'UNAUTHENTICATED';
		}

		if (flow.status === 'no_profile') {
			return 'PROFILE REQUIRED';
		}

		if (flow.status === 'pending') {
			return 'UNDER REVIEW';
		}

		if (flow.status === 'rejected') {
			return 'REVISION REQUIRED';
		}

		return 'OPERATIVE ACTIVE';
	});
	let hubPanel = $derived.by(() => {
		if (!flow || flow.status === 'guest' || flow.status === 'approved') {
			return null;
		}

		if (flow.status === 'no_profile') {
			return {
				kicker: 'Profile intake incomplete',
				title: 'Operative shell detected',
				body: 'Account exists. Identity packet still missing required profile fields and verification artifacts.',
				icon: 'assignment_ind',
				href: '/agent-id',
				ctaLabel: 'Start registration',
				tone: 'primary' as const
			};
		}

		if (flow.status === 'pending') {
			return {
				kicker: 'Review queue active',
				title: 'Dossier awaiting clearance',
				body: 'Profile submission received. Hold position while command validates identity packet and operational boundaries.',
				icon: 'hourglass_top',
				href: null,
				ctaLabel: null,
				tone: 'secondary' as const
			};
		}

		return {
			kicker: 'Review failed',
			title: 'Dossier returned for correction',
			body: 'Moderator review rejected latest profile request. Update intake data, confirm boundaries, resend package.',
			note: reviewerNote,
			icon: 'warning',
			href: '/agent-id',
			ctaLabel: 'Edit rejected profile',
			tone: 'error' as const
		};
	});
	let statusCards = $derived.by(() => {
		if (!flow || flow.status === 'guest') {
			return [];
		}

		if (flow.status === 'no_profile') {
			return [
				['identity packet', 'missing'],
				['boundary consent', 'locked'],
				['field access', 'stand by']
			] as const;
		}

		if (flow.status === 'pending') {
			return [
				['submission status', 'queued'],
				['review channel', 'command audit'],
				['field access', 'restricted']
			] as const;
		}

		if (flow.status === 'rejected') {
			return [
				['submission status', 'returned'],
				['review channel', 'moderator flagged'],
				['field access', 'restricted']
			] as const;
		}

		return [
			['submission status', 'confirmed'],
			['clearance gate', 'open'],
			['field access', 'live']
		] as const;
	});
	let quickLinks = $derived.by(() => {
		if (!flow || flow.status === 'guest') {
			return [];
		}

		if (flow.status === 'no_profile') {
			return [
				{
					href: resolve('/agent-id'),
					label: 'Resume registration',
					copy: 'Transmit missing identity data'
				},
				{
					href: resolve('/'),
					label: 'Return to landing terminal',
					copy: 'Review onboarding channel'
				}
			];
		}

		if (flow.status === 'rejected') {
			return [
				{
					href: resolve('/agent-id'),
					label: 'Reload profile editor',
					copy: 'Patch rejected identity packet'
				},
				{
					href: resolve('/dossier'),
					label: 'Review rejection state',
					copy: 'Inspect reviewer note and queue state'
				}
			];
		}

		if (flow.status === 'pending') {
			return [
				{
					href: resolve('/waiting-clearance'),
					label: 'Open waiting screen',
					copy: 'Track review queue and live access'
				},
				{
					href: resolve('/rankings'),
					label: m.nav_rank(),
					copy: 'Monitor operator standings while waiting'
				},
				{
					href: resolve('/surveillance'),
					label: 'Surveillance',
					copy: 'Preview field network channels'
				}
			];
		}

		return [
			{
				href: resolve('/target-intel'),
				label: m.dossier_view_target_intel(),
				copy: 'Track active target package'
			},
			{ href: resolve('/rankings'), label: m.nav_rank(), copy: 'Review operator standings' },
			{ href: resolve('/surveillance'), label: 'Surveillance', copy: 'Monitor field network' }
		];
	});
</script>

<TerminalShell
	topBar={{
		title: m.dossier_topbar_title(),
		icon: 'security',
		meta: m.dossier_topbar_meta(),
		avatar: agentAvatar
	}}
	nav={dossierNav}
>
	{#if !flow || flow.status === 'guest'}
		<div class="mx-auto max-w-5xl space-y-8">
			<ProfileStatePanel
				kicker="Authentication required"
				title="Signal not recognized"
				body="Establish secure session first. Landing terminal handles access verification and operative onboarding."
				icon="vpn_key"
				href="/"
				ctaLabel="Return to landing terminal"
				tone="secondary"
			/>
		</div>
	{:else}
		<div class="mx-auto max-w-5xl space-y-8">
			<section class="mb-12 border-b border-outline-variant/10 pb-8">
				<div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
					<div>
						<span
							class="mb-2 block font-headline text-xs tracking-[0.3em] text-primary/60 uppercase"
							>{m.dossier_subject_dossier()}</span
						>
						<h2 class="font-headline text-5xl font-bold tracking-tight sm:text-7xl">
							{codename}
						</h2>
						<div class="mt-2 flex flex-wrap items-center gap-4">
							<span
								class="bg-primary-container px-3 py-1 font-headline text-xs font-bold tracking-[0.2em] text-on-primary-container"
								>ID: 8829-X</span
							>
							<span class="font-headline text-xs font-bold tracking-[0.2em] text-outline"
								>LOC: [REDACTED]</span
							>
							<span
								class="bg-surface-container px-3 py-1 font-headline text-xs font-bold tracking-[0.2em] text-secondary uppercase"
							>
								{stateLabel}
							</span>
						</div>
					</div>
					<div class="text-right">
						<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">
							{m.dossier_current_authorization()}
						</div>
						<div class="font-headline text-2xl font-bold tracking-tight text-secondary">
							{clearance}
						</div>
					</div>
				</div>
			</section>

			{#if hubPanel}
				<ProfileStatePanel {...hubPanel} />
			{/if}

			<section class="grid gap-4 md:grid-cols-3">
				{#each statusCards as [label, value] (label)}
					<div class="border-l-4 border-primary bg-surface-container p-6">
						<div class="mb-3 font-headline text-[10px] tracking-[0.2em] text-primary uppercase">
							{label}
						</div>
						<p class="font-headline text-2xl font-bold text-on-surface uppercase">{value}</p>
					</div>
				{/each}
			</section>

			{#if flow.status === 'approved'}
				<div class="grid gap-1 md:grid-cols-12">
					<div
						class="relative flex min-h-80 flex-col justify-between overflow-hidden bg-surface-container-low p-8 md:col-span-8"
					>
						<div class="absolute top-0 right-0 p-4 opacity-10">
							<span
								class="material-symbols-outlined text-9xl"
								style="font-variation-settings:'FILL' 1">shield</span
							>
						</div>
						<div class="relative z-10">
							<span
								class="mb-6 block font-headline text-[10px] tracking-[0.2em] text-outline uppercase"
								>{m.dossier_combat_performance()}</span
							>
							<div class="mb-8 flex items-center gap-8">
								<div>
									<div
										class="mb-1 font-headline text-xs tracking-[0.2em] text-on-surface/60 uppercase"
									>
										{m.dossier_current_tier()}
									</div>
									<div class="font-headline text-4xl font-bold">{m.dossier_silver_iv()}</div>
								</div>
								<div class="h-12 w-px bg-outline-variant/30"></div>
								<div>
									<div
										class="mb-1 font-headline text-xs tracking-[0.2em] text-on-surface/60 uppercase"
									>
										{m.dossier_elo_rating()}
									</div>
									<div class="font-headline text-4xl font-bold text-primary">1450</div>
								</div>
							</div>
						</div>
						<div class="relative z-10">
							<div class="mb-4 flex items-end justify-between">
								<span class="font-headline text-[10px] tracking-[0.2em] uppercase"
									>{m.dossier_experience_progress()}</span
								>
								<span class="font-headline text-xs font-bold text-primary"
									>{m.dossier_progress_to_gold()}</span
								>
							</div>
							<div class="segment-bar grid-cols-6">
								<span class="active"></span><span class="active"></span><span class="active"
								></span><span class="active"></span><span></span><span></span>
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
								>{m.dossier_system_status()}</span
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
											{m.dossier_hunting()}
										</div>
										<div class="text-sm text-on-secondary-container/90">
											{m.dossier_target_broker()}
										</div>
									</div>
								</div>
								<div class="flex gap-4">
									<span class="material-symbols-outlined text-2xl text-error">warning</span>
									<div>
										<div
											class="font-headline text-xs font-bold tracking-[0.2em] text-error uppercase"
										>
											{m.dossier_under_pursuit()}
										</div>
										<div class="text-sm text-error/90">{m.dossier_threat_unknown_agent()}</div>
									</div>
								</div>
							</div>
						</div>
						<a
							href={resolve('/target-intel')}
							class="press-scale relative z-10 border border-secondary/30 bg-black px-4 py-4 text-center font-headline text-xs font-bold tracking-[0.2em] text-secondary uppercase transition-colors hover:bg-secondary hover:text-black"
						>
							{m.dossier_view_target_intel()}
						</a>
					</div>
				</div>
			{/if}

			<section class={`grid gap-4 ${quickLinks.length >= 3 ? 'md:grid-cols-3' : 'md:grid-cols-2'}`}>
				{#each quickLinks as link (link.href)}
					<a
						href={link.href}
						class="group border border-outline-variant/20 bg-surface-container p-6 transition-colors hover:border-primary/40 hover:bg-surface-container-high"
					>
						<div class="font-headline text-sm font-bold tracking-[0.2em] text-primary uppercase">
							{link.label}
						</div>
						<p class="mt-3 text-sm text-on-surface-variant">{link.copy}</p>
						<div
							class="mt-6 font-label text-[10px] tracking-[0.24em] text-outline uppercase group-hover:text-primary"
						>
							Access route
						</div>
					</a>
				{/each}
			</section>

			{#if flow.status === 'approved'}
				<div class="grid gap-1 md:grid-cols-3">
					<div class="border-l-4 border-primary bg-surface-container p-6">
						<div class="mb-3 font-headline text-[10px] tracking-[0.2em] text-primary uppercase">
							{m.dossier_recent_activity()}
						</div>
						<p class="text-sm leading-relaxed text-on-surface/80">
							{m.dossier_recent_activity_copy()}
						</p>
					</div>
					<div class="border-l-4 border-secondary bg-surface-container p-6">
						<div class="mb-3 font-headline text-[10px] tracking-[0.2em] text-secondary uppercase">
							{m.dossier_priority_objectives()}
						</div>
						<ul class="space-y-2 text-sm text-on-surface/80">
							<li class="flex items-center gap-2">
								<span class="size-1.5 bg-secondary"></span>{m.dossier_objective_infiltrate_tokyo()}
							</li>
							<li class="flex items-center gap-2 opacity-50">
								<span class="size-1.5 bg-outline"></span>{m.dossier_objective_recover_hard_drive()}
							</li>
						</ul>
					</div>
					<div
						class="scanning-grid relative flex items-center justify-center overflow-hidden bg-surface-container-high p-6 text-center"
					>
						<div class="relative z-10">
							<span class="material-symbols-outlined mb-2 text-3xl text-outline">add_circle</span>
							<div
								class="font-headline text-[10px] font-bold tracking-[0.2em] text-outline uppercase"
							>
								{m.dossier_new_mission_available()}
							</div>
						</div>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</TerminalShell>
