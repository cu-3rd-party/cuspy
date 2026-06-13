<script lang="ts">
	import { onMount } from 'svelte';
	import {
		writeAccessToken,
		writeAuthPayload,
		readAuthPayload,
		type AuthPayload
	} from '$lib/shared/auth';
	import { m } from '$lib/paraglide/messages.js';
	import { loginUser, registerUser, getCurrentUser } from '$lib/shared/api';
	import { getAppContext } from '$lib/shared/providers';
	import { applyProfileDataToDraft } from '$lib/pages/profile-flow';
	import {
		isAgentIdComplete,
		loadDossierDraft,
		saveDossierDraft,
		type DossierDraft
	} from '$lib/pages/profile-flow';
	import { ProgressBar } from '$lib/shared/ui';
	import { AgentPersonalInfo } from '$lib/shared/ui';
	import { TerminalShell } from '$lib/shared/ui';
	import { enlistNav } from '$lib/shared/config';
	import { Countdown } from '$lib/shared/ui';
	import { NodeConnectivity } from '$lib/shared/ui';
	import type { SessionFlow } from '$lib/shared/model';
	import { Icon } from '$lib/shared/ui';

	let { data } = $props<{
		data: {
			sessionFlow?: SessionFlow;
			sessionUser?: SessionFlow['user'];
		};
	}>();
	const app = getAppContext();

	app.view = 'agent-id';

	let draft = $state<DossierDraft>(loadDossierDraft());
	let uploadError = $state(false);
	let invalidUploadMessage = $state('');
	let submitError = $state('');
	let isSubmitting = $state(false);
	let popupTimeout: number | undefined;
	let hydratedRejectedRequestId = $state<string | null>(null);
	let flow = $derived(app.sessionFlow ?? data.sessionFlow ?? null);
	let activeSessionUser = $derived(app.sessionUser ?? data.sessionUser ?? flow?.user ?? null);
	let rejectedRequest = $derived(flow?.status === 'rejected' ? flow.latestProfileRequest : null);

	const academicLevels = [
		{ value: 'bachelor', label: m.agent_id_bachelor() },
		{ value: 'master', label: m.agent_id_master() },
		{ value: 'other', label: m.agent_id_worker() }
	] as const;

	const bachelorTracks = [
		{ value: 'development', label: m.agent_id_development() },
		{ value: 'ai', label: m.agent_id_ai() },
		{ value: 'business', label: m.agent_id_buisness() }
	] as const;

	let courseOptions = $derived.by(() => {
		if (draft.agentId.academicLevel === 'bachelor') {
			return ['1', '2', '3', '4'] as const;
		}

		if (draft.agentId.academicLevel === 'master') {
			return ['1', '2'] as const;
		}

		return [] as const;
	});

	let showCourseNumber = $derived(
		draft.agentId.academicLevel === 'bachelor' || draft.agentId.academicLevel === 'master'
	);
	let showBachelorTrack = $derived(
		draft.agentId.academicLevel === 'bachelor' && Number(draft.agentId.courseNumber) >= 2
	);

	const hydrateRejectedDraft = (baseDraft: DossierDraft) => {
		if (!rejectedRequest) {
			return baseDraft;
		}

		hydratedRejectedRequestId = rejectedRequest.profile_request_id;

		return applyProfileDataToDraft(baseDraft, rejectedRequest.requested_profile_data);
	};

	onMount(() => {
		draft = hydrateRejectedDraft(loadDossierDraft());

		return () => {
			if (popupTimeout) {
				window.clearTimeout(popupTimeout);
			}
		};
	});

	$effect(() => {
		if (!rejectedRequest || hydratedRejectedRequestId === rejectedRequest.profile_request_id) {
			return;
		}

		draft = hydrateRejectedDraft(loadDossierDraft());
	});

	$effect(() => {
		saveDossierDraft(draft);
	});

	$effect(() => {
		if (!courseOptions.includes(draft.agentId.courseNumber as never)) {
			draft.agentId.courseNumber = '';
		}

		if (draft.agentId.academicLevel !== 'bachelor' || Number(draft.agentId.courseNumber) < 2) {
			draft.agentId.bachelorTrack = '';
		}
	});

	const showUploadError = () => {
		uploadError = true;
		invalidUploadMessage = m.agent_id_upload_invalid_image();

		if (popupTimeout) {
			window.clearTimeout(popupTimeout);
		}

		popupTimeout = window.setTimeout(() => {
			invalidUploadMessage = '';
			uploadError = false;
		}, 3000);
	};

	const handleIdentificationChange = async (event: Event) => {
		const input = event.currentTarget as HTMLInputElement;
		const file = input.files?.[0];

		if (!file) {
			draft.agentId.identificationName = '';
			draft.agentId.identificationImage = '';
			return;
		}

		if (!file.type.startsWith('image/')) {
			input.value = '';
			showUploadError();
			return;
		}

		uploadError = false;
		invalidUploadMessage = '';
		draft.agentId.identificationName = file.name;

		const reader = new FileReader();
		const imageData = await new Promise<string | null>((resolve) => {
			reader.onload = () => resolve(typeof reader.result === 'string' ? reader.result : null);
			reader.onerror = () => resolve(null);
			reader.readAsDataURL(file);
		});

		if (!imageData) {
			input.value = '';
			showUploadError();
			return;
		}

		draft.agentId.identificationImage = imageData;
	};

	const saveAndProceed = () => {
		draft.registrationCompleted = true;
		draft.unlockedStep = Math.max(draft.unlockedStep, 2) as DossierDraft['unlockedStep'];
		saveDossierDraft(draft);
		app.navigate('/operational-boundaries');
	};

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault();

		if (!isAgentIdComplete(draft)) {
			return;
		}

		isSubmitting = true;
		submitError = '';

		try {
			if (activeSessionUser) {
				app.setSessionUser(activeSessionUser);
				saveAndProceed();
				return;
			}

			// Try to recover existing session from stored token
			try {
				const user = await getCurrentUser();
				app.setSessionUser(user);
				saveAndProceed();
				return;
			} catch {
				// Token invalid — try to re-login with stored telegram_id
				const storedAuthPayload = readAuthPayload();

				if (storedAuthPayload != null) {
					try {
						const loginPayload = await loginUser(storedAuthPayload);
						writeAccessToken(loginPayload.access_token);
						app.setSessionUser(loginPayload.user);
						writeAuthPayload(storedAuthPayload);
						saveAndProceed();
						return;
					} catch {
						// re-login also failed — fall through to registration
					}
				}
			}

			// No viable session — register a new user
			const telegramId = Date.now(); // backend automatically infers telegram id from tg webapp data, so we can use any value here
			const payload: AuthPayload = {
				email: `${draft.agentId.codename.toLowerCase()}@dev.local`,
				password: 'password123',
				telegram_id: telegramId,
				agent_name: draft.agentId.codename
			};
			const response = await registerUser(payload);
			writeAccessToken(response.access_token);
			writeAuthPayload(payload);
			app.setSessionUser(response.user);
			saveAndProceed();
		} catch (error) {
			submitError = error instanceof Error ? error.message : 'Failed to register';
		} finally {
			isSubmitting = false;
		}
	};
