import type { Reroute } from '@sveltejs/kit';
import { deLocalizeUrl } from '$lib/paraglide/runtime';

const spaPaths = new Set([
	'/agent-id',
	'/operational-boundaries',
	'/dossier-verification',
	'/dossier',
	'/waiting-clearance',
	'/target-intel',
	'/surveillance',
	'/missions',
	'/loot',
	'/perks',
	'/report-kill',
	'/reveal-confirmation',
	'/rankings',
	'/admin/moderation',
	'/admin/events'
]);

export const reroute: Reroute = (request) => {
	const pathname = deLocalizeUrl(request.url).pathname;
	return spaPaths.has(pathname) ? '/' : pathname;
};
