<script lang="ts">
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { StreamChatEvent } from '$lib/thread-client';
	import { cn } from '$lib/utils';
	import { formatMessageTimestamp } from './chat-message-utils.js';

	interface StreamLogEvent {
		id: string;
		timestamp: string;
		type: StreamChatEvent['type'];
		detail: string;
	}

	interface Props {
		collapsed: boolean;
		streamEvents: StreamLogEvent[];
	}

	let { collapsed, streamEvents }: Props = $props();
</script>

<aside
	id="stream-sidebar"
	class={cn(
		'min-h-0 w-60 flex-col border-l bg-muted/20 backdrop-blur-md',
		collapsed ? 'hidden' : 'hidden lg:flex'
	)}
	aria-hidden={collapsed}
>
	<div class="p-4">
		<h2 class="text-[10px] font-black tracking-widest text-foreground/40 uppercase">Stream Log</h2>
	</div>
	<ScrollArea.Root class="flex-1">
		<div class="space-y-4 p-4">
			{#if streamEvents.length === 0}
				<p class="text-[11px] text-muted-foreground/60">No stream events yet.</p>
			{:else}
				{#each streamEvents as event (`stream-${event.id}`)}
					<div class="group flex w-full items-start gap-2.5 text-left transition-all">
						<div
							class={cn(
								'mt-1.5 flex h-1.5 w-1.5 shrink-0 rounded-full transition-all group-hover:scale-125',
								event.type === 'message_failed' ? 'bg-destructive' : 'bg-primary'
							)}
						></div>
						<div class="flex flex-col gap-1">
							<span
								class="text-[10px] font-black tracking-tighter text-muted-foreground/50 uppercase transition-colors group-hover:text-primary/60"
							>
								{event.type}
							</span>
							<p
								class="line-clamp-2 text-[11px] leading-snug text-foreground/60 transition-colors group-hover:text-foreground"
							>
								{event.detail}
							</p>
							<span
								class="text-[9px] font-bold tracking-[0.12em] text-muted-foreground/40 uppercase"
							>
								{formatMessageTimestamp(event.timestamp)}
							</span>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</ScrollArea.Root>
</aside>
