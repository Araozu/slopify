<script lang="ts">
	import {
		CaretDoubleLeftIcon,
		CaretDoubleRightIcon,
		ChatCircleIcon,
		TrashIcon
	} from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import type { Thread } from '$lib/types';

	interface Props {
		threadTitle: string;
		activeThread: Thread | null;
		threadId: string;
		sidebarCollapsed: boolean;
		isDeletingThread: boolean;
		isSending: boolean;
		onDeleteThread: (id: string) => void;
		onToggleSidebar: () => void;
	}

	let {
		threadTitle,
		activeThread,
		threadId,
		sidebarCollapsed,
		isDeletingThread,
		isSending,
		onDeleteThread,
		onToggleSidebar
	}: Props = $props();
</script>

<header class="flex items-center border-b bg-background/20 px-6 py-1 backdrop-blur-xl">
	<div class="flex items-center gap-2">
		<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
			<ChatCircleIcon size={14} weight="fill" />
		</div>
		<h1 class="text-sm font-bold tracking-tight">{threadTitle}</h1>
	</div>
	<div class="ml-auto flex items-center gap-2">
		{#if activeThread}
			<Button
				variant="ghost"
				size="sm"
				class="gap-1.5 text-muted-foreground hover:text-destructive"
				disabled={isDeletingThread || isSending}
				onclick={() => void onDeleteThread(threadId)}
				title="Delete thread"
			>
				<TrashIcon size={16} />
				<span class="hidden sm:inline">Delete</span>
			</Button>
		{/if}
		<Button
			variant="ghost"
			size="sm"
			class="gap-1.5 text-muted-foreground hover:text-foreground"
			onclick={onToggleSidebar}
			aria-expanded={!sidebarCollapsed}
			aria-controls="thread-sidebar stream-sidebar"
			title={sidebarCollapsed ? 'Show side panels' : 'Hide side panels'}
		>
			{#if sidebarCollapsed}
				<CaretDoubleRightIcon size={16} />
				<span>Expand</span>
			{:else}
				<CaretDoubleLeftIcon size={16} />
				<span>Collapse</span>
			{/if}
		</Button>
	</div>
</header>
