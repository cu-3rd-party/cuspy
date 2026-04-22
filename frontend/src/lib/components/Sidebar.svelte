<script lang="ts">
	import { page } from '$app/state';
	import { m } from '$lib/paraglide/messages.js';
</script>

<aside class="hidden lg:flex flex-col h-[calc(100vh-64px)] w-80 bg-background shadow-[20px_0_60px_rgba(129,0,143,0.08)] p-8 space-y-4 fixed left-0 top-16 z-40 border-r border-outline-variant/10">
	<div class="flex flex-col space-y-2 mb-10 pb-6 border-b border-outline-variant/15">
		<div class="flex items-center gap-3">
			<div class="w-12 h-12 bg-surface-container-high flex items-center justify-center">
				<span class="material-symbols-outlined text-secondary-container text-3xl">shield</span>
			</div>
			<div>
				<p class="text-secondary-container font-black tracking-widest font-headline uppercase">OPERATIVE_X</p>
				<p class="text-[10px] text-on-surface-variant font-headline uppercase">CLEARANCE: PHANTOM</p>
			</div>
		</div>
		<p class="text-[9px] tracking-widest text-secondary-container/60 mt-2 uppercase font-label">ENCRYPTION ACTIVE</p>
	</div>
	<nav class="flex flex-col gap-2">
		<a href="/admin/moderation" class="group flex items-center gap-4 p-3 font-headline uppercase font-medium transition-all {page.url.pathname === '/admin/moderation' ? 'bg-surface-container text-secondary pl-6' : 'text-on-surface opacity-70 hover:bg-surface-container-high hover:pl-6'}">
			<span class="material-symbols-outlined">verified_user</span>
			<span>MODERATION</span>
		</a>
		<a href="/surveillance" class="group flex items-center gap-4 p-3 font-headline uppercase font-medium transition-all {page.url.pathname === '/surveillance' ? 'bg-surface-container text-secondary pl-6' : 'text-on-surface opacity-70 hover:bg-surface-container-high hover:pl-6'}">
			<span class="material-symbols-outlined">group</span>
			<span>DATABASE</span>
		</a>
		<a href="/admin/events" class="group flex items-center gap-4 p-3 font-headline uppercase font-medium transition-all {page.url.pathname === '/admin/events' ? 'bg-surface-container text-secondary pl-6' : 'text-on-surface opacity-70 hover:bg-surface-container-high hover:pl-6'}">
			<span class="material-symbols-outlined">language</span>
			<span>GLOBAL_EVENTS</span>
		</a>
	</nav>
	<div class="mt-auto bg-secondary-container/10 p-4 border-l-2 border-secondary">
		<p class="text-[11px] font-headline text-secondary font-bold mb-1 uppercase">Transmission Warning</p>
		<p class="text-[10px] leading-relaxed text-on-surface-variant/80">All data packets are logged. Misreporting is a terminal offense.</p>
	</div>
</aside>
