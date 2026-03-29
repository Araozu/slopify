import { queryOptions } from '@tanstack/svelte-query';
import type { QueryClient } from '@tanstack/svelte-query';
import { createThread, listThreadMessages, listThreads } from '$lib/thread-client';
import type { Thread } from '$lib/types';

export const threadKeys = {
	all: ['threads'] as const,
	messages: (threadId: string) => [...threadKeys.all, threadId, 'messages'] as const
};

export function threadsQueryOptions() {
	return queryOptions({
		queryKey: threadKeys.all,
		queryFn: ({ signal }) => listThreads(signal)
	});
}

export function threadMessagesQueryOptions(threadId: string) {
	return queryOptions({
		queryKey: threadKeys.messages(threadId),
		queryFn: ({ signal }) => listThreadMessages(threadId, signal)
	});
}

export async function ensureFirstThread(queryClient: QueryClient): Promise<Thread> {
	const threads = await queryClient.fetchQuery(threadsQueryOptions());
	const firstThread = threads[0];

	if (firstThread) {
		return firstThread;
	}

	const createdThread = await createThread();
	queryClient.setQueryData<Thread[]>(threadKeys.all, [createdThread]);

	return createdThread;
}
