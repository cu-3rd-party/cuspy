<script lang="ts">
	import { resolve } from '$app/paths';
	import { TopBar } from '$lib/shared/ui';
	import { Icon } from '$lib/shared/ui';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
</script>

<TopBar config={{ title: 'GEAR', icon: 'inventory_2' }} />

<main
	class="technical-grid relative flex min-h-screen flex-col items-center justify-center overflow-hidden pt-20 pb-24"
>
	<div class="relative z-10 w-full max-w-5xl px-6 text-center">
		<div class="mb-12">
			<h1
				class="font-display mb-2 text-4xl font-black tracking-tighter text-on-background uppercase md:text-6xl"
			>
				ENCRYPTED_CACHE_UNLOCKED
			</h1>
			<div
				class="flex items-center justify-center gap-4 font-label text-sm tracking-[0.4em] text-primary uppercase"
			>
				<span>STATUS: DECRYPTED</span>
				<span class="h-2 w-2 animate-pulse rounded-full bg-primary"></span>
				<span>PHANTOM_NODE</span>
			</div>
		</div>
		<div class="relative mx-auto mb-12 h-80 w-80 md:h-96 md:w-96">
			<div
				class="relative flex h-full w-full items-center justify-center overflow-hidden border border-outline-variant/15 bg-surface-container/60 shadow-[0_0_50px_rgba(118,221,113,0.15)] backdrop-blur-xl"
			>
				<div class="absolute top-0 left-0 h-8 w-8 border-t-2 border-l-2 border-primary"></div>
				<div
					class="absolute right-0 bottom-0 h-8 w-8 border-r-2 border-b-2 border-secondary-container"
				></div>
				<div class="flex flex-col items-center">
					<Icon
						name="deployed_code"
						class="animate-pulse text-7xl text-primary md:text-8xl"
						filled
					/>
					<div class="mt-4 font-label text-xs tracking-widest text-outline uppercase">
						CORE_ID: X-990
					</div>
				</div>
			</div>
		</div>
		<div class="mb-16 grid grid-cols-1 gap-4 md:grid-cols-3">
			<div class="border-l-4 border-primary bg-surface-container-low p-6 text-left">
				<div class="font-display text-3xl font-bold">500 GOLD</div>
				<div class="mt-1 font-label text-xs text-primary uppercase">CURRENCY_DEPOSITED</div>
			</div>
			<div class="border-l-4 border-secondary bg-surface-container-low p-6 text-left">
				<div class="font-display text-xl font-bold uppercase">PERK: GHOST_SIGNAL</div>
				<div class="mt-1 font-label text-xs text-secondary uppercase">INVISIBILITY +15%</div>
			</div>
			<div class="border-l-4 border-outline-variant bg-surface-container-low p-6 text-left">
				<div class="font-display text-xl font-bold uppercase">COSMETIC: NIGHT_VISOR</div>
				<div class="mt-1 font-label text-xs text-outline uppercase">SQUAD_ID: ENABLED</div>
			</div>
		</div>
		<a
			href={resolve('/perks')}
			class="font-display inline-block bg-primary-container px-16 py-5 text-xl font-black tracking-[0.3em] text-on-primary-container uppercase shadow-[0_0_30px_rgba(0,122,27,0.3)] transition-all hover:scale-105 active:scale-95"
		>
			CLAIM_ASSETS
		</a>
	</div>
</main>
