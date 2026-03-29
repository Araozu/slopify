<script lang="ts">
	import { RobotIcon } from 'phosphor-svelte';
	import SvelteMarkdown from 'svelte-markdown';
	import * as Avatar from '$lib/components/ui/avatar';
	import type { Message } from '$lib/types';
	import {
		formatMessageTimestamp,
		getMessageReasoning,
		getMessageText
	} from './chat-message-utils.js';

	interface Props {
		message: Message;
	}

	let { message }: Props = $props();
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
	<div class="flex max-w-[85%] flex-col items-start gap-2.5 w-full">
		<div
			class="prose prose-sm max-w-none rounded-2xl bg-background/80 font-mono text-xs leading-relaxed shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1 ring-border backdrop-blur-md prose-neutral dark:prose-invert w-full"
		>
			{#if getMessageReasoning(message)}
				<details class="rounded-t-2xl border-b border-border/70 bg-muted/30 px-3 pb-2 pt-3">
					<summary class="cursor-pointer text-[10px] font-bold tracking-wide uppercase">
						Reasoning
					</summary>
					<p class="mt-2 mb-0 font-mono text-[11px] whitespace-pre-wrap text-muted-foreground">
						{getMessageReasoning(message)}
					</p>
				</details>
			{/if}
			<div
				class="prose-headings:text-foreground prose-p:text-foreground prose-strong:text-foreground prose-code:text-foreground prose-li:text-foreground px-4 py-3"
			>
				<SvelteMarkdown source={getMessageText(message)} />
			</div>
		</div>
		<span class="px-1 text-[9px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase">
			{formatMessageTimestamp(message.timestamp)}
		</span>
	</div>
</div>
