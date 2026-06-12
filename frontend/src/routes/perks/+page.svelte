<script lang="ts">
	import { TopBar } from '$lib/shared/ui';
	import { Icon } from '$lib/shared/ui';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
</script>

<TopBar config={{ title: 'PERKS', icon: 'bolt' }} />

<main class="mx-auto min-h-screen max-w-7xl px-6 py-24">
	<div class="mb-12">
		<h2 class="font-display text-5xl font-extrabold tracking-tighter uppercase">UPGRADE_CENTER</h2>
	</div>

	<section class="mb-16">
		<div class="mb-6 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<span class="h-2 w-2 animate-pulse bg-primary"></span>
				<h3 class="font-display text-xl font-bold tracking-widest uppercase">ACTIVE_PERKS</h3>
			</div>
		</div>
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<div
				class="group flex cursor-pointer items-center justify-between border-l-4 border-primary bg-surface-container-high p-6"
			>
				<div class="flex items-center gap-6">
					<div class="flex h-16 w-16 items-center justify-center bg-primary-container/20">
						<Icon name="visibility_off" class="text-4xl text-primary" filled />
					</div>
					<div>
						<p class="font-display text-lg font-bold text-primary uppercase">Ghost Signal</p>
						<p class="font-label text-sm tracking-widest text-primary uppercase">
							02:45:00 REMAINING
						</p>
					</div>
				</div>
				<div class="flex gap-1">
					<div class="h-3 w-3 bg-primary"></div>
					<div class="h-3 w-3 bg-primary"></div>
					<div class="h-3 w-3 bg-primary/20"></div>
				</div>
			</div>
		</div>
	</section>

	<section>
		<div class="mb-8 flex items-center gap-3">
			<h3 class="font-display text-xl font-bold tracking-widest uppercase">AVAILABLE_PERKS</h3>
		</div>
		<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
			{#each [{ name: 'Elo Multiplier', cost: '1,200G', color: 'primary', icon: 'psychology' }, { name: 'Identity Mask', cost: '3,500G', color: 'secondary', icon: 'android_fingerprint' }, { name: 'Rapid Breach', cost: '800G', color: 'primary', icon: 'bolt' }] as perk (perk.name)}
				<div
					class="group flex flex-col border border-outline-variant/10 bg-surface-container transition-all hover:bg-surface-container-high"
				>
					<div
						class="relative flex aspect-square items-center justify-center overflow-hidden bg-surface-container-low"
					>
						<Icon name={perk.icon} class={`text-5xl text-${perk.color} z-10`} />
					</div>
					<div class="p-6">
						<h4 class="font-display mb-2 text-lg font-bold uppercase">{perk.name}</h4>
						<div class="flex items-center justify-between border-t border-outline-variant/10 pt-4">
							<div class="flex flex-col">
								<span class="font-label text-[10px] text-outline uppercase">COST</span>
								<span class={`font-display font-bold text-${perk.color}`}>{perk.cost}</span>
							</div>
							<button
								class={`bg-${perk.color}-container text-on-${perk.color}-container press-scale px-6 py-2 text-xs font-bold uppercase`}
								>ACTIVATE</button
							>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</section>
</main>
