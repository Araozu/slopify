import { redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/auth-client';

export async function load({ fetch, url }) {
	try {
		await getCurrentUser(fetch);
	} catch {
		return {};
	}

	throw redirect(307, url.searchParams.get('redirectTo') || '/');
}
