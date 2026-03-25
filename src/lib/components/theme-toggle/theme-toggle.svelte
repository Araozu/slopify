<script lang="ts">
	import { PaletteIcon, CheckIcon } from 'phosphor-svelte';
	import { theme, themes, type ThemeId } from '$lib/stores/theme';
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
			<Button.Root variant="outline" size="icon" {...props}>
				<PaletteIcon class="h-[1.2rem] w-[1.2rem]" />
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
				class="flex items-center justify-between"
			>
				<span>{t.name}</span>
				{#if $theme === t.id}
					<CheckIcon class="h-4 w-4" />
				{/if}
			</DropdownMenu.Item>
		{/each}
	</DropdownMenu.Content>
</DropdownMenu.Root>
