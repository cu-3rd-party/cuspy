// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			accessToken: string | null;
			sessionUser: import('$lib/stores/session').SessionUser | null;
		}
		interface PageData {
			sessionFlow?: import('$lib/stores/session').SessionFlow;
			sessionUser?: import('$lib/stores/session').SessionUser | null;
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
