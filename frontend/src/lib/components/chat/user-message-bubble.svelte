<script lang="ts">
	import { ChecksIcon, CheckIcon, UserIcon, TrashSimpleIcon } from 'phosphor-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import type { Message } from '$lib/types';
	import { formatMessageTimestamp, getMessageText } from './chat-message-utils.js';

	interface Props {
		message: Message;
		onDelete?: () => void;
	}

	let { message, onDelete }: Props = $props();
</script>

<div
	id={message.id}
	class="group flex w-full animate-in flex-row-reverse gap-5 transition-all duration-500 fade-in slide-in-from-bottom-2"
>
	<Avatar.Root title="Human" class="mt-1 h-9 w-9 shrink-0 shadow-sm ring-2 ring-background">
		<Avatar.Fallback class="border border-border bg-secondary text-secondary-foreground">
			<UserIcon size={18} />
		</Avatar.Fallback>
	</Avatar.Root>
	<div class="flex max-w-[85%] flex-col items-end gap-2.5">
		<div
			class="rounded-2xl bg-primary px-4 py-3 text-sm leading-relaxed text-primary-foreground shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1 ring-primary/20"
		>
			{getMessageText(message)}
		</div>
		<div class="flex items-center gap-2 px-1">
			{#if onDelete}
				<button
					onclick={onDelete}
					class="invisible flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors group-hover:visible hover:text-destructive"
					title="Delete message and response"
					aria-label="Delete message and response"
				>
					<TrashSimpleIcon size={13} weight="bold" />
				</button>
			{/if}
			<div
				class="flex items-center gap-1.5 text-[9px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase"
			>
				<span>{formatMessageTimestamp(message.timestamp)}</span>
				{#if message.deliveryStatus === 'delivered'}
					<ChecksIcon size={12} weight="bold" aria-label="Delivered" />
				{:else if message.deliveryStatus === 'sent'}
					<CheckIcon size={12} weight="bold" aria-label="Sent" />
				{/if}
			</div>
		</div>
	</div>
</div>
