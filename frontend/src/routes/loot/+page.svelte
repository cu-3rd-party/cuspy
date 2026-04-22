<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'GEAR', icon: 'inventory_2' }} />

<main class="min-h-screen pt-20 pb-24 flex flex-col items-center justify-center relative overflow-hidden technical-grid">
	<div class="relative z-10 w-full max-w-5xl px-6 text-center">
		<div class="mb-12">
			<h1 class="font-display text-4xl md:text-6xl font-black tracking-tighter text-on-background mb-2 uppercase">ENCRYPTED_CACHE_UNLOCKED</h1>
			<div class="flex justify-center items-center gap-4 text-primary font-label text-sm tracking-[0.4em] uppercase">
				<span>STATUS: DECRYPTED</span>
				<span class="w-2 h-2 bg-primary rounded-full animate-pulse"></span>
				<span>PHANTOM_NODE</span>
			</div>
		</div>
		<div class="relative w-80 h-80 md:w-96 md:h-96 mx-auto mb-12">
			<div class="w-full h-full bg-surface-container/60 backdrop-blur-xl border border-outline-variant/15 flex items-center justify-center relative overflow-hidden shadow-[0_0_50px_rgba(118,221,113,0.15)]">
				<div class="absolute top-0 left-0 w-8 h-8 border-t-2 border-l-2 border-primary"></div>
				<div class="absolute bottom-0 right-0 w-8 h-8 border-b-2 border-r-2 border-secondary-container"></div>
				<div class="flex flex-col items-center">
					<Icon name="deployed_code" class="text-7xl md:text-8xl text-primary animate-pulse" filled />
					<div class="mt-4 font-label text-xs tracking-widest text-outline uppercase">CORE_ID: X-990</div>
				</div>
			</div>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-16">
			<div class="bg-surface-container-low p-6 border-l-4 border-primary text-left">
				<div class="text-3xl font-display font-bold">500 GOLD</div>
				<div class="text-xs font-label text-primary mt-1 uppercase">CURRENCY_DEPOSITED</div>
			</div>
			<div class="bg-surface-container-low p-6 border-l-4 border-secondary text-left">
				<div class="text-xl font-display font-bold uppercase">PERK: GHOST_SIGNAL</div>
				<div class="text-xs font-label text-secondary mt-1 uppercase">INVISIBILITY +15%</div>
			</div>
			<div class="bg-surface-container-low p-6 border-l-4 border-outline-variant text-left">
				<div class="text-xl font-display font-bold uppercase">COSMETIC: NIGHT_VISOR</div>
				<div class="text-xs font-label text-outline mt-1 uppercase">SQUAD_ID: ENABLED</div>
			</div>
		</div>
		<a href="/perks" class="bg-primary-container text-on-primary-container px-16 py-5 font-display font-black text-xl tracking-[0.3em] hover:scale-105 active:scale-95 shadow-[0_0_30px_rgba(0,122,27,0.3)] transition-all inline-block uppercase">
			CLAIM_ASSETS
		</a>
	</div>
</main>
