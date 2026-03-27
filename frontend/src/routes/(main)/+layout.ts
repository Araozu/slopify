import { redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/auth-client';

export async function load({ fetch, url }) {
	try {
		const user = await getCurrentUser(fetch);
		return { user };
	} catch {
		const redirectTo = `${url.pathname}${url.search}`;
		throw redirect(307, `/auth?redirectTo=${encodeURIComponent(redirectTo)}`);
	}
}
