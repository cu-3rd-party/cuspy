<script lang="ts">
	import { TopBar } from '$lib/shared/ui';
	import { Icon } from '$lib/shared/ui';
	import { moderateKillReport } from '$lib/shared/api';
	import { getAppContext } from '$lib/shared/providers';
	import type { KillReport, SessionFlow, SessionUser } from '$lib/shared/model';

	let { data } = $props<{
		data: {
			sessionFlow: SessionFlow;
			sessionUser: SessionUser | null;
			reports: KillReport[];
		};
	}>();
	const app = getAppContext();
	let loadedFromApp = $state(false);
	let reports = $derived((loadedFromApp ? app.killReports : data.reports) as KillReport[]);
	let activeId = $state('');
	let reviewerNote = $state('');
	let actionError = $state('');
	let isSubmitting = $state(false);
	let activeReport = $derived(
		reports.find((report) => report.kill_report_id === activeId) ?? reports[0] ?? null
	);
	let pendingCount = $derived(reports.filter((report) => report.status === 'pending').length);

	const selectReport = (reportId: string, note: string | null) => {
		activeId = reportId;
		reviewerNote = note ?? '';
	};

	const handleModeration = async (decision: 'confirmed' | 'rejected') => {
		if (!activeReport) return;

		isSubmitting = true;
		actionError = '';

		try {
			await moderateKillReport({ reportId: activeReport.kill_report_id, decision, reviewerNote });
			await app.loadKillReports();
			loadedFromApp = true;
		} catch (error) {
			actionError = error instanceof Error ? error.message : 'Moderation request failed.';
		} finally {
			isSubmitting = false;
		}
	};
</script>

<TopBar config={{ title: 'PROTOCOL_OVERRIDE', icon: 'language' }} />

<main class="relative space-y-12 overflow-y-auto p-8 lg:p-12">
	<div class="border-l-4 border-primary pl-6">
		<h2 class="font-headline text-5xl font-black tracking-tighter uppercase italic">
			PROTOCOL_OVERRIDE
		</h2>
		<p class="mt-2 font-label text-xs tracking-widest text-on-surface-variant uppercase">
			GLOBAL COMMAND AUTHORIZATION REQUIRED
		</p>
	</div>

	<div class="grid grid-cols-1 gap-8 lg:grid-cols-12">
		<section class="space-y-6 lg:col-span-4">
			<div class="border border-outline-variant/10 bg-surface-container p-6">
				<div class="font-label text-xs tracking-[0.3em] text-primary uppercase">
					Pending reports
				</div>
				<div class="mt-3 font-headline text-4xl font-black text-primary">{pendingCount}</div>
			</div>

			{#if reports.length === 0}
				<div class="border border-outline-variant/10 bg-surface-container p-6">
					<h3 class="font-headline text-xl font-bold uppercase">NO_PENDING_REPORTS</h3>
					<p class="mt-3 text-sm text-on-surface-variant">
						Kill confirmations and disputes will appear here.
					</p>
				</div>
			{:else}
				{#each reports as report (report.kill_report_id)}
					<button
						type="button"
						onclick={() => selectReport(report.kill_report_id, report.reviewer_note)}
						class={`w-full border p-4 text-left transition-colors ${report.kill_report_id === activeId ? 'border-primary bg-surface-container-high' : 'border-outline-variant/10 bg-surface-container hover:border-primary/40'}`}
					>
						<div class="flex items-start justify-between gap-4">
							<div>
								<div class="font-headline text-lg font-bold uppercase">
									{report.target_identifier}
								</div>
								<div
									class="mt-2 font-label text-[10px] tracking-[0.2em] text-on-surface-variant uppercase"
								>
									REPORTER: {report.reporter_codename}
								</div>
							</div>
							<div
								class={`px-2 py-1 font-label text-[10px] uppercase ${report.status === 'pending' ? 'bg-primary-container text-on-primary-container' : report.status === 'confirmed' ? 'bg-secondary-container text-on-secondary-container' : 'bg-error text-white'}`}
							>
								{report.status}
							</div>
						</div>
					</button>
				{/each}
			{/if}
		</section>

		<section class="space-y-6 lg:col-span-8">
			{#if actionError}
				<div
					class="bg-error px-4 py-3 font-label text-[11px] tracking-[0.16em] text-white uppercase"
				>
					{actionError}
				</div>
			{/if}

			{#if activeReport}
				<div
					class="group relative overflow-hidden border border-outline-variant/10 bg-surface-container p-8"
				>
					<h3 class="mb-8 flex items-center gap-2 font-headline text-lg font-bold uppercase">
						<Icon name="public" class="text-primary" />KILL_REPORT_REVIEW
					</h3>

					<div class="grid gap-6 md:grid-cols-2">
						<div class="space-y-3 border-l border-primary bg-surface-container-high p-6">
							<div class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
								Target
							</div>
							<div class="font-headline text-2xl font-bold uppercase">
								{activeReport.target_identifier}
							</div>
							<div
								class="font-label text-[10px] tracking-[0.2em] text-on-surface-variant uppercase"
							>
								REPORTER: {activeReport.reporter_codename}
							</div>
						</div>

						<div class="space-y-3 border-l border-secondary bg-surface-container-high p-6">
							<div class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
								Witness present
							</div>
							<div class="font-headline text-2xl font-bold uppercase">
								{activeReport.witness_present ? 'YES' : 'NO'}
							</div>
							<div
								class="font-label text-[10px] tracking-[0.2em] text-on-surface-variant uppercase"
							>
								STATUS: {activeReport.status}
							</div>
						</div>
					</div>

					<div class="mt-6 border-l border-outline-variant/20 bg-surface-container-high p-6">
						<div class="font-label text-[10px] tracking-[0.2em] text-outline uppercase">
							Modus operandi
						</div>
						<p class="mt-3 text-sm leading-relaxed text-on-surface-variant">
							{activeReport.modus_operandi}
						</p>
					</div>

					<div class="mt-6 space-y-4 bg-surface-container-low p-6">
						<div>
							<label
								for="reviewer-note"
								class="mb-3 block font-label text-[10px] tracking-[0.2em] text-on-surface-variant uppercase"
								>Reviewer Note</label
							>
							<textarea
								id="reviewer-note"
								bind:value={reviewerNote}
								class="min-h-32 w-full bg-surface-container-lowest p-3 text-sm text-on-surface focus:outline-none"
								placeholder="Confirm evidence or document rejection reason."
							></textarea>
						</div>

						<div class="grid gap-3 md:grid-cols-2">
							<button
								type="button"
								onclick={() => handleModeration('confirmed')}
								disabled={isSubmitting}
								class="w-full bg-primary-container py-4 font-bold tracking-[0.2em] text-on-primary-container uppercase transition-colors hover:bg-primary"
								>CONFIRM_KILL</button
							>
							<button
								type="button"
								onclick={() => handleModeration('rejected')}
								disabled={isSubmitting}
								class="w-full border-2 border-error-container py-4 font-bold tracking-[0.2em] text-error uppercase transition-colors hover:bg-error hover:text-white"
								>REJECT_KILL</button
							>
						</div>
					</div>
				</div>
			{/if}
		</section>
	</div>
</main>
