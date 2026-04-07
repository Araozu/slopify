<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { KeyIcon, PaperPlaneRightIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import type { OpenRouterApiKey, Thread } from '$lib/types';

	interface Props {
		draft?: string;
		model?: string;
		keys: OpenRouterApiKey[];
		selectedKey: OpenRouterApiKey | null;
		isSending: boolean;
		isBootstrapping: boolean;
		activeThread: Thread | null;
		onSelectKey: (id: string) => void;
		onSend: () => void;
		onComposerKeydown: (event: KeyboardEvent) => void;
	}

	let {
		draft = $bindable(''),
		model = $bindable(''),
		keys,
		selectedKey,
		isSending,
		isBootstrapping,
		activeThread,
		onSelectKey,
		onSend,
		onComposerKeydown
	}: Props = $props();
</script>

<footer class="p-4 md:p-6">
	<div class="mx-auto mb-2 flex max-w-3xl flex-wrap items-center gap-2 px-2">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="flex h-8 min-w-0 flex-1 items-center justify-between gap-2 rounded-md border border-border/60 bg-background/70 px-3 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
				disabled={isSending || isBootstrapping || !activeThread}
			>
				<div class="flex items-center gap-2 truncate">
					<KeyIcon
						size={12}
						weight={selectedKey ? 'fill' : 'regular'}
						class={selectedKey ? 'text-primary' : 'text-muted-foreground/40'}
					/>
					<span class={selectedKey ? 'text-foreground' : 'text-muted-foreground/40'}>
						{selectedKey ? selectedKey.name : 'No keys saved'}
					</span>
				</div>
				<span class="text-[10px] font-black tracking-widest text-muted-foreground/30 uppercase"
					>key</span
				>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="start" class="w-56 rounded-xl shadow-xl">
				<DropdownMenu.Label
					class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
					>Select provider key</DropdownMenu.Label
				>
				<DropdownMenu.Separator />
				{#if keys.length === 0}
					<div class="px-2 py-3 text-center">
						<p class="text-[11px] text-muted-foreground/60">No keys found</p>
						<Button
							variant="link"
							class="mt-1 h-auto p-0 text-[10px] font-bold tracking-widest uppercase"
							onclick={() => goto(resolve('/(main)/settings/keys'))}
						>
							Add one in settings
						</Button>
					</div>
				{:else}
					{#each keys as key (key.id)}
						<DropdownMenu.Item
							class="flex items-center justify-between rounded-lg py-2"
							onclick={() => onSelectKey(key.id)}
						>
							<div class="flex flex-col gap-0.5">
								<span class="text-xs font-bold">{key.name}</span>
								<span class="font-mono text-[9px] text-muted-foreground/50"
									>{key.apiKey.slice(0, 8)}••••</span
								>
							</div>
							{#if selectedKey?.id === key.id}
								<div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
							{/if}
						</DropdownMenu.Item>
					{/each}
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>

		<Input
			bind:value={model}
			placeholder="openai/gpt-4o-mini"
			class="h-8 max-w-56 border-border/60 bg-background/70 text-xs"
			disabled={isSending || isBootstrapping || !activeThread}
		/>
		<span class="text-[10px] font-black tracking-widest text-muted-foreground/30 uppercase"
			>model</span
		>
	</div>
	<div
		class="mx-auto flex max-w-3xl items-center gap-3 rounded-[20px] bg-muted/40 p-2.5 shadow-inner ring-1 ring-border/50 transition-all focus-within:bg-background/60 focus-within:ring-primary/30"
	>
		<Input
			bind:value={draft}
			placeholder="Message Slopify..."
			class="h-9 border-0 bg-transparent px-3 text-sm placeholder:text-muted-foreground/40 focus-visible:ring-0 focus-visible:ring-offset-0"
			disabled={isSending || isBootstrapping || !activeThread}
			onkeydown={onComposerKeydown}
		/>
		<Button
			size="icon-sm"
			variant="default"
			class="h-9 w-9 rounded-[14px] shadow-lg shadow-primary/20 transition-transform active:scale-95"
			disabled={isSending ||
				isBootstrapping ||
				!activeThread ||
				!draft.trim() ||
				!selectedKey ||
				!model.trim()}
			onclick={onSend}
		>
			<PaperPlaneRightIcon size={18} weight="bold" />
		</Button>
	</div>
	<p
		class="mt-3 text-center text-[10px] font-medium tracking-widest text-muted-foreground/30 uppercase"
	>
		You know by now the clanker hallucinates... like a lot. Double check.
	</p>
</footer>
