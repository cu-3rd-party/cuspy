<script lang="ts">
	import { resolve } from '$app/paths';
	import ProfileStatePanel from '$lib/components/ProfileStatePanel.svelte';
	import TerminalShell from '$lib/components/TerminalShell.svelte';
	import { agentAvatar, gameplayNav } from '$lib/prototype/data';
	import type { SessionFlow } from '$lib/stores/session';

	let { data } = $props<{
		data: {
			sessionFlow: SessionFlow;
		};
	}>();

	let flow = $derived(data.sessionFlow);
	let codename = $derived(
		(flow.user?.agent_data?.codename as string | undefined) ?? flow.user?.agent_name ?? 'AGENT'
	);
	let submittedAt = $derived(
		flow.latestProfileRequest?.created_at
			? new Date(Number(flow.latestProfileRequest.created_at) * 1000).toLocaleString()
			: 'Awaiting timestamp'
	);
	const quickLinks = [
		{
			href: '/target-intel',
			label: 'Open target intel',
			copy: 'Gameplay unlocked while command reviews profile packet.'
		},
		{
			href: '/rankings',
			label: 'Monitor rankings',
			copy: 'Track live ladder movement during review window.'
		},
		{
			href: '/agent-id?mode=edit',
			label: 'Patch draft locally',
			copy: 'Prepare corrected packet before moderator feedback arrives.'
		}
	];
</script>

<TerminalShell
	topBar={{ title: 'CLEARANCE QUEUE', icon: 'hourglass_top', avatar: agentAvatar }}
	nav={gameplayNav}
>
	<div class="mx-auto max-w-5xl space-y-8">
		<section class="border-b border-outline-variant/10 pb-8">
			<div class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
				<div>
					<div class="font-headline text-xs tracking-[0.3em] text-primary/60 uppercase">Review queue live</div>
					<h1 class="mt-2 font-headline text-5xl font-bold tracking-tight sm:text-7xl">{codename}</h1>
					<p class="mt-3 max-w-2xl text-sm leading-relaxed text-on-surface-variant">
						Profile sent. Moderator confirmation still pending. Field access stays live, so user can keep playing while queue advances.
					</p>
				</div>
				<div class="bg-surface-container px-4 py-3 text-right">
					<div class="font-label text-[10px] tracking-[0.24em] text-outline uppercase">Submitted</div>
					<div class="mt-2 font-headline text-sm font-bold text-secondary">{submittedAt}</div>
				</div>
			</div>
		</section>

		<ProfileStatePanel
			kicker="Pending moderator action"
			title="Dossier in command queue"
			body="Telegram will deliver next state change. Until then, use active gameplay routes normally. Rejection returns packet for edits. Confirmation promotes full operative status automatically."
			icon="notifications_active"
			href="/target-intel"
			ctaLabel="Continue to game"
			tone="secondary"
		/>

		<section class="grid gap-4 md:grid-cols-3">
			<div class="border-l-4 border-primary bg-surface-container p-6">
				<div class="font-headline text-[10px] tracking-[0.2em] text-primary uppercase">Queue state</div>
				<p class="mt-3 font-headline text-2xl font-bold uppercase">under review</p>
			</div>
			<div class="border-l-4 border-secondary bg-surface-container p-6">
				<div class="font-headline text-[10px] tracking-[0.2em] text-secondary uppercase">Gameplay access</div>
				<p class="mt-3 font-headline text-2xl font-bold uppercase">enabled</p>
			</div>
			<div class="border-l-4 border-outline bg-surface-container p-6">
				<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">Delivery channel</div>
				<p class="mt-3 font-headline text-2xl font-bold uppercase">telegram</p>
			</div>
		</section>

		<section class="grid gap-4 md:grid-cols-3">
			{#each quickLinks as item (item.href)}
				<a href={resolve(item.href)} class="bg-surface-container-low p-6 transition-colors hover:bg-surface-container">
					<div class="font-headline text-sm font-bold uppercase">{item.label}</div>
					<p class="mt-3 text-sm leading-relaxed text-on-surface-variant">{item.copy}</p>
				</a>
			{/each}
		</section>
	</div>
</TerminalShell>
