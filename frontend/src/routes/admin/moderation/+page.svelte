<script lang="ts">
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/TopBar.svelte';
	import type { ProfileRequest, SessionFlow } from '$lib/stores/session';
	import type { ActionData, PageProps } from './$types';

	type QueueRequest = ProfileRequest & {
		requested_profile_data: ProfileRequest['requested_profile_data'] & {
			codename?: string;
			academicGroup?: string;
			identificationName?: string;
		};
	};

	let { data, form }: PageProps = $props();
	let actionState = $derived(form as ActionData | null);
	let requests = $derived(data.requests as QueueRequest[]);
	let activeId = $state('');
	let reviewerNote = $state('');
	let activeRequest = $derived(
		requests.find((request) => request.profile_request_id === activeId) ?? requests[0] ?? null
	);
	let pendingCount = $derived(requests.filter((request) => request.status === 'sent').length);
	let resolvedCount = $derived(requests.filter((request) => request.status !== 'sent').length);

	$effect(() => {
		if (!activeId && requests[0]) {
			activeId = requests[0].profile_request_id;
			reviewerNote = requests[0].reviewer_note ?? '';
		}
	});

	const selectRequest = (requestId: string, note: string | null) => {
		activeId = requestId;
		reviewerNote = note ?? '';
	};

	const formatDate = (unixSeconds: string) => new Date(Number(unixSeconds) * 1000).toLocaleString();
</script>

<TopBar config={{ title: 'COMMAND_CENTER // MODERATION', icon: 'verified_user' }} flow={data.sessionFlow as SessionFlow} />

