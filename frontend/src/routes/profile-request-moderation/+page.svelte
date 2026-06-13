<script lang="ts">
	import { TerminalShell, ProfileAvatar } from '$lib/shared/ui';
	import { getAppContext } from '$lib/shared/providers';
	import type { ProfileRequest, SessionFlow } from '$lib/shared/model';

	const KNOWN_REQUESTS_KEY = 'known-requests';

	let { data } = $props<{
		data: {
			sessionFlow?: SessionFlow;
		};
	}>();

	const app = getAppContext();
	const flow = $derived(data.sessionFlow);
	const rejected = $derived(
		flow?.allRequests.filter((r: { status: string }) => r.status === 'rejected') ?? []
	);
	const approved = $derived(
		flow?.allRequests.filter((r: { status: string }) => r.status === 'approved') ?? []
	);

	let newlyRejected = $state<string[]>([]);

	const loadKnownRequests = (): Record<string, string> => {
		try {
			return JSON.parse(localStorage.getItem(KNOWN_REQUESTS_KEY) || '{}');
		} catch {
			return {};
		}
	};

	const saveKnownRequests = (requests: ProfileRequest[]) => {
		const map: Record<string, string> = {};
		for (const r of requests) {
			map[r.profile_request_id] = r.status;
		}
		localStorage.setItem(KNOWN_REQUESTS_KEY, JSON.stringify(map));
	};

	let inited = $state(false);

	$effect(() => {
		const all = flow?.allRequests;
		if (!all || all.length === 0) return;
		if (!inited) {
			saveKnownRequests(all);
			inited = true;
			return;
		}

		const known = loadKnownRequests();
		const fresh: string[] = [];

		for (const req of all) {
			const previous = known[req.profile_request_id];
			if (previous && previous !== req.status && req.status === 'rejected') {
				fresh.push(req.profile_request_id);
			}
		}

		if (fresh.length > 0) {
			newlyRejected = [...new Set([...fresh, ...newlyRejected])];
		}

		saveKnownRequests(all);
	});

	const createNewRequest = () => {
		app.navigate('/agent-id?mode=edit');
	};
</script>

<TerminalShell topBar={{ title: 'PROFILE REVIEW', icon: 'rate_review' }}>
	<div class="mx-auto max-w-3xl space-y-8">
		<section class="border-b border-outline-variant/10 pb-6">
			<h1 class="font-headline text-3xl font-bold uppercase">Profile Request Moderation</h1>
			<p class="mt-2 text-sm text-on-surface-variant">
				Review the moderator feedback on your rejected requests and submit a corrected profile.
			</p>
		</section>

		{#each rejected as req (req.profile_request_id)}
			{@const profile = req.requested_profile_data}
			{@const isNew = newlyRejected.includes(req.profile_request_id)}
			{@const sectionClass = isNew
				? 'border-l-4 border-warning bg-warning/10 p-6 transition-all'
				: 'border-l-4 border-error bg-error/5 p-6 transition-all'}
			<section class={sectionClass}>
				<div class="flex items-start justify-between gap-4">
					<div class="min-w-0 flex-1">
						<div class="flex items-center gap-3">
							<div
								class="font-headline text-[10px] tracking-[0.25em] uppercase {isNew
									? 'text-warning'
									: 'text-error'}"
							>
								{isNew ? 'NEW MODERATION RESULT' : 'Rejected'}
							</div>
							<ProfileAvatar {profile} name={profile?.codename ?? 'N/A'} size={40} />
						</div>
						<div class="mt-2 space-y-2">
							<div class="flex flex-wrap gap-x-6 gap-y-1 text-sm">
								<span class="font-headline font-bold text-on-surface uppercase"
									>{profile?.codename ?? 'N/A'}</span
								>
								<span class="text-outline">{profile?.academicGroup ?? ''}</span>
							</div>
						</div>
						{#if req.reviewer_note}
							<div class="mt-4 rounded bg-surface-container-low p-4">
								<div class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
									Moderator note
								</div>
								<p class="mt-1 text-sm text-on-surface-variant italic">
									"{req.reviewer_note}"
								</p>
							</div>
						{/if}
						{#if isNew}
							<button
								onclick={createNewRequest}
								class="glitch-burst tactical-button mt-6 flex w-full items-center justify-center gap-3 px-6 py-4 font-headline text-sm font-bold tracking-[0.3em] uppercase transition-[filter,transform,brightness,opacity] hover:brightness-110 active:scale-[0.98]"
							>
								Continue to profile recreation
								<span class="material-symbols-outlined">arrow_forward</span>
							</button>
						{/if}
					</div>
				</div>
			</section>
		{/each}

		{#if approved.length > 0}
			{@const current = approved[approved.length - 1]}
			{@const currentProfile = current.requested_profile_data}
			<section class="border-l-4 border-secondary bg-secondary/5 p-6">
				<div class="flex items-center gap-3">
					<div class="font-headline text-[10px] tracking-[0.25em] text-secondary uppercase">
						Active operative profile
					</div>
					<ProfileAvatar
						profile={currentProfile}
						name={currentProfile?.codename ?? 'N/A'}
						size={40}
					/>
				</div>
				<div class="mt-2">
					<span class="font-headline font-bold text-on-surface uppercase"
						>{currentProfile?.codename ?? 'N/A'}</span
					>
				</div>
			</section>
		{/if}

		<section class="bg-surface-container-low p-6">
			<h2 class="font-headline text-sm font-bold uppercase">Submit a new request</h2>
			<p class="mt-2 text-sm text-on-surface-variant">
				Create a corrected agent profile and submit it for review again. Your active gameplay access
				remains until the new request is processed.
			</p>
			<button
				onclick={createNewRequest}
				class="glitch-burst tactical-button mt-6 flex w-full items-center justify-center gap-3 px-6 py-5 font-headline font-bold tracking-[0.3em] uppercase transition-[filter,transform,brightness,opacity] hover:brightness-110 active:scale-[0.98]"
			>
				Create new profile request
				<span class="material-symbols-outlined">add_circle</span>
			</button>
		</section>
	</div>
</TerminalShell>
