import { writable } from 'svelte/store';

export type SessionUser = {
	user_id: string;
	telegram_id: number;
	rating: number | null;
	agent_name: string | null;
	agent_data: Record<string, unknown>;
	created_at: string;
	updated_at: string | null;
};

export const sessionUser = writable<SessionUser | null>(null);
