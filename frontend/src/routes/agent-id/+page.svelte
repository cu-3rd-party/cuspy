<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';
	import { m } from '$lib/paraglide/messages.js';
	import {
		isAgentIdComplete,
		loadDossierDraft,
		saveDossierDraft,
		type DossierDraft
	} from '$lib/prototype/dossierDraft';
	import ProgressBar from '$lib/components/ProgressBar.svelte';
	import TerminalShell from '$lib/components/TerminalShell.svelte';
	import { enlistNav } from '$lib/prototype/data';
	import Countdown from '$lib/components/Countdown.svelte';
	import NodeConnectivity from '$lib/components/NodeConnectivity.svelte';

	let draft = $state<DossierDraft>(loadDossierDraft());
	let uploadError = $state(false);
	let invalidUploadMessage = $state('');
	let popupTimeout: number | undefined;

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

	onMount(() => {
		draft = loadDossierDraft();

		return () => {
			if (popupTimeout) {
				window.clearTimeout(popupTimeout);
			}
		};
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

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault();

		if (!isAgentIdComplete(draft)) {
			return;
		}

		draft.unlockedStep = Math.max(draft.unlockedStep, 2) as DossierDraft['unlockedStep'];
		await goto(resolve('/operational-boundaries'));
	};
</script>

<TerminalShell topBar={{ title: m.home_topbar_title(), icon: 'terminal' }} nav={enlistNav}>
	<div class="mx-auto max-w-xl">
		{#if invalidUploadMessage}
			<div
				class="fixed top-24 right-4 left-4 z-[60] mx-auto max-w-sm bg-error px-4 py-3 text-center font-label text-[11px] font-bold tracking-[0.2em] text-white uppercase shadow-[0_0_24px_rgba(186,26,26,0.45)] sm:left-auto"
			>
				{invalidUploadMessage}
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
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
							{m.agent_id_codename_label()}
						</p>
						<span class="font-label text-[10px] text-primary/40 uppercase"
							>{m.common_required()}</span
						>
					</div>
					<input
						class="w-full border-0 border-b-2 border-outline-variant bg-transparent px-0 py-3 font-label text-lg tracking-[0.2em] text-primary transition-all placeholder:text-outline/30 focus:border-primary focus:ring-0"
						placeholder={m.agent_id_codename_placeholder()}
						bind:value={draft.agentId.codename}
					/>
				</div>

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
						{#each academicLevels as level}
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
							{#each courseOptions as course}
								<option value={course}>{course}</option>
							{/each}
						</select>
					</div>
				{/if}

				{#if showBachelorTrack}
					<div class="space-y-4">
						<div class="flex items-center justify-between">
							<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
								{m.bachelor_track()}
							</p>
							<span class="font-label text-[10px] text-primary/40 uppercase"
								>{m.common_required()}</span
							>
						</div>
						<div class="grid gap-3 sm:grid-cols-3">
							{#each bachelorTracks as track}
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

				<div class="space-y-4">
					<p class="font-label text-xs tracking-[0.25em] text-on-surface-variant uppercase">
						{m.agent_id_upload_label()}
					</p>
					<div
						class={`group relative flex aspect-[4/3] w-full cursor-pointer flex-col items-center justify-center overflow-hidden border-2 border-dashed bg-surface-container-high transition-colors hover:bg-surface-container-highest ${uploadError ? 'border-error' : 'border-outline-variant'}`}
					>
						{#if draft.agentId.identificationImage}
							<img
								src={draft.agentId.identificationImage}
								alt={draft.agentId.identificationName}
								class="absolute inset-0 size-full object-cover object-center"
							/>
							<div class="absolute inset-0 bg-black/25"></div>
						{/if}

						{#if uploadError}
							<div
								class="pointer-events-none absolute inset-3 animate-[spin_3s_linear_infinite] rounded-full border border-error/60"
							></div>
							<div
								class="pointer-events-none absolute inset-6 animate-[spin_1.8s_linear_infinite] rounded-full border-2 border-transparent border-t-error"
							></div>
						{/if}

						<span
							class={`material-symbols-outlined mb-2 text-4xl transition-colors group-hover:text-primary ${draft.agentId.identificationImage ? 'relative z-10 text-white' : 'text-outline'}`}
							>add_a_photo</span
						>
						<p
							class={`font-label text-[10px] tracking-tight uppercase transition-colors group-hover:text-on-surface ${draft.agentId.identificationImage ? 'relative z-10 text-white' : 'text-outline'}`}
						>
							{m.agent_id_upload_hint()}
						</p>
						<input
							type="file"
							class="absolute inset-0 opacity-0"
							onchange={handleIdentificationChange}
						/>
					</div>
					{#if draft.agentId.identificationName}
						<p class="font-label text-[10px] text-primary uppercase">
							{draft.agentId.identificationName}
						</p>
					{/if}
				</div>

				<button
					class={`glitch-burst tactical-button flex w-full items-center justify-center gap-3 px-6 py-5 font-headline font-bold tracking-[0.3em] uppercase transition-[filter,transform,brightness,opacity] ${isAgentIdComplete(draft) ? 'hover:brightness-110 active:scale-[0.98]' : 'pointer-events-none opacity-50 saturate-0'}`}
					type="submit"
					disabled={!isAgentIdComplete(draft)}
				>
					{m.common_proceed()}
					<span class="material-symbols-outlined">arrow_forward</span>
				</button>
			</form>

			<div class="bg-surface-container-lowest p-4">
				<p
					class="flex items-start gap-4 font-label text-[9px] leading-relaxed text-outline uppercase"
				>
					<span class="signal-dot mt-1 size-2 bg-primary"></span>
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
