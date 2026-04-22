<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'REPORT_KILL', icon: 'verified_user' }} />

<div class="flex pt-16 min-h-screen technical-grid">
	<main class="flex-1 max-w-5xl mx-auto p-6 md:p-12 w-full">
		<div class="mb-12 border-l-4 border-primary pl-6">
			<h2 class="font-display text-4xl md:text-6xl font-black tracking-tighter uppercase text-on-background mb-2">SUBMIT_KILL_EVIDENCE</h2>
			<div class="flex items-center gap-4 text-primary font-label text-xs tracking-[0.3em] uppercase">
				<span>REF_NO: 992-AX-REPORT</span>
				<span class="h-px w-12 bg-primary-container"></span>
				<span>STATUS: PENDING_VALIDATION</span>
			</div>
		</div>

		<form class="grid grid-cols-1 md:grid-cols-12 gap-6" onsubmit={(e) => e.preventDefault()}>
			<div class="md:col-span-4 space-y-6">
				<section class="bg-surface-container-low p-6 space-y-8">
					<div>
						<label class="block text-[10px] font-label font-bold text-primary tracking-widest uppercase mb-4">TARGET_IDENTIFIER</label>
						<select class="w-full bg-surface-container border-0 border-b border-outline-variant focus:border-primary focus:ring-0 text-on-background font-body text-sm py-3 transition-all appearance-none">
							<option>VAL-772_SYNDICATE_HEAD</option>
							<option>RAZOR_WIND_ENFORCER</option>
						</select>
					</div>
					<div>
						<label class="block text-[10px] font-label font-bold text-primary tracking-widest uppercase mb-4">TIME_OF_INCIDENT</label>
						<input class="w-full bg-surface-container border-0 border-b border-outline-variant focus:border-primary focus:ring-0 text-on-background font-body text-sm py-3" type="text" value="2023.10.24 // 03:44:12 UTC" readonly />
					</div>
					<div>
						<label class="block text-[10px] font-label font-bold text-primary tracking-widest uppercase mb-4">LOCATION_COORDINATES</label>
						<div class="flex items-center gap-4 bg-surface-container p-4 border-l-2 border-primary mb-4">
							<Icon name="location_on" class="text-primary-container" />
							<div class="font-label text-sm tracking-tighter uppercase">
								<p>LAT: 35.6895° N</p>
								<p>LON: 139.6917° E</p>
							</div>
						</div>
						<div class="h-48 bg-surface-container relative grayscale contrast-125 opacity-60">
							<img class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDp1C_E9zeU9uFZSKH5U4UULGBf9prrxDhnbnzVbtssHWlFD8P7fnk7XWQJersvCMHutkXDPb5sUZ6S6wg_z70aYdAZoYsbMUgStfZMfAYNC6Zs0leSUzy9wICPmuScAc18QCPHTj8pha3jdj4MCzKg1Ga4mbZ1MdfQZe8c6Z_O9_dFkAozT-bjFrSesojZryRp8sqUSBDK8MIHv2LKE8pBTK62tYFCdX_WUVJU-F3Y8u6WDHHFGNofZtwzfMJIJB7gs7JleNCEXyo" alt="map" />
						</div>
					</div>
				</section>
			</div>

			<div class="md:col-span-8 space-y-6">
				<section class="bg-surface-container p-1 relative overflow-hidden group">
					<div class="absolute top-4 right-4 z-10 flex gap-2">
						<div class="px-2 py-1 bg-primary text-on-primary text-[9px] font-bold tracking-widest font-label uppercase">LIVE_SCAN</div>
					</div>
					<div class="relative h-[420px] bg-surface-container-low flex flex-col items-center justify-center border-2 border-dashed border-outline-variant/30 group-hover:border-primary/50 transition-all">
						<img class="absolute inset-0 w-full h-full object-cover opacity-40 mix-blend-screen group-hover:opacity-60 transition-opacity" src="https://lh3.googleusercontent.com/aida-public/AB6AXuAmI3zz9NVaChtL5xkXqyf1ZthMpSOaxWrRxG_R0Z-xJoB7yAWIvOyebF3keRanAhJugOLXlUvbXuf9PSEAtH7IsRKBbGAr3Qp9IUtjrOSzuFwpDq9LfHBWNrWHxj98XTY_axyleclG3TfFKBZtNzmYCoLQ0XAM2LWC4jDMSPpdjC0oiRbaqNaAeLsY-wNcaUD4s0PMn-0vOQd7hmR-3Na-6wcbLZJhOcqCo3ZWSfYl2zcYxHOQKpoIjDEmOd8mQ3J_2f9kwyIQybM" alt="forensic evidence" />
						<div class="relative z-10 flex flex-col items-center text-center p-8">
							<Icon name="android_fingerprint" class="text-6xl text-primary-container mb-4" />
							<h3 class="text-xl font-headline font-bold uppercase tracking-widest text-on-background mb-2">Visual Proof / Biometric Scan</h3>
							<p class="text-sm text-on-surface-variant font-body">Drop encrypted files or retinal link. All metadata will be scrubbed.</p>
						</div>
						<div class="scan-sweep"></div>
					</div>
				</section>

				<section class="bg-surface-container-low p-8">
					<label class="block text-[10px] font-label font-bold text-secondary tracking-[0.3em] uppercase mb-4">MODUS_OPERANDI</label>
					<textarea class="w-full bg-surface-container-lowest border-0 border-l-2 border-secondary focus:border-secondary-fixed-dim focus:ring-0 text-on-background font-body text-sm p-4 h-32" placeholder="Document the protocol specifics..."></textarea>
					<div class="mt-8 flex items-center justify-between">
						<label class="flex items-center gap-3 cursor-pointer">
							<input type="checkbox" class="sr-only peer" />
							<div class="w-11 h-6 bg-surface-container-high rounded-full peer-checked:bg-secondary-container relative after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:after:translate-x-full"></div>
							<span class="text-[10px] font-label font-bold text-on-surface uppercase tracking-widest">WITNESS_PRESENT</span>
						</label>
						<div class="flex gap-2">
							<div class="w-2 h-2 bg-secondary/30"></div>
							<div class="w-2 h-2 bg-secondary"></div>
						</div>
					</div>
				</section>
				<button class="w-full group relative bg-primary-container text-on-primary-container py-6 px-12 transition-all active:scale-95 uppercase tracking-[0.5em] font-headline font-black text-xl">
					FINALIZE_REPORT
				</button>
			</div>
		</form>
	</main>
</div>
