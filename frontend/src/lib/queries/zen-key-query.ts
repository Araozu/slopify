import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listZenKeys } from '$lib/zen-key-client';

export const zenKeyKeys = {
	all: ['zen-keys'] as const
};

export function zenKeysQueryOptions() {
	return queryOptions({
		queryKey: zenKeyKeys.all,
		queryFn: ({ signal }) => listZenKeys(signal)
	});
}

export async function invalidateZenKeys(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: zenKeyKeys.all });
}
