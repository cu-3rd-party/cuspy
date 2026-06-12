<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
	import { TerminalShell } from '$lib/shared/ui';
	import { agentAvatar, gameplayNav, targetImage } from '$lib/shared/config';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
</script>

<TerminalShell
	topBar={{ title: m.target_intel_topbar_title(), icon: 'security', avatar: agentAvatar }}
	nav={gameplayNav}
>
	<div class="scanning-grid pointer-events-none fixed inset-0 opacity-5"></div>
	<div class="relative mx-auto max-w-2xl">
		<div class="mb-10">
			<span class="font-label text-[10px] tracking-[0.3em] text-primary uppercase"
				>{m.target_intel_status_in_progress()}</span
			>
			<h2 class="mt-1 font-headline text-4xl font-bold tracking-tight uppercase">
				{m.target_intel_current_objective()}
			</h2>
			<div class="mt-2 h-1 w-12 bg-primary"></div>
		</div>

		<div
			class="scan-sweep relative mb-8 overflow-hidden border-l-4 border-primary-container bg-surface-container-low"
		>
			<div class="absolute inset-x-0 top-0 h-0.5 bg-primary/30"></div>
			<div class="flex flex-col md:flex-row">
				<div
					class="relative h-64 w-full overflow-hidden bg-surface-container-highest md:h-auto md:w-48"
				>
					<img
						src={targetImage}
						alt={m.target_intel_image_alt()}
						class="size-full object-cover contrast-125 grayscale"
					/>
					<div class="absolute inset-0 bg-primary-container/20 mix-blend-overlay"></div>
					<div class="absolute bottom-2 left-2 flex items-center gap-2 bg-black/80 px-2 py-1">
						<span class="signal-dot size-1.5 rounded-full bg-error"></span>
						<span class="font-label text-[10px] tracking-[0.2em] text-white uppercase"
							>{m.target_intel_target_live()}</span
						>
					</div>
				</div>
				<div class="flex-1 p-6">
					<div class="grid gap-6">
						<div>
							<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase"
								>{m.target_intel_subject_identifier()}</span
							>
							<div class="font-headline text-2xl font-bold tracking-tight uppercase">
								{m.target_intel_agent_raven()}
							</div>
						</div>
						<div class="grid gap-4 sm:grid-cols-2">
							<div>
								<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase"
									>{m.target_intel_operational_group()}</span
								>
								<div class="text-sm font-medium text-primary">B21-DS-01</div>
							</div>
							<div>
								<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase"
									>{m.target_intel_last_sighting()}</span
								>
								<div class="text-sm font-medium">{m.target_intel_campus_central()}</div>
							</div>
						</div>
						<div
							class="border-t border-outline-variant/30 pt-4 font-label text-[10px] text-outline uppercase"
						>
							<div class="mb-2 flex items-center gap-2">
								<span class="material-symbols-outlined text-[14px] text-primary">fingerprint</span
								>{m.target_intel_biometric_match()}
							</div>
							<div class="flex items-center gap-2">
								<span class="material-symbols-outlined text-[14px] text-primary">distance</span
								>{m.target_intel_proximity()}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="space-y-6">
			<a
				href={resolve('/reveal-confirmation')}
				class="press-down group relative flex items-center justify-between bg-primary-container px-8 py-6 font-headline text-xl font-extrabold tracking-[0.2em] text-black uppercase transition-all hover:bg-primary"
			>
				<span>{m.target_intel_report_discovery()}</span>
				<span class="material-symbols-outlined group-active-slide text-3xl">target</span>
			</a>

			<div class="bg-surface-container p-4">
				<div class="flex gap-3">
					<span class="material-symbols-outlined text-xl text-error">warning</span>
					<div>
						<p class="font-label text-[11px] font-bold text-error uppercase">
							{m.target_intel_no_witnesses()}
						</p>
						<p class="mt-1 text-xs leading-relaxed text-on-surface-variant">
							{m.target_intel_no_witnesses_copy()}
						</p>
					</div>
				</div>
				<div class="mt-3 border-t border-outline-variant/20 pt-3">
					<div class="flex gap-3">
						<span class="material-symbols-outlined text-xl text-outline">gavel</span>
						<div>
							<p class="font-label text-[11px] font-bold uppercase">
								{m.target_intel_respect_boundaries()}
							</p>
							<p class="mt-1 text-xs leading-relaxed text-on-surface-variant">
								{m.target_intel_respect_boundaries_copy()}
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div
			class="mt-12 space-y-2 font-label text-[9px] tracking-[0.2em] text-on-surface/40 uppercase"
		>
			<div class="flex justify-between">
				<span>{m.target_intel_log_handshake()}</span><span>{m.target_intel_log_ok()}</span>
			</div>
			<div class="flex justify-between">
				<span>{m.target_intel_log_scanning()}</span><span>{m.target_intel_log_complete()}</span>
			</div>
			<div class="flex justify-between">
				<span>{m.target_intel_log_location()}</span><span>12:44:02</span>
			</div>
		</div>
	</div>
</TerminalShell>
