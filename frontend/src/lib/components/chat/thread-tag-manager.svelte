<script lang="ts">
	import { PlusIcon, TagIcon, XIcon } from 'phosphor-svelte';
	import type { Tag } from '$lib/types';

	interface Props {
		threadTags: Tag[];
		availableTags: Tag[];
		isLoading?: boolean;
		onAddTag: (tagId: string) => void;
		onRemoveTag: (tagId: string) => void;
		onCreateTag: (name: string) => void;
	}

	let {
		threadTags,
		availableTags,
		isLoading = false,
		onAddTag,
		onRemoveTag,
		onCreateTag
	}: Props = $props();

	let search = $state('');
	let open = $state(false);

	const threadTagIds = $derived(new Set(threadTags.map((t) => t.id)));

	const filtered = $derived(
		availableTags
			.filter((t) => !threadTagIds.has(t.id))
			.filter((t) => t.name.toLowerCase().includes(search.toLowerCase()))
	);

	const exactMatch = $derived(
		availableTags.some((t) => t.name.toLowerCase() === search.toLowerCase())
	);

	function selectTag(tag: Tag) {
		onAddTag(tag.id);
		search = '';
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			if (filtered.length > 0) {
				selectTag(filtered[0]);
			} else if (search.trim() && !exactMatch) {
				onCreateTag(search.trim());
				search = '';
			}
		} else if (event.key === 'Escape') {
			open = false;
			search = '';
		}
	}
</script>

<div class="flex flex-wrap items-center gap-1.5">
	{#each threadTags as tag (tag.id)}
		<span
			class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium text-white"
			style="background-color: {tag.color}"
		>
			{tag.name}
			<button
				type="button"
				class="ml-0.5 opacity-70 hover:opacity-100"
				onclick={() => onRemoveTag(tag.id)}
				disabled={isLoading}
				aria-label="Remove tag {tag.name}"
			>
				<XIcon size={10} />
			</button>
		</span>
	{/each}

	<div class="relative">
		<div class="flex items-center gap-1 rounded-md border border-border bg-background px-2 py-0.5">
			<TagIcon size={12} class="shrink-0 text-muted-foreground" />
			<input
				bind:value={search}
				type="text"
				placeholder="Add tag…"
				class="w-24 bg-transparent text-xs outline-none placeholder:text-muted-foreground/60"
				onfocus={() => (open = true)}
				onblur={() => setTimeout(() => (open = false), 150)}
				onkeydown={handleKeydown}
				disabled={isLoading}
			/>
		</div>

		{#if open && (filtered.length > 0 || (search.trim() && !exactMatch))}
			<div
				class="absolute top-full left-0 z-50 mt-1 min-w-40 rounded-md border border-border bg-popover p-1 shadow-md"
			>
				{#each filtered as tag (tag.id)}
					<button
						type="button"
						class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-xs hover:bg-accent"
						onmousedown={(e) => {
							e.preventDefault();
							selectTag(tag);
						}}
					>
						<span
							class="inline-block h-2.5 w-2.5 shrink-0 rounded-full"
							style="background-color: {tag.color}"
						></span>
						{tag.name}
					</button>
				{/each}
				{#if search.trim() && !exactMatch}
					<button
						type="button"
						class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-xs text-primary hover:bg-accent"
						onmousedown={(e) => {
							e.preventDefault();
							onCreateTag(search.trim());
							search = '';
						}}
					>
						<PlusIcon size={12} />
						Create "{search.trim()}"
					</button>
				{/if}
			</div>
		{/if}
	</div>
</div>
