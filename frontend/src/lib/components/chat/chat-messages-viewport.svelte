<script lang="ts">
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { Message } from '$lib/types';
	import ChatMessageBubble from './chat-message-bubble.svelte';

	interface Props {
		viewportRef?: HTMLElement | null;
		loadError: string;
		isBootstrapping: boolean;
		isLoadingMessages: boolean;
		messages: Message[];
		onDeletePair?: (messageId: string) => void;
		onFork?: (messageId: string) => void;
	}

	let {
		viewportRef = $bindable(null),
		loadError,
		isBootstrapping,
		isLoadingMessages,
		messages,
		onDeletePair,
		onFork
	}: Props = $props();
</script>

<ScrollArea.Root class="min-h-0 flex-1" bind:viewportRef>
	<div class="flex min-h-full flex-col justify-end">
		{#if loadError}
			<div
				class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
			>
				<p
					class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
				>
					{loadError}
				</p>
			</div>
		{:else if isBootstrapping || isLoadingMessages}
			<div
				class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
			>
				<p class="text-sm text-muted-foreground">
					{isLoadingMessages ? 'Loading messages...' : 'Loading threads...'}
				</p>
			</div>
		{:else if messages.length === 0}
			<div
				class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
			>
				<div class="max-w-sm rounded-3xl border bg-background/80 px-6 py-8 text-center shadow-sm">
					<h2 class="text-base font-semibold">Empty thread</h2>
					<p class="mt-2 text-sm text-muted-foreground">
						Send the first message to start this chat.
					</p>
				</div>
			</div>
		{:else}
			<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
				{#each messages as message (message.id)}
					<ChatMessageBubble {message} {onDeletePair} {onFork} />
				{/each}
			</div>
		{/if}
	</div>
</ScrollArea.Root>
