<script lang="ts">
	import { QueryClientProvider } from '@tanstack/svelte-query';
	import './layout.css';
	import { ModeWatcher } from 'mode-watcher';
	import favicon from '$lib/assets/favicon.svg';
	import { DEFAULT_THEME, THEME_STORAGE_KEY } from '$lib/stores/theme';
	import { queryClient } from '$lib/query-client';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Slopify</title>
</svelte:head>
<QueryClientProvider client={queryClient}>
	<ModeWatcher defaultTheme={DEFAULT_THEME} themeStorageKey={THEME_STORAGE_KEY} />
	{@render children()}
</QueryClientProvider>
