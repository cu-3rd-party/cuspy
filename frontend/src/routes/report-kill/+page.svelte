<script lang="ts">
	import { TopBar } from '$lib/shared/ui';
	import { Icon } from '$lib/shared/ui';
	import { reportKill, type KillTarget } from '$lib/shared/api';

	let { data } = $props<{
		data: {
			targets: KillTarget[];
		};
	}>();

	let targets = $derived(data.targets ?? []);
	let selectedTargetId = $state('');
	let modusOperandi = $state('');
	let witnessPresent = $state(false);
	let submitError = $state('');
	let submitMessage = $state('');
	let isSubmitting = $state(false);
	let selectedTarget = $derived(
		targets.find((target: KillTarget) => target.target_id === selectedTargetId) ?? null
	);

	const handleSubmit = async () => {
		if (!selectedTargetId) {
			submitError = 'Select a target first';
			return;
		}

		if (!modusOperandi.trim()) {
			submitError = 'Document the protocol specifics';
			return;
		}

		isSubmitting = true;
		submitError = '';
		submitMessage = '';

		try {
			await reportKill({
				victimId: selectedTargetId,
				modusOperandi: modusOperandi.trim(),
				witnessPresent
			});

			submitMessage = `Kill report submitted for ${selectedTarget?.identifier ?? 'target'}.`;
			modusOperandi = '';
			witnessPresent = false;
		} catch (error) {
			submitError = error instanceof Error ? error.message : 'Failed to submit kill report';
		} finally {
			isSubmitting = false;
		}
	};
</script>

<TopBar config={{ title: 'REPORT_KILL', icon: 'verified_user' }} />

