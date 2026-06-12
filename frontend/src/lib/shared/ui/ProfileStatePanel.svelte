<script lang="ts">
	type Tone = 'primary' | 'secondary' | 'error';

	let {
		kicker,
		title,
		body,
		icon = 'security',
		note = null,
		href = null,
		ctaLabel = null,
		tone = 'primary'
	}: {
		kicker: string;
		title: string;
		body: string;
		icon?: string;
		note?: string | null;
		href?: string | null;
		ctaLabel?: string | null;
		tone?: Tone;
	} = $props();

	const toneClassMap: Record<Tone, string> = {
		primary: 'border-primary/40 bg-surface-container-low text-primary',
		secondary: 'border-secondary/40 bg-surface-container-low text-secondary',
		error: 'border-error/40 bg-surface-container-low text-error'
	};

	let toneClass = $derived(toneClassMap[tone]);
</script>

<section class="scan-sweep relative overflow-hidden border bg-surface-container p-8">
	<div class="absolute inset-0 opacity-10">
		<div class="scanning-grid size-full"></div>
	</div>

	<div class="relative z-10 flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
		<div class="max-w-2xl">
			<div
				class={`mb-4 inline-flex items-center gap-2 border px-3 py-1 font-label text-[10px] font-bold tracking-[0.28em] uppercase ${toneClass}`}
			>
				<span class="signal-dot size-2 bg-current"></span>
				<span>{kicker}</span>
			</div>
			<h2 class="font-headline text-4xl font-extrabold tracking-tight sm:text-5xl">{title}</h2>
			<p class="mt-4 max-w-xl text-sm leading-relaxed text-on-surface-variant">{body}</p>

			{#if note}
				<div class="mt-6 border-l-4 border-outline-variant/40 bg-surface-container-high p-4">
					<div class="font-label text-[10px] tracking-[0.24em] text-outline uppercase">
						Reviewer note
					</div>
					<p class="mt-2 text-sm leading-relaxed text-on-surface">{note}</p>
				</div>
			{/if}
		</div>

		<div class="flex w-full max-w-xs flex-col gap-3 md:items-end">
			<div
				class="flex size-16 items-center justify-center bg-surface-container-high text-4xl text-primary"
			>
				<span class="material-symbols-outlined" style="font-variation-settings:'FILL' 1"
					>{icon}</span
				>
			</div>

			{#if href && ctaLabel}
				<a
					{href}
					class="glitch-burst tactical-button flex w-full items-center justify-between px-6 py-4 font-headline text-xs font-bold tracking-[0.24em] uppercase transition-[filter,transform,brightness] hover:brightness-110"
				>
					<span>{ctaLabel}</span>
					<span class="material-symbols-outlined">arrow_forward</span>
				</a>
			{/if}
		</div>
	</div>
</section>
