import type { Handle } from '@sveltejs/kit';
import { getTextDirection } from '$lib/paraglide/runtime';
import { paraglideMiddleware } from '$lib/paraglide/server';

const handleParaglide: Handle = ({ event, resolve }) =>
	paraglideMiddleware(event.request, ({ request, locale }) => {
		event.request = request;

		return resolve(event, {
			transformPageChunk: ({ html }) =>
				html
					.replace('%paraglide.lang%', locale)
					.replace('%paraglide.dir%', getTextDirection(locale))
		});
	});

const handleSession: Handle = async ({ event, resolve }) => {
	event.locals.accessToken = event.cookies.get('backend-access-token') ?? null;
	event.locals.sessionUser = event.cookies.get('session-user')
		? JSON.parse(event.cookies.get('session-user')!)
		: null;

	return resolve(event);
};

export const handle: Handle = async ({ event, resolve }) =>
	handleParaglide({
		event,
		resolve: (localizedEvent, opts) =>
			handleSession({
				event: localizedEvent,
				resolve: (sessionEvent) => resolve(sessionEvent, opts)
			})
	});
