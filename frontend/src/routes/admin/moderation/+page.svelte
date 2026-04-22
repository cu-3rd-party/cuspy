<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
</script>

<TopBar config={{ title: 'COMMAND_CENTER // MODERATION', icon: 'verified_user' }} />

<main class="p-8 technical-grid">
	<header class="mb-12 border-l-4 border-primary pl-6">
		<h2 class="text-4xl font-display font-black tracking-tighter uppercase">COMMAND_CENTER // MODERATION</h2>
		<p class="text-on-surface-variant font-label text-sm tracking-[0.3em] uppercase">REAL-TIME EXECUTION AUDIT</p>
	</header>

	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
		<div class="bg-surface-container p-4 border-l-2 border-primary">
			<div class="text-xs uppercase font-label">Pending Reports</div>
			<div class="text-3xl font-display font-bold text-primary">42</div>
		</div>
		<div class="bg-surface-container p-4 border-l-2 border-secondary">
			<div class="text-xs uppercase font-label">High Priority</div>
			<div class="text-3xl font-display font-bold text-secondary">08</div>
		</div>
	</div>

	<section class="bg-surface-container-lowest border border-outline-variant/15 p-6 md:p-8">
		<div class="flex justify-between items-start mb-8">
			<h3 class="text-2xl font-display font-bold uppercase">PENDING_KILL_REPORT: #CK-99021</h3>
			<div class="text-right text-[10px] uppercase font-label">2024-05-24 // 22:14:09 UTC</div>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
			<div class="lg:col-span-8 grid grid-cols-1 md:grid-cols-2 gap-4">
				<div class="aspect-[4/5] bg-surface-container-high overflow-hidden grayscale brightness-75">
					<img class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDiYk3LNc7y-nIFdPB-M7GM5S-MRt0M8c4KfYVSw4WMHr39dIzkam2x7jXpXMz5A6JdVXO-tQ1ouNwEgmrSqeSSR9wgUhPay5wEJ2a4O_V2F38GULUgAihVMu0vQguSdiW8qzpBQkceJwulM5b0sPaBRhCusyi-0-tRiFzUQ7GtdFqEaSI8QvPU2k56XxfRDa548s_7n1Dntptd2F newLine" alt="evidence" />
				</div>
				<div class="aspect-[4/5] bg-surface-container-high overflow-hidden grayscale">
					<img class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuBuES9GuAeumiik0NoxgGbF9bxHkFe5QY4Bs7vz6xxWQ5QNSaBqFXZ4ITrPS2GDPihknKRC8SA7-hQa5r_K4okmFw2To0ip9LxBinOdT4fJXwufy5cdVANJA1hwoekTndYr8hw6vfRryKoFAwKGAde0deYTqzSXJ6SNbKHga2iJnIeN9lRJz_DzGPNNaD3fWN5DvhB6ODaA_06IOxuBbAr3uuAWHGYGMC-ePSqf2wvPuwJp_Eh_jQHXZDMvqi46QiKWpbKzbrvulKs" alt="target" />
				</div>
			</div>
			<div class="lg:col-span-4 space-y-6">
				<div class="space-y-4 pt-4">
					<button class="w-full bg-primary-container text-on-primary-container py-4 font-bold uppercase tracking-[0.2em] hover:bg-primary transition-all press-scale">APPROVE_KILL</button>
					<button class="w-full border-2 border-error-container text-error py-4 font-bold uppercase tracking-[0.2em] press-scale">REJECT_DISPUTE</button>
				</div>
			</div>
		</div>
	</section>
</main>
