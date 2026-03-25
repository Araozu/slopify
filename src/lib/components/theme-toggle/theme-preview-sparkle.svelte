<script lang="ts">
	import { cn } from '$lib/utils.js';
	import type { ThemeId } from '$lib/stores/theme';

	let {
		themeId,
		class: className
	}: {
		themeId: ThemeId;
		class?: string;
	} = $props();
</script>

<span
	class={cn('theme-preview inline-flex size-4 items-center justify-center rounded-full', className)}
	data-theme-preview={themeId}
	aria-hidden="true"
>
	<svg viewBox="0 0 24 24" class="theme-preview__sparkle size-3" fill="none">
		<path
			d="M12 2.75L13.85 8.15L19.25 10L13.85 11.85L12 17.25L10.15 11.85L4.75 10L10.15 8.15L12 2.75Z"
			fill="currentColor"
		/>
		<path
			d="M18.25 14.75L18.95 16.8L21 17.5L18.95 18.2L18.25 20.25L17.55 18.2L15.5 17.5L17.55 16.8L18.25 14.75Z"
			fill="currentColor"
			opacity="0.72"
		/>
	</svg>
</span>

<style>
	.theme-preview {
		background: color-mix(in oklab, var(--sparkle-core) 18%, white);
		border: 1px solid color-mix(in oklab, var(--sparkle-core) 28%, white);
	}

	.theme-preview__sparkle {
		color: var(--sparkle-core);
		filter: drop-shadow(0 0 4px var(--sparkle-shadow));
	}

	[data-theme-preview='corporate-memo'] {
		--sparkle-core: var(--theme-corporate-memo-preview);
		--sparkle-shadow: var(--theme-corporate-memo-shadow);
	}

	[data-theme-preview='wishful-aurosa'] {
		--sparkle-core: var(--theme-wishful-aurosa-preview);
		--sparkle-shadow: var(--theme-wishful-aurosa-shadow);
	}

	[data-theme-preview='misty-dream'] {
		--sparkle-core: var(--theme-misty-dream-preview);
		--sparkle-shadow: var(--theme-misty-dream-shadow);
	}

	[data-theme-preview='twilight-echoes'] {
		--sparkle-core: var(--theme-twilight-echoes-preview);
		--sparkle-shadow: var(--theme-twilight-echoes-shadow);
	}

	[data-theme-preview='crimson-shade'] {
		--sparkle-core: var(--theme-crimson-shade-preview);
		--sparkle-shadow: var(--theme-crimson-shade-shadow);
	}
</style>
