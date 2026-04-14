import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listCopilotTokens } from '$lib/copilot-token-client';

export const copilotTokenKeys = {
	all: ['copilot-tokens'] as const
};

export function copilotTokensQueryOptions() {
	return queryOptions({
		queryKey: copilotTokenKeys.all,
		queryFn: ({ signal }) => listCopilotTokens(signal)
	});
}

export async function invalidateCopilotTokens(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: copilotTokenKeys.all });
}
