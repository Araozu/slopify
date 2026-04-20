<script lang="ts">
	import { PlusIcon, SpinnerIcon, TrashIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { cn } from '$lib/utils';
	import type { Tag } from '$lib/types';

	interface ThreadListItem {
		id: string;
		title: string;
		lastMessage: string;
		tags?: Tag[];
		isOptimistic?: boolean;
	}

	interface Props {
		collapsed: boolean;
		chatThreads: ThreadListItem[];
		threadId: string;
		isCreatingThread: boolean;
		isDeletingThread: boolean;
		availableTags: Tag[];
		onCreateThread: () => void;
		onSelectThread: (id: string) => void;
		onDeleteThread: (id: string) => void;
	}

	let {
		collapsed,
		chatThreads,
		threadId,
		isCreatingThread,
		isDeletingThread,
		availableTags,
		onCreateThread,
		onSelectThread,
		onDeleteThread
	}: Props = $props();

	let activeTagFilter = $state<string | null>(null);

	const visibleThreads = $derived(
		activeTagFilter
			? chatThreads.filter((t) => t.tags?.some((tag) => tag.id === activeTagFilter))
			: chatThreads
	);
</script>

<aside
	id="thread-sidebar"
	class={cn(
		'absolute inset-y-0 left-0 z-40 flex min-h-0 w-64 flex-col border-r bg-muted/30 backdrop-blur-md md:static md:z-auto',
		collapsed && 'hidden'
	)}
	aria-hidden={collapsed}
>
	<div class="p-3">
		<Button
			variant="default"
			class="w-full gap-2"
			disabled={isCreatingThread}
			onclick={onCreateThread}
		>
			{#if isCreatingThread}
				<SpinnerIcon size={14} class="animate-spin" />
				Creating…
			{:else}
				<PlusIcon size={14} />
				New Thread
			{/if}
		</Button>
	</div>
	<div class="px-4 pb-1">
		<h2 class="text-[10px] font-black text-foreground/40 uppercase">Recent</h2>
	</div>

	{#if availableTags.length > 0}
		<div class="flex flex-wrap gap-1 px-3 pb-2">
			{#each availableTags as tag (tag.id)}
				<button
					type="button"
					class={cn(
						'inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium transition-opacity',
						activeTagFilter === tag.id
							? 'opacity-100 ring-2 ring-border'
							: 'opacity-60 hover:opacity-90'
					)}
					style="background-color: {tag.color}; color: white;"
					onclick={() => (activeTagFilter = activeTagFilter === tag.id ? null : tag.id)}
					title={activeTagFilter === tag.id
						? `Clear filter: ${tag.name}`
						: `Filter by: ${tag.name}`}
				>
					{tag.name}
				</button>
			{/each}
		</div>
	{/if}

	<ScrollArea.Root class="flex-1">
		<div class="space-y-2 p-2">
			{#each visibleThreads as chat (chat.id)}
				<div
					class={cn(
						'group flex w-full items-stretch gap-0.5 rounded-lg transition-all',
						threadId === chat.id
							? 'bg-background shadow-sm ring-1 shadow-primary/5 ring-border'
							: 'hover:bg-muted/80'
					)}
				>
					<button
						type="button"
						onclick={() => onSelectThread(chat.id)}
						class="flex min-w-0 flex-1 flex-col gap-1 px-3 py-1.5 text-left"
					>
						<div class="flex items-center justify-between">
							<span
								class={cn(
									'truncate text-sm font-semibold transition-colors',
									threadId === chat.id
										? 'text-primary'
										: 'text-foreground/80 group-hover:text-foreground'
								)}>{chat.title}</span
							>
						</div>
						{#if chat.tags && chat.tags.length > 0}
							<div class="flex flex-wrap gap-1">
								{#each chat.tags as tag (tag.id)}
									<span
										class="inline-block h-2 w-2 rounded-full"
										style="background-color: {tag.color}"
										title={tag.name}
									></span>
								{/each}
							</div>
						{/if}
					</button>
					{#if !chat.isOptimistic}
						<Button
							type="button"
							variant="ghost"
							size="icon-xs"
							class="shrink-0 self-center text-muted-foreground hover:text-destructive"
							disabled={isDeletingThread}
							onclick={(event) => {
								event.stopPropagation();
								onDeleteThread(chat.id);
							}}
							title="Delete thread"
						>
							<TrashIcon size={14} />
						</Button>
					{/if}
				</div>
			{/each}
		</div>
	</ScrollArea.Root>
</aside>
