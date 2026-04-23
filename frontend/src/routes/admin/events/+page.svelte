<script lang="ts">
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/TopBar.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import type { ActionData, PageProps } from './$types';
	import type { KillReport } from '$lib/stores/session';

	let { data, form }: PageProps = $props();
	let actionState = $derived(form as ActionData | null);
	let reports = $derived(data.reports as KillReport[]);
	let activeId = $state('');
	let reviewerNote = $state('');
	let activeReport = $derived(reports.find((report) => report.kill_report_id === activeId) ?? reports[0] ?? null);
	let pendingCount = $derived(reports.filter((report) => report.status === 'pending').length);

	const selectReport = (reportId: string, note: string | null) => {
		activeId = reportId;
		reviewerNote = note ?? '';
	};
</script>

<TopBar config={{ title: 'PROTOCOL_OVERRIDE', icon: 'language' }} />

<main class="p-8 lg:p-12 space-y-12 relative overflow-y-auto">
	<div class="border-l-4 border-primary pl-6">
		<h2 class="text-5xl font-headline font-black tracking-tighter uppercase italic">PROTOCOL_OVERRIDE</h2>
		<p class="text-on-surface-variant font-label tracking-widest text-xs mt-2 uppercase">GLOBAL COMMAND AUTHORIZATION REQUIRED</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
		<section class="lg:col-span-4 space-y-6">
			<div class="bg-surface-container p-6 border border-outline-variant/10">
				<div class="text-xs font-label tracking-[0.3em] text-primary uppercase">Pending reports</div>
				<div class="mt-3 text-4xl font-headline font-black text-primary">{pendingCount}</div>
			</div>

			{#if reports.length === 0}
				<div class="bg-surface-container p-6 border border-outline-variant/10">
					<h3 class="font-headline text-xl font-bold uppercase">NO_PENDING_REPORTS</h3>
					<p class="mt-3 text-sm text-on-surface-variant">Kill confirmations and disputes will appear here.</p>
				</div>
			{:else}
				{#each reports as report (report.kill_report_id)}
					<button type="button" onclick={() => selectReport(report.kill_report_id, report.reviewer_note)} class={`w-full border p-4 text-left transition-colors ${report.kill_report_id === activeId ? 'border-primary bg-surface-container-high' : 'border-outline-variant/10 bg-surface-container hover:border-primary/40'}`}>
						<div class="flex items-start justify-between gap-4">
							<div>
								<div class="font-headline text-lg font-bold uppercase">{report.target_identifier}</div>
								<div class="mt-2 text-[10px] font-label tracking-[0.2em] text-on-surface-variant uppercase">REPORTER: {report.reporter_codename}</div>
							</div>
							<div class={`px-2 py-1 text-[10px] font-label uppercase ${report.status === 'pending' ? 'bg-primary-container text-on-primary-container' : report.status === 'confirmed' ? 'bg-secondary-container text-on-secondary-container' : 'bg-error text-white'}`}>{report.status}</div>
						</div>
					</button>
				{/each}
			{/if}
		</section>

		<section class="lg:col-span-8 space-y-6">
			{#if actionState?.error}
				<div class="bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase">{actionState.error}</div>
			{/if}

			{#if activeReport}
				<div class="bg-surface-container p-8 relative overflow-hidden group border border-outline-variant/10">
					<h3 class="font-headline font-bold text-lg mb-8 flex items-center gap-2 uppercase">
						<Icon name="public" class="text-primary" />KILL_REPORT_REVIEW
					</h3>

					<div class="grid md:grid-cols-2 gap-6">
						<div class="bg-surface-container-high p-6 space-y-3 border-l border-primary">
							<div class="text-[10px] font-label tracking-[0.2em] text-outline uppercase">Target</div>
							<div class="font-headline text-2xl font-bold uppercase">{activeReport.target_identifier}</div>
							<div class="text-[10px] font-label tracking-[0.2em] text-on-surface-variant uppercase">REPORTER: {activeReport.reporter_codename}</div>
						</div>

						<div class="bg-surface-container-high p-6 space-y-3 border-l border-secondary">
							<div class="text-[10px] font-label tracking-[0.2em] text-outline uppercase">Witness present</div>
							<div class="font-headline text-2xl font-bold uppercase">{activeReport.witness_present ? 'YES' : 'NO'}</div>
							<div class="text-[10px] font-label tracking-[0.2em] text-on-surface-variant uppercase">STATUS: {activeReport.status}</div>
						</div>
					</div>

					<div class="mt-6 bg-surface-container-high p-6 border-l border-outline-variant/20">
						<div class="text-[10px] font-label tracking-[0.2em] text-outline uppercase">Modus operandi</div>
						<p class="mt-3 text-sm leading-relaxed text-on-surface-variant">{activeReport.modus_operandi}</p>
					</div>

					<form method="POST" action="?/moderate" use:enhance class="mt-6 space-y-4 bg-surface-container-low p-6">
						<input type="hidden" name="reportId" value={activeReport.kill_report_id} />
						<div>
							<label for="reviewer-note" class="mb-3 block text-[10px] font-label tracking-[0.2em] text-on-surface-variant uppercase">Reviewer Note</label>
							<textarea id="reviewer-note" name="reviewerNote" bind:value={reviewerNote} class="min-h-32 w-full bg-surface-container-lowest p-3 text-sm text-on-surface focus:outline-none" placeholder="Confirm evidence or document rejection reason."></textarea>
						</div>

						<div class="grid gap-3 md:grid-cols-2">
							<button name="decision" value="confirmed" class="w-full bg-primary-container py-4 font-bold tracking-[0.2em] text-on-primary-container uppercase transition-colors hover:bg-primary">CONFIRM_KILL</button>
							<button name="decision" value="rejected" class="w-full border-2 border-error-container py-4 font-bold tracking-[0.2em] text-error uppercase transition-colors hover:bg-error hover:text-white">REJECT_KILL</button>
						</div>
					</form>
				</div>
			{/if}
		</section>
	</div>
</main>
