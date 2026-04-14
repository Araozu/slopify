import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listCopilotModels } from '$lib/copilot-model-client';

export const copilotModelKeys = {
	all: ['copilot-models'] as const
};

export function copilotModelsQueryOptions() {
	return queryOptions({
		queryKey: copilotModelKeys.all,
		queryFn: ({ signal }) => listCopilotModels(signal)
	});
}

export async function invalidateCopilotModels(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: copilotModelKeys.all });
}
