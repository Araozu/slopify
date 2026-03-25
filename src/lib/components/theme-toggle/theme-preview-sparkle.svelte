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
		background: var(--sparkle-core);
	}

	.theme-preview__sparkle {
		color: #000;
	}

	[data-theme-preview='corporate-memo'] {
		--sparkle-core: var(--theme-corporate-memo-preview);
	}

	[data-theme-preview='wishful-aurosa'] {
		--sparkle-core: var(--theme-wishful-aurosa-preview);
	}

	[data-theme-preview='misty-dream'] {
		--sparkle-core: var(--theme-misty-dream-preview);
	}

	[data-theme-preview='twilight-echoes'] {
		--sparkle-core: var(--theme-twilight-echoes-preview);
	}

	[data-theme-preview='crimson-shade'] {
		--sparkle-core: var(--theme-crimson-shade-preview);
	}

	[data-theme-preview='spectral-mist'] {
		--sparkle-core: var(--theme-spectral-mist-preview);
	}

	[data-theme-preview='golden-plume'] {
		--sparkle-core: var(--theme-golden-plume-preview);
	}

	[data-theme-preview='forest-ripple'] {
		--sparkle-core: var(--theme-forest-ripple-preview);
	}

	[data-theme-preview='blazing-fields'] {
		--sparkle-core: var(--theme-blazing-fields-preview);
	}

	[data-theme-preview='enchanted-encounter'] {
		--sparkle-core: var(--theme-enchanted-encounter-preview);
	}
</style>
