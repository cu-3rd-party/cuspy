<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'SURVEILLANCE', icon: 'android_fingerprint' }} />

<main class="min-h-screen pt-20 pb-24 px-4 md:px-8 max-w-7xl mx-auto technical-grid">
	<section class="mt-8 mb-12 flex flex-col md:flex-row md:items-end justify-between gap-6">
		<div>
			<h1 class="font-display text-4xl md:text-6xl font-black tracking-tighter text-on-surface uppercase">ACTIVE_SURVEILLANCE</h1>
			<div class="flex items-center gap-2 mt-2">
				<span class="w-2 h-2 bg-primary rounded-full animate-pulse"></span>
				<p class="font-label text-xs tracking-widest text-primary uppercase">Satellite Link Stable // 3 Targets Localized</p>
			</div>
		</div>
		<div class="bg-surface-container-high p-4 border-l-4 border-primary">
			<span class="font-label text-[10px] text-outline uppercase tracking-[0.2em]">Coordinate</span>
			<span class="font-display font-bold text-lg tracking-widest">34.0522° N, 118.2437° W</span>
		</div>
	</section>

	<div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
		<div class="lg:col-span-8 space-y-6">
			{#each [
				{ name: 'PHANTOM_STRIKE', id: 'CAM_01', color: 'primary', img: 'https://lh3.googleusercontent.com/aida-public/AB6AXuDmThpWSfMtoWM96Q13EIVkgH8GeW1YmobA4_Twl-Azmm8p6DuZ8HNen3BL001b8wlg4C6U5mSBfLVGRatqSu7nUqMSgTLuCeIn9fl04mgst0qJF73EYc_fxuU9qR_ro4lqY7x1zsJMg-Rmc2WpNG7trJp9FF7s8l7Qv7e89g2AS4jv4FFjay3PcD3kjTL8F561dHQw7NQrnP35n8mNWFuWzJe9rDwUUhCeVWe7SY3u1d531ueFRBMDpxDpmonk4njLAaU_A8y0ucQ' },
				{ name: 'NEON_WIDOW', id: 'CAM_04', color: 'secondary', img: 'https://lh3.googleusercontent.com/aida-public/AB6AXuCREGMi-Wz9FQ3-DFiXTMYYROvQPcA2l0BfN36j86q9UgNfP0APQ_m3KNVABqVcoICX5-erzW2qXVc8ujHqndPvwAFHNJgniUOmXbkfGOmgZlz5mMURUVD40AJwGDzxhbnlSFvHBjigdTM-sKp868FzHYnXojMJSwWny25nAusp4g1872LzP5oaanRZxFJc5fvOPvrH7hkhQvJn0icmPXOjkoKi2YIm-S7WUvmPESwKVG8ghmOBCog-XATFcPMpIjiwdCvOQBD_YH8' }
			] as target}
				<div class="bg-surface-container-low group relative overflow-hidden transition-all duration-300 hover:bg-surface-container-high">
					<div class="flex flex-col md:flex-row">
						<div class="relative w-full md:w-48 h-48 md:h-auto overflow-hidden grayscale contrast-125 hover:grayscale-0 transition-all">
							<img src={target.img} class="w-full h-full object-cover" alt="target" />
							<div class="absolute top-2 left-2 bg-black/80 px-2 py-1 text-[10px] font-label text-primary uppercase">{target.id}_FEED</div>
						</div>
						<div class="flex-1 p-6 flex flex-col justify-between">
							<div class="flex justify-between items-start">
								<h2 class="font-display text-2xl font-bold tracking-tight text-on-surface uppercase">CODENAME: {target.name}</h2>
								<div class={`bg-${target.color}/10 border border-${target.color}/20 px-3 py-1`}>
									<span class={`text-${target.color} font-label text-xs font-bold tracking-widest uppercase`}>TRACKING</span>
								</div>
							</div>
							<div class="mt-8 flex justify-end">
								<a href="/target-intel" class="bg-primary-container text-on-primary-container font-label text-sm font-bold py-3 px-8 hover:brightness-110 uppercase tracking-widest press-scale">OPEN_DOSSIER</a>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
		<div class="lg:col-span-4 space-y-6">
			<div class="bg-surface-container overflow-hidden relative">
				<div class="p-4 border-b border-outline-variant/10 flex justify-between items-center text-primary font-bold uppercase text-xs">Tactical Radar</div>
				<div class="aspect-square relative flex items-center justify-center p-8 bg-black/20">
					<div class="absolute w-[90%] h-[90%] border border-primary/10 rounded-full"></div>
					<div class="absolute w-[60%] h-[60%] border border-primary/10 rounded-full"></div>
					<div class="absolute inset-0 bg-gradient-to-tr from-primary/20 to-transparent rounded-full rotate-45"></div>
					<div class="blip w-3 h-3 bg-primary rounded-full absolute top-1/4 left-1/3 animate-pulse"></div>
				</div>
			</div>
		</div>
	</div>
</main>