<div class="technical-grid flex min-h-screen pt-16">
	<main class="mx-auto w-full max-w-5xl flex-1 p-6 md:p-12">
		<div class="mb-12 border-l-4 border-primary pl-6">
			<h2
				class="font-display mb-2 text-4xl font-black tracking-tighter text-on-background uppercase md:text-6xl"
			>
				SUBMIT_KILL_EVIDENCE
			</h2>
			<div
				class="flex items-center gap-4 font-label text-xs tracking-[0.3em] text-primary uppercase"
			>
				<span>REF_NO: 992-AX-REPORT</span>
				<span class="h-px w-12 bg-primary-container"></span>
				<span>STATUS: PENDING_VALIDATION</span>
			</div>
		</div>

		<form class="grid grid-cols-1 gap-6 md:grid-cols-12" onsubmit={(e) => e.preventDefault()}>
			<div class="space-y-6 md:col-span-4">
				<section class="space-y-8 bg-surface-container-low p-6">
					<div>
						<label
							for="target-identifier"
							class="mb-4 block font-label text-[10px] font-bold tracking-widest text-primary uppercase"
							>TARGET_IDENTIFIER</label
						>
						<select
							id="target-identifier"
							bind:value={selectedTargetId}
							class="w-full appearance-none border-0 border-b border-outline-variant bg-surface-container py-3 font-body text-sm text-on-background transition-all focus:border-primary focus:ring-0"
						>
							{#each targets as target (target.target_id)}
								<option value={target.target_id}>{target.identifier}</option>
							{/each}
						</select>
					</div>
					<div>
						<label
							for="incident-time"
							class="mb-4 block font-label text-[10px] font-bold tracking-widest text-primary uppercase"
							>TIME_OF_INCIDENT</label
						>
						<input
							id="incident-time"
							class="w-full border-0 border-b border-outline-variant bg-surface-container py-3 font-body text-sm text-on-background focus:border-primary focus:ring-0"
							type="text"
							value="2023.10.24 // 03:44:12 UTC"
							readonly
						/>
					</div>
					<div aria-labelledby="location-coordinates-label">
						<div
							id="location-coordinates-label"
							class="mb-4 block font-label text-[10px] font-bold tracking-widest text-primary uppercase"
						>
							LOCATION_COORDINATES
						</div>
						<div
							class="mb-4 flex items-center gap-4 border-l-2 border-primary bg-surface-container p-4"
						>
							<Icon name="location_on" class="text-primary-container" />
							<div class="font-label text-sm tracking-tighter uppercase">
								<p>TARGET: {selectedTarget?.identifier ?? 'UNASSIGNED'}</p>
								<p>LAST_SEEN: {selectedTarget?.last_known_location ?? 'UNKNOWN'}</p>
							</div>
						</div>
						<div class="relative h-48 bg-surface-container opacity-60 contrast-125 grayscale">
							<img
								class="h-full w-full object-cover"
								src="https://lh3.googleusercontent.com/aida-public/AB6AXuDp1C_E9zeU9uFZSKH5U4UULGBf9prrxDhnbnzVbtssHWlFD8P7fnk7XWQJersvCMHutkXDPb5sUZ6S6wg_z70aYdAZoYsbMUgStfZMfAYNC6Zs0leSUzy9wICPmuScAc18QCPHTj8pha3jdj4MCzKg1Ga4mbZ1MdfQZe8c6Z_O9_dFkAozT-bjFrSesojZryRp8sqUSBDK8MIHv2LKE8pBTK62tYFCdX_WUVJU-F3Y8u6WDHHFGNofZtwzfMJIJB7gs7JleNCEXyo"
								alt="map"
							/>
						</div>
					</div>
				</section>
			</div>

			<div class="space-y-6 md:col-span-8">
				{#if submitError}
					<div
						class="bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase"
					>
						{submitError}
					</div>
				{/if}

				{#if submitMessage}
					<div
						class="bg-primary-container px-4 py-3 font-label text-[11px] tracking-[0.16em] text-on-primary-container uppercase"
					>
						{submitMessage}
					</div>
				{/if}

				<section class="group relative overflow-hidden bg-surface-container p-1">
					<div class="absolute top-4 right-4 z-10 flex gap-2">
						<div
							class="bg-primary px-2 py-1 font-label text-[9px] font-bold tracking-widest text-on-primary uppercase"
						>
							LIVE_SCAN
						</div>
					</div>
					<div
						class="relative flex h-[420px] flex-col items-center justify-center border-2 border-dashed border-outline-variant/30 bg-surface-container-low transition-all group-hover:border-primary/50"
					>
						<img
							class="absolute inset-0 h-full w-full object-cover opacity-40 mix-blend-screen transition-opacity group-hover:opacity-60"
							src="https://lh3.googleusercontent.com/aida-public/AB6AXuAmI3zz9NVaChtL5xkXqyf1ZthMpSOaxWrRxG_R0Z-xJoB7yAWIvOyebF3keRanAhJugOLXlUvbXuf9PSEAtH7IsRKBbGAr3Qp9IUtjrOSzuFwpDq9LfHBWNrWHxj98XTY_axyleclG3TfFKBZtNzmYCoLQ0XAM2LWC4jDMSPpdjC0oiRbaqNaAeLsY-wNcaUD4s0PMn-0vOQd7hmR-3Na-6wcbLZJhOcqCo3ZWSfYl2zcYxHOQKpoIjDEmOd8mQ3J_2f9kwyIQybM"
							alt="forensic evidence"
						/>
						<div class="relative z-10 flex flex-col items-center p-8 text-center">
							<Icon name="android_fingerprint" class="mb-4 text-6xl text-primary-container" />
							<h3
								class="mb-2 font-headline text-xl font-bold tracking-widest text-on-background uppercase"
							>
								Visual Proof / Biometric Scan
							</h3>
							<p class="font-body text-sm text-on-surface-variant">
								Drop encrypted files or retinal link. All metadata will be scrubbed.
							</p>
						</div>
						<div class="scan-sweep"></div>
					</div>
				</section>

				<section class="bg-surface-container-low p-8">
					<label
						for="modus-operandi"
						class="mb-4 block font-label text-[10px] font-bold tracking-[0.3em] text-secondary uppercase"
						>MODUS_OPERANDI</label
					>
					<textarea
						id="modus-operandi"
						bind:value={modusOperandi}
						class="h-32 w-full border-0 border-l-2 border-secondary bg-surface-container-lowest p-4 font-body text-sm text-on-background focus:border-secondary-fixed-dim focus:ring-0"
						placeholder="Document the protocol specifics..."
					></textarea>
					<div class="mt-8 flex items-center justify-between">
						<label class="flex cursor-pointer items-center gap-3">
							<input type="checkbox" bind:checked={witnessPresent} class="peer sr-only" />
							<div
								class="relative h-6 w-11 rounded-full bg-surface-container-high peer-checked:bg-secondary-container after:absolute after:top-[2px] after:left-[2px] after:h-5 after:w-5 after:rounded-full after:bg-white after:transition-all peer-checked:after:translate-x-full"
							></div>
							<span
								class="font-label text-[10px] font-bold tracking-widest text-on-surface uppercase"
								>WITNESS_PRESENT</span
							>
						</label>
						<div class="flex gap-2">
							<div class="h-2 w-2 bg-secondary/30"></div>
							<div class="h-2 w-2 bg-secondary"></div>
						</div>
					</div>
				</section>
				<button
					type="button"
					onclick={handleSubmit}
					disabled={isSubmitting}
					class="group relative w-full bg-primary-container px-12 py-6 font-headline text-xl font-black tracking-[0.5em] text-on-primary-container uppercase transition-all active:scale-95 disabled:opacity-50"
				>
					{isSubmitting ? 'SUBMITTING_REPORT' : 'FINALIZE_REPORT'}
				</button>
			</div>
		</form>
	</main>
</div>
