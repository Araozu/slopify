<script lang="ts">
	import {
		CaretDoubleLeftIcon,
		CaretDoubleRightIcon,
		ChatCircleIcon,
		CheckIcon,
		PencilSimpleIcon,
		TrashIcon,
		XIcon
	} from 'phosphor-svelte';
	import { tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import type { Thread } from '$lib/types';

	interface Props {
		threadTitle: string;
		activeThread: Thread | null;
		threadId: string;
		sidebarCollapsed: boolean;
		isDeletingThread: boolean;
		isRenamingThread: boolean;
		isSending: boolean;
		onRenameThread: (title: string) => Promise<void>;
		onDeleteThread: (id: string) => void;
		onToggleSidebar: () => void;
	}

	let {
		threadTitle,
		activeThread,
		threadId,
		sidebarCollapsed,
		isDeletingThread,
		isRenamingThread,
		isSending,
		onRenameThread,
		onDeleteThread,
		onToggleSidebar
	}: Props = $props();

	let editingTitle = $state(false);
	let titleDraft = $state('');
	let titleInputRef: HTMLInputElement | null = $state(null);

	$effect(() => {
		threadId;
		editingTitle = false;
	});

	async function beginRename() {
		titleDraft = threadTitle;
		editingTitle = true;
		await tick();
		titleInputRef?.focus();
		titleInputRef?.select();
	}

	async function commitRename() {
		const next = titleDraft.trim();
		if (next === threadTitle) {
			editingTitle = false;
			return;
		}
		try {
			await onRenameThread(next);
			editingTitle = false;
		} catch {
			/* parent surfaces error; stay in edit mode */
		}
	}

	function cancelRename() {
		editingTitle = false;
		titleDraft = threadTitle;
	}

	function onTitleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			void commitRename();
		} else if (event.key === 'Escape') {
			event.preventDefault();
			cancelRename();
		}
	}

	let headerBusy = $derived(isDeletingThread || isRenamingThread || isSending);
</script>

<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl">
	<div class="flex min-w-0 flex-1 items-center gap-2">
		<div
			class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
		>
			<ChatCircleIcon size={14} weight="fill" />
		</div>
		{#if editingTitle && activeThread}
			<div class="flex min-w-0 flex-1 items-center gap-1">
				<Input
					bind:ref={titleInputRef}
					bind:value={titleDraft}
					class="h-8 min-w-0 flex-1 text-sm font-bold"
					disabled={isRenamingThread}
					aria-label="Thread title"
					onkeydown={onTitleKeydown}
				/>
				<Button
					type="button"
					variant="ghost"
					size="icon-xs"
					class="shrink-0 text-muted-foreground hover:text-foreground"
					disabled={isRenamingThread}
					title="Save title"
					onmousedown={(event) => event.preventDefault()}
					onclick={() => void commitRename()}
				>
					<CheckIcon size={16} />
				</Button>
				<Button
					type="button"
					variant="ghost"
					size="icon-xs"
					class="shrink-0 text-muted-foreground hover:text-foreground"
					disabled={isRenamingThread}
					title="Cancel"
					onmousedown={(event) => event.preventDefault()}
					onclick={cancelRename}
				>
					<XIcon size={16} />
				</Button>
			</div>
		{:else}
			<h1 class="min-w-0 truncate text-sm font-bold tracking-tight">{threadTitle}</h1>
			{#if activeThread}
				<Button
					type="button"
					variant="ghost"
					size="icon-xs"
					class="shrink-0 text-muted-foreground hover:text-foreground"
					disabled={headerBusy}
					title="Rename thread"
					onclick={() => void beginRename()}
				>
					<PencilSimpleIcon size={16} />
				</Button>
			{/if}
		{/if}
	</div>
	<div class="ml-auto flex shrink-0 items-center gap-2">
		{#if activeThread}
			<Button
				variant="ghost"
				size="sm"
				class="gap-1.5 text-muted-foreground hover:text-destructive"
				disabled={headerBusy}
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
			class="gap-1.5 hover:text-foreground"
			onclick={onToggleSidebar}
			aria-expanded={sidebarCollapsed}
			aria-controls="thread-sidebar stream-sidebar"
			title={sidebarCollapsed ? 'Show side panels' : 'Hide side panels'}
		>
			{#if sidebarCollapsed}
				<CaretDoubleLeftIcon size={16} />
				<span>Expand</span>
			{:else}
				<CaretDoubleRightIcon size={16} />
				<span>Collapse</span>
			{/if}
		</Button>
	</div>
</header>
