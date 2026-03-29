<script lang="ts">
	import { RobotIcon } from 'phosphor-svelte';
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
	}

	let { message }: Props = $props();

	let hideStreamingContent = $derived(
		message.status === 'streaming' && !$showAssistantStreamingText
	);
</script>

<div
	id={message.id}
	class="flex w-full animate-in flex-row gap-5 transition-all duration-500 fade-in slide-in-from-bottom-2"
>
	<Avatar.Root title="Clanker" class="mt-1 h-9 w-9 shrink-0 shadow-sm ring-2 ring-background">
		<Avatar.Fallback class="border border-primary/20 bg-primary/10 text-primary">
			<RobotIcon size={18} />
		</Avatar.Fallback>
	</Avatar.Root>
	<div class="flex w-full max-w-[85%] flex-col items-start gap-2.5">
		<div
			class="prose prose-sm w-full max-w-none rounded-2xl bg-background/80 font-mono text-xs leading-relaxed shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1 ring-border backdrop-blur-md prose-neutral dark:prose-invert"
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
					<details class="rounded-t-2xl border-b border-border/70 bg-muted/30 px-3 pt-3 pb-2">
						<summary class="cursor-pointer text-[10px] font-bold tracking-wide uppercase">
							Reasoning
						</summary>
						<p class="mt-2 mb-0 font-mono text-[11px] whitespace-pre-wrap text-muted-foreground">
							{getMessageReasoning(message)}
						</p>
					</details>
				{/if}
				<div
					class="px-4 py-3 prose-headings:text-foreground prose-p:text-foreground prose-strong:text-foreground prose-code:text-foreground prose-li:text-foreground"
				>
					<SvelteMarkdown source={getMessageText(message)} />
				</div>
			{/if}
		</div>
		<span class="px-1 text-[9px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase">
			{formatMessageTimestamp(message.timestamp)}
		</span>
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
</style>
