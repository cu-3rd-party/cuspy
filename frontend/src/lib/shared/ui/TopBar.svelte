<script lang="ts">
	import Icon from './Icon.svelte';
	import ProfileAvatar from './ProfileAvatar.svelte';
	import type { TopBarConfig } from '$lib/shared/config';
	import type { SessionFlow } from '$lib/shared/model';
	import { deriveRequestState } from '$lib/shared/model';
	import { getAppContext } from '$lib/shared/providers';
	import { getLocale, setLocale, type Locale } from '$lib/paraglide/runtime.js';

	let { config, flow }: { config: TopBarConfig; flow?: SessionFlow } = $props();
	let app = getAppContext();
	let codename = $derived(
		(flow?.user?.agent_data?.codename as string | undefined) ??
			flow?.user?.agent_name ??
			flow?.status ??
			'undef'
	);
	let status = $derived(
		flow?.status === 'approved'
			? 'OPERATIVE ACTIVE'
			: flow?.status === 'pending'
				? 'WAITING CLEARANCE'
				: flow?.status === 'rejected'
					? 'REVISION REQUIRED'
					: flow?.status === 'no_profile'
						? 'PROFILE REQUIRED'
						: 'GUEST SESSION'
	);

	let requestState = $derived(flow ? deriveRequestState(flow) : null);
	let dropdownOpen = $state(false);

	let pendingRequests = $derived(flow?.allRequests.filter((r) => r.status === 'pending') ?? []);
	let currentLocale = $state(getLocale());
	let languageOptions: Array<{ value: Locale; label: string }> = [
		{ value: 'en', label: 'EN' },
		{ value: 'ru', label: 'RU' }
	];

	const toggleDropdown = () => {
		dropdownOpen = !dropdownOpen;
	};

	const switchLanguage = (locale: Locale) => {
		if (locale === currentLocale) return;
		currentLocale = locale;
		void setLocale(locale);
	};
</script>

<header
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-between bg-surface-container-lowest px-6 py-4 text-primary shadow-[0_0_15px_rgba(0,122,27,0.1)]"
>
	<div class="flex items-center gap-3">
		{#if config.backHref}
			<a
				href={config.backHref}
				class="rounded-full p-2 text-outline transition-colors hover:bg-surface-container-low hover:text-primary"
			>
				<Icon name="arrow_back" />
			</a>
		{/if}
		<a href="/" class="group flex items-center gap-3">
			<Icon name="terminal" class="text-xl transition-transform group-hover:scale-110" />
			<h1 class="font-headline text-sm font-bold tracking-[0.2em] uppercase sm:text-base">
				{config.title || 'CUKILLER // PROTOCOL'}
			</h1>
		</a>
	</div>

	<div class="flex items-center gap-4">
		{#if requestState?.type === 'approved-pending'}
			<div
				class="flex items-center gap-2 rounded bg-primary/10 px-3 py-1 font-label text-[10px] tracking-[0.2em] text-primary uppercase"
			>
				<span class="size-1.5 rounded-full bg-primary"></span>
				{pendingRequests.length} pending
			</div>
		{:else if requestState?.type === 'approved-rejected'}
			<button
				onclick={() => app.navigate('/profile-request-moderation')}
				class="flex cursor-pointer items-center gap-2 rounded bg-error/20 px-3 py-1 font-label text-[10px] font-bold tracking-[0.2em] text-error uppercase transition-colors hover:bg-error/30"
			>
				<Icon name="warning" class="text-sm" />
				Rejected request
			</button>
		{:else if requestState?.type === 'approved-multiple'}
			<div class="relative">
				<button
					onclick={toggleDropdown}
					class="flex cursor-pointer items-center gap-2 rounded bg-surface-container px-3 py-1 font-label text-[10px] tracking-[0.2em] text-on-surface uppercase transition-colors hover:bg-surface-container-high"
				>
					{flow!.allRequests.length} requests
					<Icon name={dropdownOpen ? 'arrow_drop_up' : 'arrow_drop_down'} class="text-sm" />
				</button>
				{#if dropdownOpen}
					<div
						class="absolute top-full right-0 z-50 mt-2 w-72 bg-surface-container-low shadow-xl"
						role="menu"
						tabindex="-1"
						onclick={() => (dropdownOpen = false)}
						onkeydown={() => {}}
					>
						{#each flow!.allRequests as req (req.profile_request_id)}
							{@const profile = req.requested_profile_data}
							{@const nameVal = profile?.codename ?? 'N/A'}
							<div
								class="flex items-center gap-3 border-b border-outline-variant/10 px-4 py-3"
								role="menuitem"
							>
								<ProfileAvatar {profile} name={nameVal} size={32} />
								<div class="min-w-0 flex-1">
									<div class="truncate font-label text-xs font-bold uppercase">{nameVal}</div>
									<div
										class="font-label text-[9px] tracking-[0.15em] uppercase"
										class:text-secondary={req.status === 'approved'}
										class:text-warning={req.status === 'rejected'}
										class:text-outline={req.status === 'pending'}
									>
										{req.status}
										{#if req.reviewer_note}
											— {req.reviewer_note}
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/if}

		<div class="flex items-center gap-1 bg-background/70 p-1">
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

		<div class="hidden min-w-0 md:block">
			<div class="font-headline text-[10px] tracking-[0.2em] text-outline uppercase">{status}</div>
			<div class="truncate font-headline text-sm font-bold text-on-surface uppercase">
				{codename}
			</div>
		</div>
	</div>
</header>
