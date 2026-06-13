<script lang="ts">
	import { resolve } from '$app/paths';
	import { Icon } from '$lib/shared/ui';
	import { TerminalShell } from '$lib/shared/ui';
	import { gameplayNav } from '$lib/shared/config';

	let { data: _data = undefined } = $props<{ data?: unknown }>();
</script>

<TerminalShell topBar={{ title: 'SURVEILLANCE', icon: 'android_fingerprint' }} nav={gameplayNav}>
	<section class="mt-8 mb-12 flex flex-col justify-between gap-6 md:flex-row md:items-end">
		<div>
			<h1
				class="font-display text-4xl font-black tracking-tighter text-on-surface uppercase md:text-6xl"
			>
				ACTIVE_SURVEILLANCE
			</h1>
			<div class="mt-2 flex items-center gap-2">
				<span class="h-2 w-2 animate-pulse rounded-full bg-primary"></span>
				<p class="font-label text-xs tracking-widest text-primary uppercase">
					Satellite Link Stable // 3 Targets Localized
				</p>
			</div>
		</div>
		<div class="border-l-4 border-primary bg-surface-container-high p-4">
			<span class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">Coordinate</span>
			<span class="font-display text-lg font-bold tracking-widest">34.0522° N, 118.2437° W</span>
		</div>
	</section>

	<div class="grid grid-cols-1 gap-6 lg:grid-cols-12">
		<div class="space-y-6 lg:col-span-8">
			{#each [{ name: 'PHANTOM_STRIKE', id: 'CAM_01', color: 'primary', img: 'https://lh3.googleusercontent.com/aida-public/AB6AXuDmThpWSfMtoWM96Q13EIVkgH8GeW1YmobA4_Twl-Azmm8p6DuZ8HNen3BL001b8wlg4C6U5mSBfLVGRatqSu7nUqMSgTLuCeIn9fl04mgst0qJF73EYc_fxuU9qR_ro4lqY7x1zsJMg-Rmc2WpNG7trJp9FF7s8l7Qv7e89g2AS4jv4FFjay3PcD3kjTL8F561dHQw7NQrnP35n8mNWFuWzJe9rDwUUhCeVWe7SY3u1d531ueFRBMDpxDpmonk4njLAaU_A8y0ucQ' }, { name: 'NEON_WIDOW', id: 'CAM_04', color: 'secondary', img: 'https://lh3.googleusercontent.com/aida-public/AB6AXuCREGMi-Wz9FQ3-DFiXTMYYROvQPcA2l0BfN36j86q9UgNfP0APQ_m3KNVABqVcoICX5-erzW2qXVc8ujHqndPvwAFHNJgniUOmXbkfGOmgZlz5mMURUVD40AJwGDzxhbnlSFvHBjigdTM-sKp868FzHYnXojMJSwWny25nAusp4g1872LzP5oaanRZxFJc5fvOPvrH7hkhQvJn0icmPXOjkoKi2YIm-S7WUvmPESwKVG8ghmOBCog-XATFcPMpIjiwdCvOQBD_YH8' }] as target (target.id)}
				<div
					class="group relative overflow-hidden bg-surface-container-low transition-all duration-300 hover:bg-surface-container-high"
				>
					<div class="flex flex-col md:flex-row">
						<div
							class="relative h-48 w-full overflow-hidden contrast-125 grayscale transition-all hover:grayscale-0 md:h-auto md:w-48"
						>
							<img src={target.img} class="h-full w-full object-cover" alt="target" />
							<div
								class="absolute top-2 left-2 bg-black/80 px-2 py-1 font-label text-[10px] text-primary uppercase"
							>
								{target.id}_FEED
							</div>
						</div>
						<div class="flex flex-1 flex-col justify-between p-6">
							<div class="flex items-start justify-between">
								<h2
									class="font-display text-2xl font-bold tracking-tight text-on-surface uppercase"
								>
									CODENAME: {target.name}
								</h2>
								<div class={`bg-${target.color}/10 border border-${target.color}/20 px-3 py-1`}>
									<span
										class={`text-${target.color} font-label text-xs font-bold tracking-widest uppercase`}
										>TRACKING</span
									>
								</div>
							</div>
							<div class="mt-8 flex justify-end">
								<a
									href={resolve('/target-intel')}
									class="press-scale bg-primary-container px-8 py-3 font-label text-sm font-bold tracking-widest text-on-primary-container uppercase hover:brightness-110"
									>OPEN_DOSSIER</a
								>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
		<div class="space-y-6 lg:col-span-4">
			<div class="relative overflow-hidden bg-surface-container">
				<div
					class="flex items-center justify-between border-b border-outline-variant/10 p-4 text-xs font-bold text-primary uppercase"
				>
					Tactical Radar
				</div>
				<div class="relative flex aspect-square items-center justify-center bg-black/20 p-8">
					<div class="absolute h-[90%] w-[90%] rounded-full border border-primary/10"></div>
					<div class="absolute h-[60%] w-[60%] rounded-full border border-primary/10"></div>
					<div
						class="absolute inset-0 rotate-45 rounded-full bg-gradient-to-tr from-primary/20 to-transparent"
					></div>
					<div
						class="blip absolute top-1/4 left-1/3 h-3 w-3 animate-pulse rounded-full bg-primary"
					></div>
				</div>
			</div>
		</div>
	</div>
</TerminalShell>
