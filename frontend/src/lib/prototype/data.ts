import { m } from '$lib/paraglide/messages.js';

export type TopBarConfig = {
	title: string;
	icon: string;
	subtitle?: string;
	meta?: string;
	avatar?: string;
	backHref?: string;
};

export type BottomNavItem = {
	href: string;
	label: string;
	icon: string;
	group: 'enlist' | 'dossier' | 'gameplay';
	match?: 'exact' | 'prefix';
	fill?: boolean;
};

export const enlistNav: BottomNavItem[] = [
	{
		href: '/agent-id',
		label: m.nav_id(),
		icon: 'fingerprint',
		group: 'enlist',
		match: 'prefix',
		fill: true
	},
	{
		href: '/operational-boundaries',
		label: m.nav_boundaries(),
		icon: 'assignment_ind',
		group: 'enlist'
	},
	{
		href: '/dossier-verification',
		label: m.nav_verify(),
		icon: 'refresh',
		group: 'enlist',
	},
	{
		href: '/dossier',
		label: m.nav_waiting(),
		icon: 'security',
		group: 'enlist',
	},
];

export const dossierNav: BottomNavItem[] = [
	{
		href: '/dossier',
		label: m.nav_dossier(),
		icon: 'account_circle',
		group: 'dossier',
		match: 'prefix',
		fill: true
	},
	{ href: '/target-intel', label: m.nav_intel(), icon: 'target', group: 'dossier' },
	{ href: '/rankings', label: m.nav_rank(), icon: 'leaderboard', group: 'dossier' },
	{ href: '/', label: m.nav_rules(), icon: 'gavel', group: 'dossier' }
];

export const gameplayNav: BottomNavItem[] = [
	{ href: '/target-intel', label: 'INTEL', icon: 'monitoring', group: 'gameplay' },
	{ href: '/surveillance', label: 'TARGETS', icon: 'android_fingerprint', group: 'gameplay' },
	{ href: '/missions', label: 'MISSIONS', icon: 'assignment', group: 'gameplay' },
	{ href: '/loot', label: 'GEAR', icon: 'inventory_2', group: 'gameplay' },
	{ href: '/perks', label: 'PERKS', icon: 'bolt', group: 'gameplay' }
];

export const agentAvatar =
	'https://lh3.googleusercontent.com/aida-public/AB6AXuB8C3wgoiqdVcapGg7c7DDcz8iysCidMlJz3J9Fx-HOuA-m4H327QZtLp6TW04c0uULzo0-8Tft_PihOg3wiHCMJlI5vx9LPpZoYaObxbGPqSZDeRidnmc6-D8prtzBRudpYf6fS3D_kIq6uFEMyMeRIpn8IacL87JqvzSE3XY5pwgj87_072qq34adD22W3uBfb54C8F7ftLOvVRYvomjDTDyb96AIDSoraoUrzEVyULqb8U8bsC23Us9vtm1j0kvGAWiCOaSYXvI';

export const heroServerImage =
	'https://lh3.googleusercontent.com/aida-public/AB6AXuD8aavccR0ceGp29YD3ixRhjPB_KbYtMVtAF162BAdofaplks8nSrf0h4oPOhKZG7tJxrCfvEBdLuoTyzfkfKN_7S5QUQyPrixZLmoJRu4v85YGg8vAyA22FmO3wwtHRo7nyMbpKEFt4OX3qSoQ2JocRimKJ9GIRUyF6vWpUxf6m1fgme86KXc83DgmYCnh809HuhE6TwzWfb0KqZzHyJQDAu70BvSHLqlTgnv0DWpO8uDKf8M8VSc04kCvUng_IhUL919ZM54MQII';

export const verificationImage =
	'https://lh3.googleusercontent.com/aida-public/AB6AXuASn-ohXf3Umktly1O3QUTDhFXzGZ-L2pD4dcZOxaf1hsOU3UNCHMYEEnjL3pzo09RgDGW7zvRFj9C2bkNbZHSd_QDxzbVP9fqJAns3fIrUWk0ZwP-djIgIWhNfWFYw1i49ShpcaVcPFAQFgGu7IjGuNzLdVBnILGz_5vVWE25JeYpW98gX65zQ0TCag44JhJ1tx6R4_NT_BJb8xTLAHiV4FF5Ln7y7M0n0BqUBY6PqBaBcwE9UTS009SGkmVHopokLQTQQzfBqEUM';

export const boundariesImage =
	'https://lh3.googleusercontent.com/aida-public/AB6AXuCuEZLWnSjtB4ZrMuyVRfFVn_xQTMR0jLQ-G8mbSluh2FJpeQySFhcv_Bnz9EFMWQhrmCe08rSLiLdHNGKJjttFu4MHV0jow5EmSSGj2_5dG4i2IIKkz6d-f3b23SpK0usVFFt77W4YdUgjhn-U5tv2GyGU0pFzsIZxOwUYgt7FR1cz1n3Y4AqmZGjFy9yz6RINZ3PPzk99mfBH-UnfQaGscOc_mfMI-6kY9P6GOcUBhfYPa5fP6rW6FbuaMLvr1Q9ZSa7S3kjGDYA';

export const targetImage =
	'https://lh3.googleusercontent.com/aida-public/AB6AXuDm6G9ZSKvPnaT5iOyg2PtnlML0Ts21HfKT_eWEMkOp8OCfmrxfuijiba_jA5p8w8_gjne3kWc1kcYSjMlta2lkuLUJRoGSD6q-RVYX76jt2ZbsClXs49RKMvTww8QVcqPkeXvRLEqEjNGOPPx4Ml2ghfr9EhE9s21XQl5onvyrUEyQoW2qwtj3Dazk7gNlmDxjhdbSdoyXVHhEV6p5CqxFHgPXyTgyJHcu--2IuXT0wGL3rRj2WnwsJ4x2qdUq6KZz9cZVtcoo0_Y';

export const roster = [
	{ rank: '01', name: 'V_SPECTRE', syndicate: 'NEON_RAVEN', rating: '2,840', discoveries: '1,402' },
	{ rank: '02', name: 'K0_BALT', syndicate: 'INDEPENDENT', rating: '2,715', discoveries: '988' },
	{
		rank: '12',
		name: 'AGENT_092',
		syndicate: 'ACTIVE_SESSION',
		rating: '1,942',
		discoveries: '431',
		active: true
	},
	{ rank: '13', name: 'NIGHT_O_L', syndicate: 'CRYPTIC_CORE', rating: '1,899', discoveries: '428' },
	{ rank: '14', name: 'S_SHADOW', syndicate: 'VOID_RUNNERS', rating: '1,854', discoveries: '412' }
];
