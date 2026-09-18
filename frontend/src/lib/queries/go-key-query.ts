import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listGoKeys } from '$lib/go-key-client';

export const goKeyKeys = {
	all: ['go-keys'] as const
};

export function goKeysQueryOptions() {
	return queryOptions({
		queryKey: goKeyKeys.all,
		queryFn: ({ signal }) => listGoKeys(signal)
	});
}

export async function invalidateGoKeys(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: goKeyKeys.all });
}
