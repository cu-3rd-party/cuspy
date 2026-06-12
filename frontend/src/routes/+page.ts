import type { PageLoad } from './$types';

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const clearances = ['LEVEL 3-SPECTER', 'LEVEL 4-OMEGA', 'LEVEL 5-ECLIPSE', 'LEVEL 6-NOIR'];

const makeRefId = () => {
	const digits = Math.floor(100 + Math.random() * 900);
	const suffix = ['GHOST', 'RAVEN', 'VOID', 'PHANTOM'][Math.floor(Math.random() * 4)];
	return `PX-${digits}-${suffix}`;
};

export const load: PageLoad = async () => {
	const verification = (async () => {
		await delay(3000);

		return {
			refId: makeRefId(),
			clearance: clearances[Math.floor(Math.random() * clearances.length)],
			grantedAt: new Date().toISOString()
		};
	})();

	return {
		verification
	};
};
