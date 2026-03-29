<script lang="ts">
	import { PlusIcon, TrashIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { cn } from '$lib/utils';

	interface ThreadListItem {
		id: string;
		title: string;
		lastMessage: string;
	}

	interface Props {
		collapsed: boolean;
		chatThreads: ThreadListItem[];
		threadId: string;
		isCreatingThread: boolean;
		isDeletingThread: boolean;
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
		onCreateThread,
		onSelectThread,
		onDeleteThread
	}: Props = $props();
</script>

<aside
	id="thread-sidebar"
	class={cn(
		'flex min-h-0 w-64 flex-col border-r bg-muted/30 backdrop-blur-md',
		collapsed && 'hidden'
	)}
	aria-hidden={collapsed}
>
	<div class="flex items-center justify-between p-4">
		<h2 class="text-[10px] font-black text-foreground/40 uppercase">Recent</h2>
		<Button
			variant="ghost"
			size="icon-xs"
			class="rounded-full hover:bg-primary/10 hover:text-primary"
			disabled={isCreatingThread}
			onclick={onCreateThread}
		>
			<PlusIcon size={14} />
		</Button>
	</div>
	<ScrollArea.Root class="flex-1">
		<div class="space-y-2 p-2">
			{#each chatThreads as chat (chat.id)}
				<div
					class={cn(
						'group flex w-full items-stretch gap-0.5 rounded-xl transition-all',
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
						<p class="line-clamp-1 text-[11px] text-muted-foreground/70">{chat.lastMessage}</p>
					</button>
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
				</div>
			{/each}
		</div>
	</ScrollArea.Root>
</aside>
