<script lang="ts">
	import type { Message } from '$lib/types';
	import AssistantMessageBubble from './assistant-message-bubble.svelte';
	import UserMessageBubble from './user-message-bubble.svelte';

	interface Props {
		message: Message;
		onDeletePair?: (messageId: string) => void;
		onFork?: (messageId: string) => void;
	}

	let { message, onDeletePair, onFork }: Props = $props();
</script>

{#if message.role === 'user'}
	<UserMessageBubble {message} onDelete={() => onDeletePair?.(message.id)} />
{:else}
	<AssistantMessageBubble {message} onFork={() => onFork?.(message.id)} />
{/if}
