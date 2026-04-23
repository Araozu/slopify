<script lang="ts">
	import { PaletteIcon, CheckIcon } from 'phosphor-svelte';
	import { theme, themes, type ThemeId } from '$lib/stores/theme';
	import ThemePreviewSparkle from '$lib/components/theme-toggle/theme-preview-sparkle.svelte';
	import * as Button from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	function handleSelect(id: ThemeId, e: Event) {
		e.preventDefault();
		theme.setTheme(id);
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button.Root
				variant="ghost"
				size="icon"
				class="h-6 w-6 text-muted-foreground hover:text-foreground"
				{...props}
			>
				<PaletteIcon size={14} />
				<span class="sr-only">Select color scheme</span>
			</Button.Root>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-48">
		<DropdownMenu.Label>Color Scheme</DropdownMenu.Label>
		<DropdownMenu.Separator />
		{#each themes as t (t.id)}
			<DropdownMenu.Item
				onSelect={(e) => handleSelect(t.id, e)}
				class="flex items-center justify-between gap-3"
			>
				<span class="flex items-center gap-2">
					<ThemePreviewSparkle themeId={t.id} />
					<span>{t.name}</span>
				</span>
				{#if $theme === t.id}
					<CheckIcon class="h-4 w-4" />
				{/if}
			</DropdownMenu.Item>
		{/each}
	</DropdownMenu.Content>
</DropdownMenu.Root>
