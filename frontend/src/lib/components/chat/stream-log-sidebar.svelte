<script lang="ts">
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { cn } from '$lib/utils';
	import type { Message } from '$lib/types';
	import { getMessageText } from './chat-message-utils.js';

	interface Props {
		collapsed: boolean;
		messages: Message[];
		viewportRef: HTMLElement | null;
	}

	let { collapsed, messages, viewportRef }: Props = $props();

	const userMessages = $derived(messages.filter((m) => m.role === 'user'));

	function scrollToMessage(messageId: string) {
		if (!viewportRef) return;
		const el = viewportRef.querySelector(`#msg-${messageId}`);
		if (!el) return;
		el.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}
</script>

<aside
	id="stream-sidebar"
	class={cn(
		'min-h-0 w-56 flex-col border-l bg-muted/20 backdrop-blur-md',
		collapsed ? 'hidden' : 'hidden lg:flex'
	)}
	aria-hidden={collapsed}
>
	<div class="px-3 pt-3 pb-1">
		<h2 class="text-[10px] font-black text-foreground/40 uppercase">Prompts</h2>
	</div>

	{#if userMessages.length === 0}
		<div class="flex flex-1 items-center justify-center p-4 text-center">
			<p class="text-[11px] leading-relaxed text-muted-foreground/50">No messages yet</p>
		</div>
	{:else}
		<ScrollArea.Root class="min-h-0 flex-1">
			<div class="space-y-0.5 p-2">
				{#each userMessages as message (message.id)}
					<button
						type="button"
						onclick={() => scrollToMessage(message.id)}
						class={cn(
							'w-full rounded-md px-2.5 py-2 text-left text-xs leading-snug text-foreground/70',
							'line-clamp-2 transition-colors hover:bg-muted/60 hover:text-foreground',
							'focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none'
						)}
						title={getMessageText(message)}
					>
						{getMessageText(message)}
					</button>
				{/each}
			</div>
		</ScrollArea.Root>
	{/if}
</aside>
