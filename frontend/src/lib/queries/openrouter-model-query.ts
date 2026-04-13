import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listOpenRouterModels } from '$lib/openrouter-model-client';

export const openRouterModelKeys = {
	all: ['openrouter-models'] as const
};

export function openRouterModelsQueryOptions() {
	return queryOptions({
		queryKey: openRouterModelKeys.all,
		queryFn: ({ signal }) => listOpenRouterModels(signal)
	});
}

export async function invalidateOpenRouterModels(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: openRouterModelKeys.all });
}
