<script lang="ts">
	import { resolve } from '$app/paths';
	import { TopBar } from '$lib/shared/ui';
	import { Icon } from '$lib/shared/ui';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
</script>

<TopBar config={{ title: 'MISSIONS', icon: 'assignment' }} />

<main class="scan-sweep min-h-screen px-6 pb-32 lg:px-12">
	<div class="mb-12 border-l-4 border-primary px-6 py-8">
		<p class="mb-2 font-label text-xs tracking-[0.5em] text-primary/60 uppercase">
			SYSTEM_LOG // ACCESS_GRANTED
		</p>
		<h2
			class="font-display text-4xl font-black tracking-tighter text-on-surface uppercase md:text-6xl"
		>
			INTEL_GATHERING_<span class="text-primary">TASKS</span>
		</h2>
	</div>

	<div class="mx-auto grid max-w-7xl grid-cols-1 gap-8 lg:grid-cols-12">
		<div class="space-y-6 lg:col-span-3">
			<div class="space-y-4 bg-surface-container p-6">
				<h3 class="font-label text-sm font-bold tracking-widest text-secondary uppercase">
					OPERATIVE_STAT
				</h3>
				<div class="space-y-4">
					<div>
						<p class="font-label text-xs uppercase opacity-60">MISSIONS_COMPLETED: 142</p>
						<div class="mt-1 h-1 overflow-hidden bg-surface-container-highest">
							<div class="h-full w-[70%] bg-primary transition-all"></div>
						</div>
					</div>
					<div>
						<p class="font-label text-xs uppercase opacity-60">RANK_CREDITS: 42k</p>
						<div class="mt-1 h-1 overflow-hidden bg-surface-container-highest">
							<div class="h-full w-[45%] bg-secondary transition-all"></div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="space-y-12 lg:col-span-9">
			<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
				<a
					href={resolve('/report-kill')}
					class="group space-y-4 border-l-4 border-primary bg-surface-container-low p-6 transition-all hover:bg-surface-container-high"
				>
					<div class="flex items-start justify-between">
						<h4 class="font-headline text-lg font-bold uppercase">Dead Drop in Sector 7</h4>
						<span
							class="bg-primary-container/20 px-2 py-1 font-label text-[10px] text-primary uppercase"
							>IN_PROGRESS</span
						>
					</div>
					<p class="text-sm text-on-surface-variant">
						Retrieve the encrypted flash drive from the central train hub.
					</p>
					<div class="flex items-end justify-between pt-4">
						<div>
							<p class="font-label text-[10px] text-outline uppercase">Reward</p>
							<p class="font-headline text-lg font-bold text-primary">200 Gold</p>
						</div>
						<button
							class="press-scale bg-primary px-4 py-2 text-xs font-bold tracking-widest text-on-primary uppercase"
							>REPORT</button
						>
					</div>
				</a>

				<a
					href={resolve('/loot')}
					class="group flex flex-col items-center justify-between gap-6 border-2 border-primary/30 bg-surface-container-low p-6 transition-all hover:bg-surface-container-high md:col-span-2 md:flex-row"
				>
					<div class="flex-grow">
						<div class="flex items-center gap-3">
							<h4 class="font-headline text-xl font-black uppercase">SYN_FLOOD_ANALYTICS</h4>
							<span
								class="animate-pulse bg-secondary-container/40 px-2 py-1 font-label text-[10px] text-secondary-fixed-dim uppercase"
								>READY_FOR_UPLOAD</span
							>
						</div>
						<p class="max-w-xl text-sm text-on-surface-variant">
							Initial data packet successfully extracted. Secure connection ready.
						</p>
					</div>
					<button
						class="press-scale flex items-center gap-2 bg-secondary-container px-8 py-3 text-xs font-bold tracking-widest text-on-secondary-container uppercase transition-all hover:brightness-125"
					>
						<Icon name="upload" class="text-sm" />
						UPLOAD_INTEL
					</button>
				</a>
			</div>
		</div>
	</div>
</main>