</script>

<TerminalShell topBar={{ title: m.home_topbar_title(), icon: 'terminal' }} nav={enlistNav} {flow}>
	<div class="mx-auto max-w-xl">
		{#if invalidUploadMessage}
			<div
				class="fixed top-24 right-4 left-4 z-[60] mx-auto max-w-sm bg-error px-4 py-3 text-center font-label text-[11px] font-bold tracking-[0.2em] text-white uppercase shadow-[0_0_24px_rgba(186,26,26,0.45)] sm:left-auto"
			>
				{invalidUploadMessage}
			</div>
		{/if}

		{#if submitError}
			<div
				class="mt-4 bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase"
			>
				{submitError}
			</div>
		{/if}

		<ProgressBar
			current={1}
			total={2}
			label={m.agent_id_progress_label()}
			status={m.agent_id_progress_status()}
		/>

		<section class="scan-sweep relative mt-12 overflow-hidden bg-surface-container">
			<div class="bg-surface-container-low p-6">
				<div class="flex items-center justify-between">
					<div>
						<h2 class="font-headline text-2xl font-bold uppercase">{m.agent_id_title()}</h2>
						<p class="mt-1 font-label text-[10px] tracking-[0.3em] text-outline uppercase">
							{m.agent_id_clearance_level()}
						</p>
					</div>
					<span
						class="material-symbols-outlined text-3xl text-secondary"
						style="font-variation-settings:'FILL' 1">security</span
					>
				</div>
			</div>

			<form class="space-y-10 p-8" onsubmit={handleSubmit}>
				<AgentPersonalInfo
					{handleIdentificationChange}
					{uploadError}
					bind:agentId={draft.agentId}
				/>

				<div class="space-y-4">
					<div class="flex items-center justify-between">
						<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
							{m.agent_id_academic_level()}
						</p>
						<span class="font-label text-[10px] text-primary/40 uppercase"
							>{m.common_required()}</span
						>
					</div>
					<div class="grid gap-3 sm:grid-cols-3">
						{#each academicLevels as level (level.value)}
							<label
								class={[
									'flex cursor-pointer items-center justify-between border px-4 py-3 transition-colors',
									draft.agentId.academicLevel === level.value
										? 'border-primary bg-primary/10 text-primary'
										: 'border-outline-variant bg-surface-container-low text-on-surface'
								]}
							>
								<span class="font-label text-[11px] tracking-[0.16em] uppercase">{level.label}</span
								>
								<input
									type="radio"
									name="academicLevel"
									class="sr-only"
									value={level.value}
									bind:group={draft.agentId.academicLevel}
								/>
							</label>
						{/each}
					</div>
				</div>

				{#if showCourseNumber}
					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
								{m.course_number()}
							</p>
							<span class="font-label text-[10px] text-primary/40 uppercase"
								>{m.common_required()}</span
							>
						</div>
						<select
							class="w-full border-0 border-b-2 border-outline-variant bg-transparent px-0 py-3 font-label text-lg tracking-[0.2em] text-primary transition-all focus:border-primary focus:ring-0"
							bind:value={draft.agentId.courseNumber}
						>
							<option value="">Select course</option>
							{#each courseOptions as course (course)}
								<option value={course}>{course}</option>
							{/each}
						</select>
					</div>
				{/if}

				{#if showBachelorTrack}
					<div class="space-y-4">
						<div class="flex items-center justify-between">
							<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
								{m.agent_id_bachelor_track()}
							</p>
							<span class="font-label text-[10px] text-primary/40 uppercase"
								>{m.common_required()}</span
							>
						</div>
						<div class="grid gap-3 sm:grid-cols-3">
							{#each bachelorTracks as track (track.value)}
								<label
									class={[
										'flex cursor-pointer items-center justify-between border px-4 py-3 transition-colors',
										draft.agentId.bachelorTrack === track.value
											? 'border-primary bg-primary/10 text-primary'
											: 'border-outline-variant bg-surface-container-low text-on-surface'
									]}
								>
									<span class="font-label text-[11px] tracking-[0.16em] uppercase"
										>{track.label}</span
									>
									<input
										type="radio"
										name="bachelorTrack"
										class="sr-only"
										value={track.value}
										bind:group={draft.agentId.bachelorTrack}
									/>
								</label>
							{/each}
						</div>
					</div>
				{/if}

				<button
					class={`glitch-burst tactical-button flex w-full items-center justify-center gap-3 px-6 py-5 font-headline font-bold tracking-[0.3em] uppercase transition-[filter,transform,brightness,opacity] ${isAgentIdComplete(draft) ? 'hover:brightness-110 active:scale-[0.98]' : 'pointer-events-none opacity-50 saturate-0'}`}
					type="submit"
					disabled={!isAgentIdComplete(draft)}
				>
					{isSubmitting ? 'Processing' : m.common_proceed()}
					<span class="material-symbols-outlined">arrow_forward</span>
				</button>
			</form>

			<div class="bg-surface-container-lowest p-4">
				<p
					class="flex items-start gap-4 font-label text-[9px] leading-relaxed text-outline uppercase"
				>
					<Icon name="shield" class="text-xl transition-transform group-hover:scale-110" />
					<span>{m.agent_id_falsification_notice()}</span>
				</p>
			</div>
		</section>

		<div class="mt-10 grid gap-4 sm:grid-cols-2">
			<NodeConnectivity>{m.agent_id_stable_encrypted()}</NodeConnectivity>
			<Countdown timeRemaining={new Date(0, 0, 0, 0, 15, 0, 0)}></Countdown>
		</div>
	</div>
</TerminalShell>
