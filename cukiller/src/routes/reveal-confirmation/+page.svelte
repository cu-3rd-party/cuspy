<script lang="ts">
	import TerminalShell from '$lib/prototype/TerminalShell.svelte';
</script>

<TerminalShell topBar={{ title: 'GHOST SIGNAL', icon: 'warning', subtitle: 'Priority One Alert' }}>
	<div class="fixed inset-0 pointer-events-none overflow-hidden">
		<div class="absolute -left-[10%] -top-[10%] h-[40%] w-[40%] rounded-full bg-error-container/10 blur-[120px]"></div>
		<div class="absolute right-[-5%] top-[20%] h-[50%] w-[30%] rounded-full bg-secondary-container/10 blur-[100px]"></div>
	</div>

	<div class="relative z-10 mx-auto flex max-w-2xl flex-col space-y-12 py-10">
		<header class="space-y-4">
			<div class="flex items-center gap-3 text-error">
				<span class="material-symbols-outlined text-3xl" style="font-variation-settings:'FILL' 1">warning</span>
				<span class="font-label text-sm font-bold uppercase tracking-[0.3em]">Priority One Alert</span>
			</div>
			<h1 class="font-headline text-5xl font-bold uppercase leading-none tracking-tight sm:text-7xl">
				SECURITY <br />
				<span class="text-error">BREACH</span>
			</h1>
			<div class="h-1 w-24 bg-error"></div>
		</header>

		<section class="relative overflow-hidden bg-surface-container-low p-1">
			<div class="absolute right-0 top-0 p-4 font-label text-[10px] uppercase text-outline opacity-40">Ref: 099-ALPHA-SIGMA</div>
			<div class="border-l-4 border-error bg-surface-container p-6 md:p-10">
				<div class="space-y-6">
					<div class="flex items-start justify-between gap-4">
						<p class="max-w-md font-headline text-xl leading-relaxed md:text-2xl">
							<span class="bg-secondary-container/20 px-1 text-secondary">Agent X</span> claims to have revealed you.
						</p>
						<span class="material-symbols-outlined text-outline-variant">fingerprint</span>
					</div>
					<div class="grid gap-8 border-y border-outline-variant/20 py-6 md:grid-cols-2">
						<div>
							<p class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Observation status</p>
							<p class="mt-2 text-sm font-medium text-on-surface-variant">Was there a witness to this compromise?</p>
						</div>
						<div>
							<p class="font-label text-[10px] uppercase tracking-[0.2em] text-outline">Physical engagement</p>
							<p class="mt-2 text-sm font-medium text-on-surface-variant">Did they make physical contact during the reveal?</p>
						</div>
					</div>
					<div class="flex items-center gap-4 bg-surface-container-highest/50 p-4">
						<span class="material-symbols-outlined text-secondary" style="font-variation-settings:'FILL' 1">lock_person</span>
						<p class="font-label text-[11px] uppercase leading-tight text-outline">Immediate action required. Failure to resolve status will result in <span class="font-bold text-error">automatic network expulsion</span>.</p>
					</div>
				</div>
			</div>
		</section>

		<div class="grid gap-4 md:grid-cols-2">
			<button class="group press-down flex flex-col bg-error-container p-6 text-left transition-all hover:bg-[#a5000b]">
				<div class="mb-8 flex items-center justify-between">
					<span class="material-symbols-outlined group-hover-grow text-on-error-container">emergency_home</span>
					<span class="font-label text-[10px] uppercase text-on-error-container/60">Code: Redact</span>
				</div>
				<span class="font-headline text-xl font-bold uppercase text-on-error-container">I AM COMPROMISED</span>
				<p class="mt-2 font-label text-[10px] uppercase text-on-error-container/70">Submit and accept loss of field status</p>
			</button>

			<button class="group flex flex-col border-t-2 border-transparent bg-surface-container-highest p-6 text-left transition-all hover:bg-outline-variant active:border-secondary">
				<div class="mb-8 flex items-center justify-between">
					<span class="material-symbols-outlined group-hover-tilt text-secondary">gavel</span>
					<span class="font-label text-[10px] uppercase text-outline">Code: Contest</span>
				</div>
				<span class="font-headline text-xl font-bold uppercase">DISPUTE CLAIM</span>
				<p class="mt-2 font-label text-[10px] uppercase text-on-surface-variant">Initiate mediation protocol via witness audit</p>
			</button>
		</div>

		<footer class="pt-12 text-center">
			<div class="flex justify-center gap-8">
				{#each [['Latency', '24MS'], ['Node', 'HK_09'], ['Signal', 'STABLE']] as [label, value]}
					<div class="opacity-30">
						<div class="font-label text-[10px] uppercase">{label}</div>
						<div class="font-headline text-xs font-bold">{value}</div>
					</div>
				{/each}
			</div>
			<p class="mt-4 font-label text-[9px] uppercase tracking-[0.4em] text-outline">Ghost Signal encrypted terminal v1.02.4</p>
		</footer>
	</div>

	<div class="fixed left-0 top-0 z-50 h-1 w-full bg-surface-container-highest">
		<div class="live-bar h-full w-1/3 bg-error"></div>
	</div>
</TerminalShell>