<main class="technical-grid min-h-screen p-8 pt-24">
	<header class="mb-12 border-l-4 border-primary pl-6">
		<h2 class="text-4xl font-display font-black tracking-tighter uppercase">PROFILE_CLEARANCE_QUEUE</h2>
		<p class="font-label text-sm tracking-[0.3em] text-on-surface-variant uppercase">REAL-TIME DOSSIER REVIEW</p>
	</header>

	<div class="mb-8 grid gap-4 md:grid-cols-4">
		<div class="border-l-2 border-primary bg-surface-container p-4">
			<div class="text-xs font-label uppercase">Pending Requests</div>
			<div class="text-3xl font-display font-bold text-primary">{pendingCount}</div>
		</div>
		<div class="border-l-2 border-secondary bg-surface-container p-4">
			<div class="text-xs font-label uppercase">Reviewed</div>
			<div class="text-3xl font-display font-bold text-secondary">{resolvedCount}</div>
		</div>
	</div>

	{#if actionState?.error}
		<div class="mb-6 bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase">
			{actionState.error}
		</div>
	{/if}

	{#if requests.length === 0}
		<section class="border border-outline-variant/15 bg-surface-container-lowest p-8">
			<h3 class="font-display text-2xl font-bold uppercase">NO_PENDING_PROFILES</h3>
			<p class="mt-4 max-w-xl text-sm text-on-surface-variant">Queue clear. New profile requests will appear here for approval or rejection.</p>
		</section>
	{:else}
		<div class="grid gap-8 lg:grid-cols-12">
			<section class="space-y-4 lg:col-span-4">
				{#each requests as request (request.profile_request_id)}
					<button
						type="button"
						onclick={() => selectRequest(request.profile_request_id, request.reviewer_note)}
						class={`w-full border p-4 text-left transition-colors ${request.profile_request_id === activeId ? 'border-primary bg-surface-container text-on-surface' : 'border-outline-variant/20 bg-surface-container-lowest hover:border-primary/40 hover:bg-surface-container-low'}`}
					>
						<div class="flex items-start justify-between gap-4">
							<div>
								<div class="font-display text-lg font-bold uppercase">
									{request.requested_profile_data.codename ?? 'UNNAMED_AGENT'}
								</div>
								<div class="mt-2 text-[10px] font-label tracking-[0.25em] text-on-surface-variant uppercase">
									{request.requested_profile_data.academicGroup ?? 'GROUP_UNSET'}
								</div>
							</div>
							<span class={`px-2 py-1 text-[10px] font-label uppercase ${request.status === 'sent' ? 'bg-primary-container text-on-primary-container' : request.status === 'confirmed' ? 'bg-secondary-container text-on-secondary-container' : 'bg-error text-white'}`}>
								{request.status}
							</span>
						</div>
						<div class="mt-4 text-[10px] font-label uppercase text-on-surface-variant">
							{formatDate(request.created_at)}
						</div>
					</button>
				{/each}
			</section>

			{#if activeRequest}
				<section class="border border-outline-variant/15 bg-surface-container-lowest p-6 md:p-8 lg:col-span-8">
					<div class="mb-8 flex flex-wrap items-start justify-between gap-4">
						<div>
							<h3 class="text-2xl font-display font-bold uppercase">{activeRequest.requested_profile_data.codename ?? 'UNNAMED_AGENT'}</h3>
							<p class="mt-2 text-[10px] font-label tracking-[0.3em] text-on-surface-variant uppercase">REQUEST_ID: {activeRequest.profile_request_id}</p>
						</div>
						<div class="text-right text-[10px] font-label uppercase text-on-surface-variant">
							<div>SUBMITTED: {formatDate(activeRequest.created_at)}</div>
							{#if activeRequest.reviewed_at}
								<div class="mt-2">REVIEWED: {formatDate(activeRequest.reviewed_at)}</div>
							{/if}
						</div>
					</div>

					<div class="grid gap-4 md:grid-cols-2">
						<div class="bg-surface-container p-4">
							<div class="text-[10px] font-label tracking-[0.2em] text-primary uppercase">Academic Group</div>
							<div class="mt-2 font-display text-lg font-bold uppercase">{activeRequest.requested_profile_data.academicGroup ?? 'UNSET'}</div>
						</div>
						<div class="bg-surface-container p-4">
							<div class="text-[10px] font-label tracking-[0.2em] text-primary uppercase">Identification</div>
							<div class="mt-2 font-display text-lg font-bold uppercase">{activeRequest.requested_profile_data.identificationName ?? 'MISSING'}</div>
						</div>
					</div>

					<div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,1fr)_20rem]">
						<div class="space-y-4">
							<div class="bg-surface-container p-4">
								<div class="mb-3 text-[10px] font-label tracking-[0.2em] text-primary uppercase">Submitted Profile Data</div>
								<pre class="overflow-x-auto text-xs leading-relaxed text-on-surface-variant">{JSON.stringify(activeRequest.requested_profile_data, null, 2)}</pre>
							</div>
							{#if activeRequest.requested_profile_data.identificationImage}
								<div class="overflow-hidden bg-surface-container">
									<img src={activeRequest.requested_profile_data.identificationImage} alt="Submitted identification" class="max-h-[28rem] w-full object-cover" />
								</div>
							{/if}
						</div>

						<form method="POST" action="?/moderate" use:enhance class="space-y-4 bg-surface-container p-4">
							<input type="hidden" name="requestId" value={activeRequest.profile_request_id} />
							<div>
								<label for="reviewer-note" class="mb-3 block text-[10px] font-label tracking-[0.2em] text-on-surface-variant uppercase">Reviewer Note</label>
								<textarea id="reviewer-note" name="reviewerNote" bind:value={reviewerNote} class="min-h-40 w-full bg-surface-container-lowest p-3 text-sm text-on-surface focus:outline-none" placeholder="Explain approval context or rejection reason."></textarea>
							</div>

							<div class="space-y-3 pt-2">
								<button name="decision" value="confirmed" class="w-full bg-primary-container py-4 font-bold tracking-[0.2em] text-on-primary-container uppercase transition-colors hover:bg-primary">
									APPROVE_PROFILE
								</button>
								<button name="decision" value="rejected" class="w-full border-2 border-error-container py-4 font-bold tracking-[0.2em] text-error uppercase transition-colors hover:bg-error hover:text-white">
									REJECT_PROFILE
								</button>
							</div>
						</form>
					</div>
				</section>
			{/if}
		</div>
	{/if}
</main>
