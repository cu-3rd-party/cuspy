<script lang="ts">
	import { ProfileStatePanel } from '$lib/shared/ui';
	import { TerminalShell } from '$lib/shared/ui';
	import { agentAvatar } from '$lib/shared/config';
	import { getAppContext } from '$lib/shared/providers';
	import { deleteProfileRequest } from '$lib/shared/api';
	import type { ProfileRequest, SessionFlow } from '$lib/shared/model';

	const KNOWN_REQUESTS_KEY = 'known-requests';

	let { data } = $props<{
		data: {
			sessionFlow: SessionFlow;
		};
	}>();

	const app = getAppContext();
	let flow = $derived(data.sessionFlow);
	let codename = $derived(
		(flow.user?.agent_data?.codename as string | undefined) ?? flow.user?.agent_name ?? 'AGENT'
	);
	let submittedAt = $derived(
		flow.latestProfileRequest?.created_at
			? new Date(Number(flow.latestProfileRequest.created_at) * 1000).toLocaleString()
			: 'Awaiting timestamp'
	);

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
		let needsNavigate = false;

		for (const req of all) {
			const previous = known[req.profile_request_id];
			if (previous && previous !== req.status) {
				if (req.status === 'rejected') {
					app.navigate('/profile-request-moderation');
					return;
				}
				if (req.status === 'approved') {
					app.navigate('/dossier');
					return;
				}
				needsNavigate = true;
			}
		}

		if (needsNavigate) {
			app.navigate('/profile-request-moderation');
		}

		saveKnownRequests(all);
	});

	const handlePatchDraft = async () => {
		if (flow?.latestProfileRequest?.profile_request_id) {
			try {
				await deleteProfileRequest(flow.latestProfileRequest.profile_request_id);
			} catch {
				// Proceed to editor even if delete fails
			}
		}
		app.navigate('/agent-id');
	};

	const quickLinks = [
		{
			href: '/target-intel' as const,
			label: 'Open target intel',
			copy: 'Gameplay unlocked while command reviews profile packet.'
		},
		{
			href: '/rankings' as const,
			label: 'Monitor rankings',
			copy: 'Track live ladder movement during review window.'
		},
		{
			href: null as string | null,
			action: handlePatchDraft,
			label: 'Patch draft locally',
			copy: 'Prepare corrected packet before moderator feedback arrives.'
		}
	];
</script>

<TerminalShell topBar={{ title: 'CLEARANCE QUEUE', icon: 'hourglass_top', avatar: agentAvatar }}>
	<div class="mx-auto max-w-5xl space-y-8">
		<section class="border-b border-outline-variant/10 pb-8">
			<div class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
				<div>
					<div class="font-headline text-xs tracking-[0.3em] text-primary/60 uppercase">
						Review queue live
					</div>
					<h1 class="mt-2 font-headline text-5xl font-bold tracking-tight sm:text-7xl">
						{codename}
					</h1>
				</div>
				<div class="bg-surface-container px-4 py-3 text-right">
					<div class="font-label text-[10px] tracking-[0.24em] text-outline uppercase">
						Submitted
					</div>
					<div class="mt-2 font-headline text-sm font-bold text-secondary">{submittedAt}</div>
				</div>
			</div>
		</section>

		<ProfileStatePanel
			kicker="Pending moderator action"
			title="Dossier in command queue"
			body={flow.canPlay
				? 'Telegram will deliver next state change. Until then, use active gameplay routes normally. Rejection returns packet for edits. Confirmation promotes full operative status automatically.'
				: 'Telegram will deliver next state change. Gameplay access is granted once a moderator confirms your profile. Check back after review.'}
			icon="notifications_active"
			href={flow.canPlay ? '/target-intel' : null}
			ctaLabel={flow.canPlay ? 'Continue to game' : null}
			tone="secondary"
		/>

		<section class="grid gap-4 md:grid-cols-3">
			<div class="border-l-4 border-primary bg-surface-container p-6">
				<div class="font-headline text-[10px] tracking-[0.2em] text-primary uppercase">
					Queue state
				</div>
				<p class="mt-3 font-headline text-2xl font-bold uppercase">under review</p>
			</div>
			<div
				class="border-l-4 p-6"
				class:border-secondary={flow.canPlay}
				class:border-outline={!flow.canPlay}
				class:bg-surface-container={flow.canPlay}
				class:bg-surface-container-low={!flow.canPlay}
			>
				<div
					class="font-headline text-[10px] tracking-[0.2em] uppercase"
					class:text-secondary={flow.canPlay}
					class:text-outline={!flow.canPlay}
				>
					Gameplay access
				</div>
				<p
					class="mt-3 font-headline text-2xl font-bold uppercase"
					class:text-secondary={flow.canPlay}
					class:text-outline={!flow.canPlay}
				>
					{flow.canPlay ? 'enabled' : 'locked'}
				</p>
			</div>
			<div class="border-l-4 border-outline bg-surface-container p-6">
				<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">
					Delivery channel
				</div>
				<p class="mt-3 font-headline text-2xl font-bold uppercase">telegram</p>
			</div>
		</section>

		<section class="grid gap-4 md:grid-cols-3">
			{#each quickLinks as item (item.label)}
				{#if item.href}
					<a
						href={item.href}
						class="bg-surface-container-low p-6 transition-colors hover:bg-surface-container"
					>
						<div class="font-headline text-sm font-bold uppercase">{item.label}</div>
						<p class="mt-3 text-sm leading-relaxed text-on-surface-variant">{item.copy}</p>
					</a>
				{:else if item.action}
					<button
						type="button"
						onclick={item.action}
						class="w-full bg-surface-container-low p-6 text-left transition-colors hover:bg-surface-container"
					>
						<div class="font-headline text-sm font-bold uppercase">{item.label}</div>
						<p class="mt-3 text-sm leading-relaxed text-on-surface-variant">{item.copy}</p>
					</button>
				{/if}
			{/each}
		</section>
	</div>
</TerminalShell>
