<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';
	import { m } from '$lib/paraglide/messages.js';
	import { getAppContext } from '$lib/shared/providers';
	import { getLocale, setLocale, type Locale } from '$lib/paraglide/runtime.js';
	import { TerminalShell } from '$lib/shared/ui';
	import { enlistNav, heroServerImage } from '$lib/shared/config';
	import AgentIdPage from './agent-id/+page.svelte';
	import OperationalBoundariesPage from './operational-boundaries/+page.svelte';
	import DossierVerificationPage from './dossier-verification/+page.svelte';
	import DossierPage from './dossier/+page.svelte';
	import WaitingClearancePage from './waiting-clearance/+page.svelte';
	import TargetIntelPage from './target-intel/+page.svelte';
	import SurveillancePage from './surveillance/+page.svelte';
	import MissionsPage from './missions/+page.svelte';
	import LootPage from './loot/+page.svelte';
	import PerksPage from './perks/+page.svelte';
	import ReportKillPage from './report-kill/+page.svelte';
	import RevealConfirmationPage from './reveal-confirmation/+page.svelte';
	import RankingsPage from './rankings/+page.svelte';
	import AdminModerationPage from './admin/moderation/+page.svelte';
	import AdminEventsPage from './admin/events/+page.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const app = getAppContext();
	const flow = $derived(app.sessionFlow ?? data.sessionFlow ?? null);
	const isGuest = $derived(!flow || flow.status === 'guest');
	const childData = $derived({
		sessionFlow: app.sessionFlow ?? data.sessionFlow,
		sessionUser: app.sessionUser ?? data.sessionUser ?? null,
		rankings: app.rankings,
		targets: app.killTargets,
		requests: app.adminProfileRequests,
		reports: app.killReports
	});

	const loadedViews: string[] = [];

	const briefingSteps = [
		m.home_briefing_step_1(),
		m.home_briefing_step_2(),
		m.home_briefing_step_3()
	];
	const languageOptions: Array<{ value: Locale; label: string }> = [
		{ value: 'en', label: 'EN' },
		{ value: 'ru', label: 'RU' }
	];

	const bufferSegments = 4;
	let verificationProgress = $state(8);
	let encryptedBuffer = $state(0);
	let accessGranted = $state(false);
	let currentLocale = $state(getLocale());
	let cta = $derived.by(() => {
		if (!flow || flow.status === 'guest') {
			return {
				href: '/agent-id',
				label: m.home_start_registration(),
				copy: accessGranted ? m.home_system_breach_logged() : m.home_registration_terminal_locked(),
				disabled: !accessGranted,
				badge: null as string | null
			};
		}

		if (flow.status === 'no_profile') {
			return {
				href: '/agent-id',
				label: 'Complete agent registration',
				copy: 'Identity shell active. Finish profile intake to unlock dossier review.',
				disabled: false,
				badge: 'PROFILE REQUIRED'
			};
		}

		if (flow.status === 'rejected') {
			return {
				href: '/agent-id',
				label: 'Revise rejected profile',
				copy: 'Reviewer flagged dossier mismatch. Reload latest submission and resend corrected intel.',
				disabled: false,
				badge: 'REVISION REQUIRED'
			};
		}

		if (flow.status === 'pending') {
			return {
				href: '/dossier',
				label: 'Open dossier review hub',
				copy: 'Profile request in queue. Monitor review state and operative access from dossier hub.',
				disabled: false,
				badge: 'UNDER REVIEW'
			};
		}

		return {
			href: '/dossier',
			label: 'Open operative dossier hub',
			copy: 'Clearance confirmed. Resume dossier command view for mission intel, status feeds, and live access routes.',
			disabled: false,
			badge: 'OPERATIVE ACTIVE'
		};
	});

	$effect(() => {
		const view = app.view;

		if (view === 'rankings' && !loadedViews.includes(view)) {
			loadedViews.push(view);
			void app.loadRankings();
		}

		if (view === 'report-kill' && !loadedViews.includes(view)) {
			loadedViews.push(view);
			void app.loadKillTargets();
		}

		if (view === 'admin-moderation' && !loadedViews.includes(view)) {
			loadedViews.push(view);
			void app.loadAdminProfileRequests();
		}

		if (view === 'admin-events' && !loadedViews.includes(view)) {
			loadedViews.push(view);
			void app.loadKillReports();
		}
	});

	onMount(() => {
		currentLocale = getLocale();
		let cancelled = false;

		data.verification.then(() => {
			if (cancelled) {
				return;
			}

			accessGranted = true;
			verificationProgress = 100;
		});

		const verificationTimer = window.setInterval(() => {
			verificationProgress = Math.min(verificationProgress + 3, 88);
		}, 90);

		const bufferTimer = window.setInterval(() => {
			encryptedBuffer = Math.min(encryptedBuffer + 1, 100);
		}, 45);

		return () => {
			cancelled = true;
			window.clearInterval(verificationTimer);
			window.clearInterval(bufferTimer);
		};
	});

	const activeSegments = (progress: number, total: number) =>
		Math.max(1, Math.min(total, Math.round((progress / 100) * total)));

	const switchLanguage = (locale: Locale) => {
		if (locale === currentLocale) return;

		currentLocale = locale;
		void setLocale(locale);
	};
