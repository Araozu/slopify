import { queryOptions } from '@tanstack/svelte-query';
import { getCurrentUser } from '$lib/auth-client';

export const authKeys = {
	currentUser: ['auth', 'current-user'] as const
};

export function currentUserQueryOptions() {
	return queryOptions({
		queryKey: authKeys.currentUser,
		queryFn: ({ signal }) => getCurrentUserWithAbort(signal)
	});
}

async function getCurrentUserWithAbort(signal?: AbortSignal) {
	if (!signal) {
		return getCurrentUser();
	}

	const response = await fetch('/api/v1/auth/me', {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as import('$lib/types').AuthUser | { error?: string };

	if (!response.ok || !('id' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to load your session.');
	}

	return payload;
}
