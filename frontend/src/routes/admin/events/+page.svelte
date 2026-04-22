<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'PROTOCOL_OVERRIDE', icon: 'language' }} />

<main class="p-8 lg:p-12 space-y-12 relative overflow-y-auto">
	<div class="border-l-4 border-primary pl-6">
		<h2 class="text-5xl font-headline font-black tracking-tighter uppercase italic">PROTOCOL_OVERRIDE</h2>
		<p class="text-on-surface-variant font-label tracking-widest text-xs mt-2 uppercase">GLOBAL COMMAND AUTHORIZATION REQUIRED</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
		<section class="lg:col-span-8 space-y-8">
			<div class="bg-surface-container p-8 relative overflow-hidden group border border-outline-variant/10">
				<h3 class="font-headline font-bold text-lg mb-8 flex items-center gap-2 uppercase">
					<Icon name="public" class="text-primary" />EVENT_CATALYST
				</h3>
				<div class="grid md:grid-cols-2 gap-8">
					<div class="bg-surface-container-high p-6 space-y-4 border-l border-error">
						<h4 class="font-bold text-error uppercase">CASCADE_FAILURE / TRIGGER_SMITE</h4>
						<p class="text-xs opacity-70">Immediate system-wide disruption. Targets all non-cleared nodes.</p>
						<button class="w-full bg-error-container text-on-error-container py-4 font-bold uppercase hover:bg-error transition-all press-scale">EXECUTE_SMITE</button>
					</div>
					<div class="bg-surface-container-high p-6 space-y-4 border-l border-primary">
						<h4 class="font-bold text-primary uppercase">TERRITORY_EXPANSION</h4>
						<p class="text-xs opacity-70">Expand influence radius. Increases operative visibility.</p>
						<div class="h-1 bg-surface-container-lowest overflow-hidden">
							<div class="h-full w-2/3 bg-primary transition-all"></div>
						</div>
					</div>
				</div>
			</div>

			<div class="bg-surface-container p-8 space-y-6">
				<h3 class="font-headline font-bold uppercase">AGENT_DATABASE_QUERY</h3>
				<div class="overflow-x-auto">
					<table class="w-full text-left font-label text-xs border-collapse">
						<thead class="bg-surface-container-low uppercase tracking-widest">
							<tr class="border-b border-outline-variant/10">
								<th class="py-4 px-4">Identifier</th>
								<th class="py-4 px-4">Status</th>
								<th class="py-4 px-4">Threat</th>
							</tr>
						</thead>
						<tbody>
							<tr class="hover:bg-primary/5 transition-all">
								<td class="py-4 px-4 font-bold uppercase">OPERATIVE_99</td>
								<td class="py-4 px-4 text-primary uppercase">ACTIVE</td>
								<td class="py-4 px-4 uppercase">OMEGA</td>
							</tr>
							<tr class="hover:bg-primary/5 transition-all">
								<td class="py-4 px-4 bg-on-surface text-transparent select-none">REDACTED_TEXT</td>
								<td class="py-4 px-4 text-error uppercase">COMPROMISED</td>
								<td class="py-4 px-4 uppercase">SIGMA</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</section>

		<aside class="lg:col-span-4 flex flex-col gap-8">
			<section class="bg-surface-container h-[600px] flex flex-col border-t-2 border-primary">
				<div class="p-4 bg-surface-container-low flex justify-between items-center">
					<div class="flex items-center gap-2">
						<div class="w-2 h-2 bg-primary animate-pulse"></div>
						<span class="font-bold text-xs uppercase">SYSTEM_LOGS</span>
					</div>
				</div>
				<div class="flex-1 p-6 font-mono text-[11px] overflow-y-auto space-y-2 text-primary/80">
					<p><span class="opacity-50">[14:02]</span> AUTH_LEVEL: PHANTOM</p>
					<p><span class="opacity-50">[14:03]</span> NODE_77_INFILTRATION: SUCCESS</p>
					<p class="text-error"><span class="opacity-50">[14:04]</span> ALERT: UNAUTHORIZED_QUERY</p>
					<p class="bg-on-surface text-transparent select-none">ENCRYPTED_LOG_STRING_REDACTED</p>
				</div>
			</section>
		</aside>
	</div>
</main>
