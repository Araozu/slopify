<script lang="ts">
	import { SlidersIcon, TextAaIcon, ArrowsHorizontalIcon } from 'phosphor-svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import {
		messageFontSize,
		contentWidth,
		type MessageFontSize,
		type ContentWidth
	} from '$lib/stores/ui-preferences';

	const FONT_SIZE_OPTIONS: { value: MessageFontSize; label: string }[] = [
		{ value: 'sm', label: 'Sm' },
		{ value: 'md', label: 'Md' },
		{ value: 'lg', label: 'Lg' }
	];

	const CONTENT_WIDTH_OPTIONS: { value: ContentWidth; label: string }[] = [
		{ value: 'default', label: 'Default' },
		{ value: 'wide', label: 'Wide' },
		{ value: 'full', label: 'Full' }
	];
</script>

<Popover.Root>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				variant="ghost"
				size="icon"
				class="h-6 w-6 text-muted-foreground hover:text-foreground"
				title="UI settings"
				aria-label="UI settings"
				{...props}
			>
				<SlidersIcon size={14} weight="bold" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content align="end" class="w-72 p-0">
		<Popover.Header class="border-b px-3 py-2">
			<Popover.Title class="text-xs font-bold tracking-wide">UI Settings</Popover.Title>
		</Popover.Header>
		<div class="space-y-4 px-3 py-3">
			<!-- Font size -->
			<div class="flex items-center justify-between gap-3">
				<div class="flex items-center gap-2">
					<TextAaIcon size={14} class="shrink-0 text-muted-foreground" />
					<span class="text-xs font-medium">Message size</span>
				</div>
				<div class="flex items-center gap-0.5 rounded-md border border-border/60 bg-muted/30 p-0.5">
					{#each FONT_SIZE_OPTIONS as option (option.value)}
						<button
							class="rounded px-2.5 py-1 text-xs font-semibold transition-all
								{$messageFontSize === option.value
								? 'bg-background text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'}"
							onclick={() => messageFontSize.set(option.value)}
							aria-pressed={$messageFontSize === option.value}
						>
							{option.label}
						</button>
					{/each}
				</div>
			</div>

			<!-- Content width -->
			<div class="flex items-center justify-between gap-3">
				<div class="flex items-center gap-2">
					<ArrowsHorizontalIcon size={14} class="shrink-0 text-muted-foreground" />
					<span class="text-xs font-medium">Content width</span>
				</div>
				<div class="flex items-center gap-0.5 rounded-md border border-border/60 bg-muted/30 p-0.5">
					{#each CONTENT_WIDTH_OPTIONS as option (option.value)}
						<button
							class="rounded px-2.5 py-1 text-xs font-semibold transition-all
								{$contentWidth === option.value
								? 'bg-background text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'}"
							onclick={() => contentWidth.set(option.value)}
							aria-pressed={$contentWidth === option.value}
						>
							{option.label}
						</button>
					{/each}
				</div>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
