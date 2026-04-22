<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'PERKS', icon: 'bolt' }} />

<main class="max-w-7xl mx-auto px-6 py-24 min-h-screen">
	<div class="mb-12">
		<h2 class="font-display text-5xl font-extrabold uppercase tracking-tighter">UPGRADE_CENTER</h2>
	</div>

	<section class="mb-16">
		<div class="flex items-center justify-between mb-6">
			<div class="flex items-center gap-3">
				<span class="w-2 h-2 bg-primary animate-pulse"></span>
				<h3 class="font-display text-xl font-bold uppercase tracking-widest">ACTIVE_PERKS</h3>
			</div>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<div class="bg-surface-container-high p-6 flex items-center justify-between group cursor-pointer border-l-4 border-primary">
				<div class="flex items-center gap-6">
					<div class="w-16 h-16 bg-primary-container/20 flex items-center justify-center">
						<Icon name="visibility_off" class="text-primary text-4xl" filled />
					</div>
					<div>
						<p class="font-display font-bold text-lg text-primary uppercase">Ghost Signal</p>
						<p class="font-label text-sm text-primary tracking-widest uppercase">02:45:00 REMAINING</p>
					</div>
				</div>
				<div class="flex gap-1">
					<div class="w-3 h-3 bg-primary"></div>
					<div class="w-3 h-3 bg-primary"></div>
					<div class="w-3 h-3 bg-primary/20"></div>
				</div>
			</div>
		</div>
	</section>

	<section>
		<div class="flex items-center gap-3 mb-8">
			<h3 class="font-display text-xl font-bold uppercase tracking-widest">AVAILABLE_PERKS</h3>
		</div>
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
			{#each [
				{ name: 'Elo Multiplier', cost: '1,200G', color: 'primary', icon: 'psychology' },
				{ name: 'Identity Mask', cost: '3,500G', color: 'secondary', icon: 'android_fingerprint' },
				{ name: 'Rapid Breach', cost: '800G', color: 'primary', icon: 'bolt' }
			] as perk}
				<div class="bg-surface-container flex flex-col group border border-outline-variant/10 hover:bg-surface-container-high transition-all">
					<div class="aspect-square bg-surface-container-low flex items-center justify-center relative overflow-hidden">
						<Icon name={perk.icon} class={`text-5xl text-${perk.color} z-10`} />
					</div>
					<div class="p-6">
						<h4 class="font-display font-bold text-lg uppercase mb-2">{perk.name}</h4>
						<div class="flex items-center justify-between pt-4 border-t border-outline-variant/10">
							<div class="flex flex-col">
								<span class="text-[10px] text-outline uppercase font-label">COST</span>
								<span class={`font-display font-bold text-${perk.color}`}>{perk.cost}</span>
							</div>
							<button class={`bg-${perk.color}-container text-on-${perk.color}-container px-6 py-2 text-xs font-bold uppercase press-scale`}>ACTIVATE</button>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</section>
</main>
