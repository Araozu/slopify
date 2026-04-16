import { queryOptions } from '@tanstack/svelte-query';
import { listTags } from '$lib/tag-client';

export const tagKeys = {
	all: ['tags'] as const
};

export function tagsQueryOptions() {
	return queryOptions({
		queryKey: tagKeys.all,
		queryFn: () => listTags()
	});
}
