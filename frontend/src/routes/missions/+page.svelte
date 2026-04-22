<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'MISSIONS', icon: 'assignment' }} />

<main class="min-h-screen pb-32 px-6 lg:px-12 scan-sweep">
	<div class="px-6 py-8 border-l-4 border-primary mb-12">
		<p class="font-label text-xs tracking-[0.5em] text-primary/60 mb-2 uppercase">SYSTEM_LOG // ACCESS_GRANTED</p>
		<h2 class="font-display text-4xl md:text-6xl font-black uppercase tracking-tighter text-on-surface">
			INTEL_GATHERING_<span class="text-primary">TASKS</span>
		</h2>
	</div>

	<div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-8">
		<div class="lg:col-span-3 space-y-6">
			<div class="bg-surface-container p-6 space-y-4">
				<h3 class="font-label text-sm font-bold text-secondary uppercase tracking-widest">OPERATIVE_STAT</h3>
				<div class="space-y-4">
					<div>
						<p class="text-xs opacity-60 uppercase font-label">MISSIONS_COMPLETED: 142</p>
						<div class="h-1 bg-surface-container-highest mt-1 overflow-hidden">
							<div class="h-full bg-primary w-[70%] transition-all"></div>
						</div>
					</div>
					<div>
						<p class="text-xs opacity-60 uppercase font-label">RANK_CREDITS: 42k</p>
						<div class="h-1 bg-surface-container-highest mt-1 overflow-hidden">
							<div class="h-full bg-secondary w-[45%] transition-all"></div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="lg:col-span-9 space-y-12">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<a href="/report-kill" class="bg-surface-container-low border-l-4 border-primary p-6 space-y-4 hover:bg-surface-container-high transition-all group">
					<div class="flex justify-between items-start">
						<h4 class="font-headline text-lg font-bold uppercase">Dead Drop in Sector 7</h4>
						<span class="text-[10px] bg-primary-container/20 text-primary px-2 py-1 font-label uppercase">IN_PROGRESS</span>
					</div>
					<p class="text-sm text-on-surface-variant">Retrieve the encrypted flash drive from the central train hub.</p>
					<div class="flex justify-between items-end pt-4">
						<div>
							<p class="text-[10px] uppercase text-outline font-label">Reward</p>
							<p class="font-headline text-primary font-bold text-lg">200 Gold</p>
						</div>
						<button class="bg-primary text-on-primary text-xs px-4 py-2 font-bold uppercase tracking-widest press-scale">REPORT</button>
					</div>
				</a>

				<a href="/loot" class="bg-surface-container-low border-2 border-primary/30 p-6 md:col-span-2 flex flex-col md:flex-row gap-6 justify-between items-center group hover:bg-surface-container-high transition-all">
					<div class="flex-grow">
						<div class="flex items-center gap-3">
							<h4 class="font-headline text-xl font-black uppercase">SYN_FLOOD_ANALYTICS</h4>
							<span class="text-[10px] bg-secondary-container/40 text-secondary-fixed-dim px-2 py-1 animate-pulse font-label uppercase">READY_FOR_UPLOAD</span>
						</div>
						<p class="text-sm text-on-surface-variant max-w-xl">Initial data packet successfully extracted. Secure connection ready.</p>
					</div>
					<button class="bg-secondary-container text-on-secondary-container text-xs font-bold px-8 py-3 hover:brightness-125 transition-all flex items-center gap-2 uppercase tracking-widest press-scale">
						<Icon name="upload" class="text-sm" />
						UPLOAD_INTEL
					</button>
				</a>
			</div>
		</div>
	</div>
</main>
