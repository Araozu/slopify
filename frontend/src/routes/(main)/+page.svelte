<script lang="ts">
	import { useQueryClient } from '@tanstack/svelte-query';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { ensureFirstThread } from '$lib/queries/thread-query';
	import { onMount } from 'svelte';

	const queryClient = useQueryClient();

	let isLoading = $state(true);
	let errorMessage = $state('');

	onMount(() => {
		void redirectToThread();
	});

	async function redirectToThread() {
		try {
			const targetThread = await ensureFirstThread(queryClient);

			await goto(resolve(`/thread/${targetThread.id}`), { replaceState: true });
		} catch (error) {
			errorMessage = error instanceof Error ? error.message : 'Failed to open a thread.';
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="flex h-[calc(100vh-2rem)] items-center justify-center px-6">
	{#if errorMessage}
		<p
			class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
		>
			{errorMessage}
		</p>
	{:else if isLoading}
		<p class="text-sm text-muted-foreground">Opening your latest thread...</p>
	{/if}
</div>
