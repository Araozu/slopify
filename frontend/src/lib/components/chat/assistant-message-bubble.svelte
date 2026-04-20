<script lang="ts">
	import {
		RobotIcon,
		GitForkIcon,
		CopyIcon,
		CheckIcon,
		ArrowCounterClockwiseIcon
	} from 'phosphor-svelte';
	import SvelteMarkdown from 'svelte-markdown';
	import * as Avatar from '$lib/components/ui/avatar';
	import type { Message } from '$lib/types';
	import { showAssistantStreamingText } from '$lib/stores/streaming-preference';
	import {
		formatMessageTimestamp,
		getMessageReasoning,
		getMessageText
	} from './chat-message-utils.js';

	interface Props {
		message: Message;
		onFork?: () => void;
		onRetry?: () => void;
	}

	let { message, onFork, onRetry }: Props = $props();

	let hideStreamingContent = $derived(
		message.status === 'streaming' && !$showAssistantStreamingText
	);

	let copied = $state(false);
	let copyTimer: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		return () => {
			if (copyTimer) clearTimeout(copyTimer);
		};
	});

	function copyText() {
		const text = getMessageText(message);
		navigator.clipboard.writeText(text).then(() => {
			copied = true;
			if (copyTimer) clearTimeout(copyTimer);
			copyTimer = setTimeout(() => {
				copied = false;
				copyTimer = null;
			}, 2000);
		});
	}
</script>

<div
	id={message.id}
	class="group flex w-full animate-in flex-row transition-all duration-500 fade-in slide-in-from-bottom-2"
>
	<div class="relative flex w-full max-w-[85%] flex-col items-start gap-2.5">
		<Avatar.Root
			title="Clanker"
			class="absolute -top-2 -left-2 z-10 h-5 w-5 shadow-sm ring-2 ring-background"
		>
			<Avatar.Fallback class="border border-primary/20 bg-primary/10 text-primary">
				<RobotIcon size={12} />
			</Avatar.Fallback>
		</Avatar.Root>
		<div
			class="assistant-prose prose prose-sm w-full max-w-none rounded-xl bg-background/80 font-mono text-xs leading-relaxed shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1 ring-border backdrop-blur-md prose-neutral dark:prose-invert"
		>
			{#if hideStreamingContent}
				<div
					class="flex items-center gap-1.5 px-4 py-3"
					role="status"
					aria-live="polite"
					aria-label="Assistant is responding"
				>
					<span class="typing-dot rounded-full bg-muted-foreground"></span>
					<span class="typing-dot rounded-full bg-muted-foreground"></span>
					<span class="typing-dot rounded-full bg-muted-foreground"></span>
				</div>
			{:else}
				{#if getMessageReasoning(message)}
					<details class="rounded-t-xl border-b border-border/70 bg-muted/30 px-3 pt-3 pb-2">
						<summary class="cursor-pointer text-[10px] font-bold tracking-wide uppercase">
							Reasoning
						</summary>
						<p class="mt-2 mb-0 font-mono text-[11px] whitespace-pre-wrap text-muted-foreground">
							{getMessageReasoning(message)}
						</p>
					</details>
				{/if}
				<div
					class="px-4 py-3 prose-headings:text-foreground prose-p:text-foreground prose-strong:text-foreground prose-li:text-foreground"
				>
					<SvelteMarkdown source={getMessageText(message)} />
				</div>
			{/if}
		</div>
		<div class="flex items-center gap-2 px-1">
			<span class="text-[9px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase">
				{formatMessageTimestamp(message.timestamp)}
			</span>
			{#if message.status !== 'streaming'}
				<button
					onclick={copyText}
					class="flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors hover:text-primary md:invisible md:group-hover:visible"
					title={copied ? 'Copied!' : 'Copy response'}
					aria-label={copied ? 'Copied!' : 'Copy response'}
				>
					{#if copied}
						<CheckIcon size={13} weight="bold" />
					{:else}
						<CopyIcon size={13} weight="bold" />
					{/if}
				</button>
				{#if onRetry}
					<button
						onclick={onRetry}
						class="flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors hover:text-primary md:invisible md:group-hover:visible"
						title="Retry response"
						aria-label="Retry response"
					>
						<ArrowCounterClockwiseIcon size={13} weight="bold" />
					</button>
				{/if}
				{#if onFork}
					<button
						onclick={onFork}
						class="flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors hover:text-primary md:invisible md:group-hover:visible"
						title="Fork thread from this response"
						aria-label="Fork thread from this response"
					>
						<GitForkIcon size={13} weight="bold" />
					</button>
				{/if}
			{/if}
		</div>
	</div>
</div>

<style>
	@keyframes typing-wave {
		0%,
		80%,
		100% {
			opacity: 0.35;
			transform: translateY(0);
		}
		40% {
			opacity: 1;
			transform: translateY(-2px);
		}
	}

	.typing-dot {
		display: inline-block;
		height: 0.35rem;
		width: 0.35rem;
		animation: typing-wave 1.2s ease-in-out infinite;
	}

	.typing-dot:nth-child(2) {
		animation-delay: 0.15s;
	}

	.typing-dot:nth-child(3) {
		animation-delay: 0.3s;
	}

	.assistant-prose :global(pre) {
		background-color: var(--muted);
		color: var(--foreground);
	}

	.assistant-prose :global(pre code) {
		background-color: transparent;
		color: inherit;
		padding: 0;
	}

	.assistant-prose :global(:not(pre) > code) {
		background-color: var(--muted);
		color: var(--foreground);
	}
</style>
