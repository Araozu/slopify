<script lang="ts">
	import {
		ChecksIcon,
		CheckIcon,
		UserIcon,
		TrashSimpleIcon,
		PencilSimpleIcon,
		XIcon
	} from 'phosphor-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import type { Message } from '$lib/types';
	import { formatMessageTimestamp, getMessageText } from './chat-message-utils.js';
	import { messageFontSize } from '$lib/stores/ui-preferences';

	interface Props {
		message: Message;
		onDelete?: () => void;
		onEditResend?: (newText: string) => void;
	}

	let { message, onDelete, onEditResend }: Props = $props();

	let editing = $state(false);
	let editDraft = $state('');

	const fontSizeClass = $derived(
		$messageFontSize === 'lg' ? 'text-base' : $messageFontSize === 'md' ? 'text-sm' : 'text-xs'
	);

	function beginEdit() {
		editDraft = getMessageText(message);
		editing = true;
	}

	function cancelEdit() {
		editing = false;
		editDraft = '';
	}

	function commitEdit() {
		const trimmed = editDraft.trim();
		if (!trimmed) return;
		onEditResend?.(trimmed);
		editing = false;
		editDraft = '';
	}

	function onEditKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && event.ctrlKey) {
			event.preventDefault();
			commitEdit();
		} else if (event.key === 'Escape') {
			event.preventDefault();
			cancelEdit();
		}
	}
</script>

<div
	id={message.id}
	class="group flex w-full animate-in justify-end transition-all duration-500 fade-in slide-in-from-bottom-2"
>
	<div class="relative flex max-w-[85%] flex-col items-end gap-2.5">
		<Avatar.Root
			title="Human"
			class="absolute -top-2 -right-2 z-10 h-5 w-5 shadow-sm ring-2 ring-background"
		>
			<Avatar.Fallback class="border border-border bg-secondary text-secondary-foreground">
				<UserIcon size={12} />
			</Avatar.Fallback>
		</Avatar.Root>
		{#if editing}
			<div class="flex w-full max-w-[85%] flex-col gap-2">
				<Textarea
					bind:value={editDraft}
					class="field-sizing-content min-h-[4rem] resize-none rounded-xl text-sm leading-relaxed"
					onkeydown={onEditKeydown}
					rows={3}
				/>
				<div class="flex justify-end gap-1.5">
					<Button variant="ghost" size="xs" onclick={cancelEdit}>
						<XIcon size={12} />
						Cancel
					</Button>
					<Button size="xs" onclick={commitEdit} disabled={!editDraft.trim()}>Send</Button>
				</div>
			</div>
		{:else}
			<div
				class="rounded-xl bg-primary px-4 py-3 leading-relaxed text-primary-foreground shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1 ring-primary/20 {fontSizeClass}"
			>
				{getMessageText(message)}
			</div>
		{/if}
		<div class="flex items-center gap-2 px-1">
			{#if !editing}
				{#if onEditResend}
					<button
						onclick={beginEdit}
						class="flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors hover:text-primary md:invisible md:group-hover:visible"
						title="Edit and resend"
						aria-label="Edit and resend"
					>
						<PencilSimpleIcon size={13} weight="bold" />
					</button>
				{/if}
				{#if onDelete}
					<button
						onclick={onDelete}
						class="flex h-5 w-5 items-center justify-center rounded text-muted-foreground/40 transition-colors hover:text-destructive md:invisible md:group-hover:visible"
						title="Delete message and response"
						aria-label="Delete message and response"
					>
						<TrashSimpleIcon size={13} weight="bold" />
					</button>
				{/if}
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
