import { queryOptions, type QueryClient } from '@tanstack/svelte-query';
import { listSystemPrompts } from '$lib/system-prompt-client';

export const systemPromptKeys = {
	all: ['system-prompts'] as const
};

export function systemPromptsQueryOptions() {
	return queryOptions({
		queryKey: systemPromptKeys.all,
		queryFn: ({ signal }) => listSystemPrompts(signal)
	});
}

export async function invalidateSystemPrompts(queryClient: QueryClient) {
	await queryClient.invalidateQueries({ queryKey: systemPromptKeys.all });
}
