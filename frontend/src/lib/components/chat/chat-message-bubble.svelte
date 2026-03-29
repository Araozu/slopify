<script lang="ts">
	import type { Message } from '$lib/types';
	import AssistantMessageBubble from './assistant-message-bubble.svelte';
	import UserMessageBubble from './user-message-bubble.svelte';

	interface Props {
		message: Message;
	}

	let { message }: Props = $props();
</script>

{#if message.role === 'user'}
	<UserMessageBubble {message} />
{:else}
	<AssistantMessageBubble {message} />
{/if}
