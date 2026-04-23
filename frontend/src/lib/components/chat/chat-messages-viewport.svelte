<script lang="ts">
	import { ArrowDownIcon } from 'phosphor-svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { Message } from '$lib/types';
	import ChatMessageBubble from './chat-message-bubble.svelte';
	import { contentWidth } from '$lib/stores/ui-preferences';

	interface Props {
		viewportRef?: HTMLElement | null;
		loadError: string;
		isBootstrapping: boolean;
		isLoadingMessages: boolean;
		messages: Message[];
		onDeletePair?: (messageId: string) => void;
		onFork?: (messageId: string) => void;
		onRetry?: (messageId: string) => void;
		onEditResend?: (messageId: string, newText: string) => void;
	}

	let {
		viewportRef = $bindable(null),
		loadError,
		isBootstrapping,
		isLoadingMessages,
		messages,
		onDeletePair,
		onFork,
		onRetry,
		onEditResend
	}: Props = $props();

	let isNearBottom = $state(true);

	const SCROLL_THRESHOLD = 120;

	const maxWidthClass = $derived(
		$contentWidth === 'full'
			? 'max-w-none'
			: $contentWidth === 'wide'
				? 'max-w-[60rem]'
				: 'max-w-3xl'
	);

	function checkNearBottom() {
		if (!viewportRef) return;
		const dist = viewportRef.scrollHeight - (viewportRef.scrollTop + viewportRef.clientHeight);
		isNearBottom = dist <= SCROLL_THRESHOLD;
	}

	function scrollToBottom() {
		viewportRef?.scrollTo({ top: viewportRef.scrollHeight, behavior: 'smooth' });
	}

	$effect(() => {
		const el = viewportRef;
		if (!el) return;
		checkNearBottom();
		el.addEventListener('scroll', checkNearBottom, { passive: true });
		return () => el.removeEventListener('scroll', checkNearBottom);
	});
</script>

<div class="relative min-h-0 flex-1">
	<ScrollArea.Root class="h-full" bind:viewportRef>
		<div class="flex min-h-full flex-col justify-end">
			{#if loadError}
				<div
					class="mx-auto flex w-full {maxWidthClass} flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
				>
					<p
						class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
					>
						{loadError}
					</p>
				</div>
			{:else if isBootstrapping || isLoadingMessages}
				<div
					class="mx-auto flex w-full {maxWidthClass} flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
				>
					<p class="text-sm text-muted-foreground">
						{isLoadingMessages ? 'Loading messages...' : 'Loading threads...'}
					</p>
				</div>
			{:else if messages.length === 0}
				<div
					class="mx-auto flex w-full {maxWidthClass} flex-1 items-center justify-center px-4 py-6 md:px-6 md:py-10"
				>
					<div class="max-w-sm rounded-2xl border bg-background/80 px-6 py-8 text-center shadow-sm">
						<h2 class="text-base font-semibold">Empty thread</h2>
						<p class="mt-2 text-sm text-muted-foreground">
							Send the first message to start this chat.
						</p>
					</div>
				</div>
			{:else}
				<div class="mx-auto w-full {maxWidthClass} space-y-10 px-4 py-6 md:px-6 md:py-10">
					{#each messages as message (message.id)}
						<div id="msg-{message.id}">
							<ChatMessageBubble {message} {onDeletePair} {onFork} {onRetry} {onEditResend} />
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</ScrollArea.Root>

	{#if !isNearBottom}
		<button
			onclick={scrollToBottom}
			class="absolute right-4 bottom-4 flex h-8 w-8 items-center justify-center rounded-full border border-border/60 bg-background/90 text-muted-foreground shadow-md backdrop-blur-sm transition-all hover:bg-background hover:text-foreground active:scale-95"
			aria-label="Scroll to bottom"
			title="Scroll to bottom"
		>
			<ArrowDownIcon size={15} weight="bold" />
		</button>
	{/if}
</div>
