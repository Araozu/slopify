import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listOpenAiTokens } from '$lib/openai-token-client';

export const openAiTokenKeys = {
	all: ['openai-tokens'] as const
};

export function openAiTokensQueryOptions() {
	return queryOptions({
		queryKey: openAiTokenKeys.all,
		queryFn: ({ signal }) => listOpenAiTokens(signal)
	});
}

export async function invalidateOpenAiTokens(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: openAiTokenKeys.all });
}
