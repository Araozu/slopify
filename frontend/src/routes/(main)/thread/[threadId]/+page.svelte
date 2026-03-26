<script lang="ts">
	import { page } from '$app/state';
	import ChatPage from '$lib/components/chat/chat-page.svelte';

	const threadId = $derived(page.params.threadId ?? '');
</script>

<ChatPage {threadId} />
