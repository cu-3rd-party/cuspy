// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			accessToken: string | null;
			sessionUser: import('$lib/shared/model').SessionUser | null;
		}
		interface PageData {
			sessionFlow?: import('$lib/shared/model').SessionFlow;
			sessionUser?: import('$lib/shared/model').SessionUser | null;
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
