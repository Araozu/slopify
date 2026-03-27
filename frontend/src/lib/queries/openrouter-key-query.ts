import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listOpenRouterKeys } from '$lib/openrouter-key-client';

export const openRouterKeyKeys = {
	all: ['openrouter-keys'] as const
};

export function openRouterKeysQueryOptions() {
	return queryOptions({
		queryKey: openRouterKeyKeys.all,
		queryFn: ({ signal }) => listOpenRouterKeys(signal)
	});
}

export async function invalidateOpenRouterKeys(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: openRouterKeyKeys.all });
}