</script>

{#if app.view === 'agent-id'}
	<AgentIdPage data={childData} />
{:else if app.view === 'operational-boundaries'}
	<OperationalBoundariesPage data={childData} />
{:else if app.view === 'dossier-verification'}
	<DossierVerificationPage data={childData} />
{:else if app.view === 'dossier'}
	<DossierPage data={childData} />
{:else if app.view === 'waiting-clearance'}
	<WaitingClearancePage data={childData} />
{:else if app.view === 'target-intel'}
	<TargetIntelPage data={childData} />
{:else if app.view === 'surveillance'}
	<SurveillancePage data={childData} />
{:else if app.view === 'missions'}
	<MissionsPage data={childData} />
{:else if app.view === 'loot'}
	<LootPage data={childData} />
{:else if app.view === 'perks'}
	<PerksPage data={childData} />
{:else if app.view === 'report-kill'}
	<ReportKillPage data={childData} />
{:else if app.view === 'reveal-confirmation'}
	<RevealConfirmationPage data={childData} />
{:else if app.view === 'rankings'}
	<RankingsPage data={childData} />
{:else if app.view === 'admin-moderation'}
	<AdminModerationPage data={childData} />
{:else if app.view === 'admin-events'}
	<AdminEventsPage data={childData} />
{:else}
	<TerminalShell topBar={{ title: m.home_topbar_title(), icon: 'terminal' }}>
	<section class="mb-12">
		<div
			class="scan-sweep-soft relative flex h-72 items-end overflow-hidden bg-surface-container p-6 sm:p-8"
		>
			<img
				src={heroServerImage}
				alt={m.home_hero_image_alt()}
				class="absolute inset-0 size-full object-cover opacity-40 mix-blend-overlay grayscale"
			/>
			<div class="absolute inset-0 bg-linear-to-t from-background to-transparent"></div>
			<div
				class="absolute top-4 right-4 z-20 flex items-center gap-1 bg-background/70 p-1 backdrop-blur-sm"
			>
				{#each languageOptions as option (option.value)}
					<button
						type="button"
						onclick={() => switchLanguage(option.value)}
						class={[
							'px-3 py-1 font-label text-[10px] font-bold tracking-[0.25em] transition-colors',
							currentLocale === option.value
								? 'bg-primary text-on-primary'
								: 'text-outline hover:bg-surface-container-high hover:text-on-surface'
						]}
						aria-pressed={currentLocale === option.value}
					>
						{option.label}
					</button>
				{/each}
			</div>
			<div class="relative z-10 max-w-2xl">
				<div
					class="mb-4 inline-block bg-primary-container px-3 py-1 font-label text-xs font-bold text-on-primary-container"
				>
					{m.home_signal_acquired()}
				</div>
				<h2 class="font-headline text-4xl leading-none font-extrabold tracking-tight sm:text-5xl">
					{m.home_initiating_enlistment_protocol()}
				</h2>
				<div class="mt-4 flex items-center gap-2">
					<span class="signal-dot size-2 rounded-full bg-primary"></span>
					<p class="font-label text-sm tracking-[0.3em] text-primary">
						{m.home_encrypted_connection_stable()}
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="grid gap-8 md:grid-cols-12">
		<div class="md:col-span-7">
			<div class="relative bg-surface-container-low p-8">
				{#await data.verification}
					<div
						class="absolute top-4 right-6 font-label text-[10px] tracking-[0.3em] text-outline-variant/60"
					>
						{m.home_ref_id_establishing()}
					</div>
					<div class="mb-8 space-y-4">
						<div class="flex items-center gap-2 text-primary">
							<span class="material-symbols-outlined signal-dot">settings_ethernet</span>
							<span class="font-headline text-lg font-bold tracking-[0.25em]"
								>{m.home_verifying_signal()}</span
							>
						</div>
						<div class="space-y-2">
							<div
								class="flex items-center justify-between font-label text-[10px] tracking-[0.25em] text-outline uppercase"
							>
								<span>{m.home_server_side_credential_handshake()}</span>
								<span>{verificationProgress}%</span>
							</div>
							<div class="h-2 bg-surface-container-highest">
								<div
									class="h-full bg-linear-to-r from-primary to-primary-container transition-[width] duration-200 ease-out"
									style={`width:${verificationProgress}%`}
								></div>
							</div>
						</div>
					</div>
					<div class="mt-10 grid gap-4 sm:grid-cols-2">
						<div class="bg-surface-container p-4">
							<p class="mb-1 font-label text-[10px] text-outline uppercase">
								{m.home_clearance_level()}
							</p>
							<p class="font-headline font-bold text-outline">{m.home_pending_classification()}</p>
						</div>
						<div class="bg-surface-container p-4">
							<p class="mb-1 font-label text-[10px] text-outline uppercase">
								{m.home_threat_vector()}
							</p>
							<p class="font-headline font-bold">{m.home_cyber_insurgency()}</p>
						</div>
					</div>
				{:then verification}
					<div
						class="absolute top-4 right-6 font-label text-[10px] tracking-[0.3em] text-outline-variant"
					>
						REF_ID: {verification.refId}
					</div>
					<div class="animate-in mb-8 space-y-4">
						<div class="flex items-center gap-2 text-primary">
							<span class="material-symbols-outlined signal-dot">verified_user</span>
							<span class="font-headline text-lg font-bold tracking-[0.25em]"
								>{m.home_access_granted()}</span
							>
						</div>
						<div class="space-y-2">
							<div
								class="flex items-center justify-between font-label text-[10px] tracking-[0.25em] text-outline uppercase"
							>
								<span>{m.home_server_side_credential_handshake()}</span>
								<span>100%</span>
							</div>
							<div class="h-2 bg-surface-container-highest">
								<div
									class="h-full bg-linear-to-r from-primary to-primary-container transition-[width] duration-500 ease-out"
									style="width:100%"
								></div>
							</div>
						</div>
					</div>
					<div class="mt-10 grid gap-4 sm:grid-cols-2">
						<div class="bg-surface-container p-4 transition-all duration-500 ease-out">
							<p class="mb-1 font-label text-[10px] text-outline uppercase">
								{m.home_clearance_level()}
							</p>
							<p class="font-headline font-bold text-secondary">{verification.clearance}</p>
						</div>
						<div class="bg-surface-container p-4 transition-all duration-500 ease-out">
							<p class="mb-1 font-label text-[10px] text-outline uppercase">
								{m.home_threat_vector()}
							</p>
							<p class="font-headline font-bold">{m.home_cyber_insurgency()}</p>
						</div>
					</div>
				{/await}
			</div>
		</div>

		<div class="md:col-span-5">
			<div class="flex h-full flex-col justify-between bg-surface-container-high p-6">
				<div>
					{#if cta.badge}
						<div
							class="mb-4 inline-block bg-primary-container px-3 py-1 font-label text-[10px] font-bold tracking-[0.24em] text-on-primary-container uppercase"
						>
							{cta.badge}
						</div>
					{/if}
					<h3
						class="mb-6 font-headline text-xs font-bold tracking-[0.4em] text-on-surface-variant uppercase"
					>
						{m.home_mission_briefing()}
					</h3>
					{#if !isGuest}
						<p
							class="mb-6 border-l-4 border-primary/50 bg-surface-container px-4 py-3 text-sm leading-relaxed text-on-surface-variant"
						>
							{cta.copy}
						</p>
					{/if}
					<ul class="space-y-4">
						{#each briefingSteps as step, index (step)}
							<li class="flex gap-4">
								<span class="font-label font-bold text-primary">0{index + 1}</span>
								<span class="flex-1 border-b border-outline-variant/30 pb-2 text-sm">{step}</span>
							</li>
						{/each}
					</ul>
				</div>

				<div class="mt-8">
					<a
					href={resolve(cta.href as Pathname)}
						aria-disabled={cta.disabled}
						class={`glitch-burst tactical-button flex w-full items-center justify-between px-8 py-5 font-headline font-extrabold tracking-[0.2em] uppercase transition-[filter,transform,brightness,opacity] hover:brightness-110 ${cta.disabled ? 'pointer-events-none opacity-50 saturate-0' : ''}`}
					>
						<span>{cta.label}</span>
						<span class="material-symbols-outlined">chevron_right</span>
					</a>
					<p class="mt-4 text-center font-label text-[10px] tracking-[0.3em] text-outline">
						{cta.copy}
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="mt-4 mb-12">
		<div class="flex items-center gap-4 bg-surface-container p-1">
			<div class="bg-secondary-container p-3 text-secondary">
				<span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1"
					>security</span
				>
			</div>
			<div class="flex-1">
				<p class="font-label text-xs font-bold tracking-tight text-secondary uppercase">
					{m.home_encrypted_buffer()}
				</p>
				<div class="mt-2 grid grid-cols-4 gap-1">
					{#each Array.from({ length: bufferSegments }, (_, index) => index) as index (index)}
						<div class="h-2 overflow-hidden bg-surface-container-highest">
							<div
								class={`h-full transition-[width,background-color,box-shadow] duration-300 ease-out ${index < activeSegments(encryptedBuffer, bufferSegments) ? 'bg-secondary shadow-[0_0_14px_rgba(254,169,255,0.28)]' : 'bg-secondary/20'}`}
								style={`width:${Math.max(0, Math.min(100, encryptedBuffer - index * (100 / bufferSegments)) * bufferSegments)}%`}
							></div>
						</div>
					{/each}
				</div>
			</div>
			<span class="px-4 font-label text-[10px] text-outline transition-all duration-300"
				>{m.home_buffer_ready({ percent: String(encryptedBuffer) })}</span
			>
		</div>
	</section>
	</TerminalShell>
{/if}
