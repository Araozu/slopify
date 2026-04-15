<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import {
		KeyIcon,
		PaperPlaneRightIcon,
		ChatCircleTextIcon,
		CubeIcon,
		PlusIcon,
		XIcon
	} from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Input } from '$lib/components/ui/input';
	import type { ProviderCredential, SystemPrompt, Thread, OpenRouterModel } from '$lib/types';

	interface Props {
		draft?: string;
		model?: string;
		credentials: ProviderCredential[];
		selectedCredential: ProviderCredential | null;
		systemPrompts: SystemPrompt[];
		selectedSystemPromptId: string | null;
		savedModels: OpenRouterModel[];
		canSaveModel: boolean;
		isSending: boolean;
		isBootstrapping: boolean;
		activeThread: Thread | null;
		onSelectCredential: (id: string) => void;
		onSelectSystemPrompt: (id: string | null) => void;
		onSaveModel: (modelId: string) => void;
		onSend: () => void;
		onComposerKeydown: (event: KeyboardEvent) => void;
	}

	let {
		draft = $bindable(''),
		model = $bindable(''),
		credentials,
		selectedCredential,
		systemPrompts,
		selectedSystemPromptId,
		savedModels,
		canSaveModel,
		isSending,
		isBootstrapping,
		activeThread,
		onSelectCredential,
		onSelectSystemPrompt,
		onSaveModel,
		onSend,
		onComposerKeydown
	}: Props = $props();

	const selectedSystemPrompt = $derived(
		systemPrompts.find((p) => p.id === selectedSystemPromptId) ?? null
	);

	const filteredModels = $derived(
		model.trim()
			? savedModels.filter((m) => m.modelId.toLowerCase().includes(model.toLowerCase()))
			: savedModels
	);

	const isModelAlreadySaved = $derived(
		savedModels.some((m) => m.modelId.toLowerCase() === model.trim().toLowerCase())
	);

	const PREVIEW_LENGTH = 40;

	const PROVIDER_LABELS: Record<string, string> = {
		openrouter: 'OR',
		'github-copilot': 'Copilot'
	};

	const openRouterCredentials = $derived(credentials.filter((c) => c.provider === 'openrouter'));
	const copilotCredentials = $derived(credentials.filter((c) => c.provider === 'github-copilot'));

	let modelDropdownOpen = $state(false);
	let modelSearchRef: HTMLInputElement | null = $state(null);

	function clearModel(e: MouseEvent) {
		e.stopPropagation();
		model = '';
		modelDropdownOpen = true;
		setTimeout(() => modelSearchRef?.focus(), 0);
	}
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
						weight={selectedCredential ? 'fill' : 'regular'}
						class={selectedCredential ? 'text-primary' : 'text-muted-foreground/40'}
					/>
					{#if selectedCredential}
						<span
							class="rounded bg-muted px-1 py-0.5 text-[9px] font-black tracking-widest text-muted-foreground/60 uppercase"
							>{PROVIDER_LABELS[selectedCredential.provider] ?? selectedCredential.provider}</span
						>
						<span class="text-foreground">{selectedCredential.name}</span>
					{:else}
						<span class="text-muted-foreground/40">No credentials saved</span>
					{/if}
				</div>
				<span class="text-[10px] font-black tracking-widest text-muted-foreground/30 uppercase"
					>key</span
				>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="start" class="w-64 rounded-lg shadow-xl">
				<DropdownMenu.Label
					class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
					>Select provider credential</DropdownMenu.Label
				>
				<DropdownMenu.Separator />
				{#if credentials.length === 0}
					<div class="px-2 py-3 text-center">
						<p class="text-[11px] text-muted-foreground/60">No credentials found</p>
						<Button
							variant="link"
							class="mt-1 h-auto p-0 text-[10px] font-bold tracking-widest uppercase"
							onclick={() => goto(resolve('/(main)/settings/providers'))}
						>
							Add one in settings
						</Button>
					</div>
				{:else}
					{#if openRouterCredentials.length > 0}
						<DropdownMenu.Label
							class="text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
							>OpenRouter</DropdownMenu.Label
						>
						{#each openRouterCredentials as cred (cred.id)}
							<DropdownMenu.Item
								class="flex items-center justify-between rounded-md py-2"
								onclick={() => onSelectCredential(cred.id)}
							>
								<div class="flex flex-col gap-0.5">
									<div class="flex items-center gap-1.5">
										<span
											class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
											>OR</span
										>
										<span class="text-xs font-bold">{cred.name}</span>
									</div>
									<span class="font-mono text-[9px] text-muted-foreground/50"
										>{cred.token.slice(0, 8)}••••</span
									>
								</div>
								{#if selectedCredential?.id === cred.id}
									<div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
								{/if}
							</DropdownMenu.Item>
						{/each}
					{/if}
					{#if copilotCredentials.length > 0}
						{#if openRouterCredentials.length > 0}
							<DropdownMenu.Separator />
						{/if}
						<DropdownMenu.Label
							class="text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
							>GitHub Copilot</DropdownMenu.Label
						>
						{#each copilotCredentials as cred (cred.id)}
							<DropdownMenu.Item
								class="flex items-center justify-between rounded-md py-2"
								onclick={() => onSelectCredential(cred.id)}
							>
								<div class="flex flex-col gap-0.5">
									<div class="flex items-center gap-1.5">
										<span
											class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
											>Copilot</span
										>
										<span class="text-xs font-bold">{cred.name}</span>
									</div>
									<span class="font-mono text-[9px] text-muted-foreground/50"
										>{cred.token.slice(0, 8)}••••</span
									>
								</div>
								{#if selectedCredential?.id === cred.id}
									<div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
								{/if}
							</DropdownMenu.Item>
						{/each}
					{/if}
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>

		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="flex h-8 min-w-0 flex-1 items-center justify-between gap-2 rounded-md border border-border/60 bg-background/70 px-3 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
				disabled={isSending || isBootstrapping || !activeThread}
			>
				<div class="flex items-center gap-2 truncate">
					<ChatCircleTextIcon
						size={12}
						weight={selectedSystemPrompt ? 'fill' : 'regular'}
						class={selectedSystemPrompt ? 'text-primary' : 'text-muted-foreground/40'}
					/>
					<span class={selectedSystemPrompt ? 'text-foreground' : 'text-muted-foreground/40'}>
						{selectedSystemPrompt ? selectedSystemPrompt.name : 'No system prompt'}
					</span>
				</div>
				<span class="text-[10px] font-black tracking-widest text-muted-foreground/30 uppercase"
					>system</span
				>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="start" class="w-64 rounded-lg shadow-xl">
				<DropdownMenu.Label
					class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
					>Select system prompt</DropdownMenu.Label
				>
				<DropdownMenu.Separator />
				<DropdownMenu.Item
					class="flex items-center justify-between rounded-md py-2"
					onclick={() => onSelectSystemPrompt(null)}
				>
					<span class="text-xs text-muted-foreground/60">None</span>
					{#if !selectedSystemPrompt}
						<div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
					{/if}
				</DropdownMenu.Item>
				{#if systemPrompts.length === 0}
					<div class="px-2 py-3 text-center">
						<p class="text-[11px] text-muted-foreground/60">No presets saved</p>
						<Button
							variant="link"
							class="mt-1 h-auto p-0 text-[10px] font-bold tracking-widest uppercase"
							onclick={() => goto(resolve('/(main)/settings/system-prompts'))}
						>
							Create one in settings
						</Button>
					</div>
				{:else}
					{#each systemPrompts as prompt (prompt.id)}
						<DropdownMenu.Item
							class="flex items-center justify-between rounded-md py-2"
							onclick={() => onSelectSystemPrompt(prompt.id)}
						>
							<div class="flex flex-col gap-0.5">
								<span class="text-xs font-bold">{prompt.name}</span>
								<span class="truncate text-[9px] text-muted-foreground/50"
									>{prompt.content.slice(0, PREVIEW_LENGTH)}{prompt.content.length > PREVIEW_LENGTH
										? '...'
										: ''}</span
								>
							</div>
							{#if selectedSystemPromptId === prompt.id}
								<div class="h-1.5 w-1.5 shrink-0 rounded-full bg-primary"></div>
							{/if}
						</DropdownMenu.Item>
					{/each}
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>

		<div class="relative min-w-0 flex-1">
			<DropdownMenu.Root bind:open={modelDropdownOpen}>
				<DropdownMenu.Trigger
					class="flex h-8 w-full items-center gap-2 rounded-md border border-border/60 bg-background/70 pr-7 pl-3 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
					disabled={isSending || isBootstrapping || !activeThread}
				>
					<div class="flex min-w-0 flex-1 items-center gap-2 truncate">
						<CubeIcon
							size={12}
							weight={model.trim() ? 'fill' : 'regular'}
							class={model.trim() ? 'text-primary' : 'text-muted-foreground/40'}
						/>
						<span class={model.trim() ? 'truncate text-foreground' : 'text-muted-foreground/40'}>
							{model.trim() || 'Select model'}
						</span>
					</div>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="start" class="w-72 rounded-lg shadow-xl">
					<DropdownMenu.Label
						class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
						>Select or type model</DropdownMenu.Label
					>
					<div class="p-2">
						<Input
							bind:ref={modelSearchRef}
							bind:value={model}
							placeholder="Search or type a model..."
							class="h-8 border-border/40 bg-muted/30 text-xs"
							autofocus
						/>
					</div>
					<DropdownMenu.Separator />
					<div class="max-h-60 overflow-y-auto p-1">
						{#if filteredModels.length === 0 && !model.trim()}
							<div class="px-2 py-3 text-center">
								<p class="text-[11px] text-muted-foreground/60">No models saved</p>
								<Button
									variant="link"
									class="mt-1 h-auto p-0 text-[10px] font-bold tracking-widest uppercase"
									onclick={() => goto(resolve('/(main)/settings/providers'))}
								>
									Manage in settings
								</Button>
							</div>
						{:else}
							{#each filteredModels as m (m.id)}
								<DropdownMenu.Item
									class="flex items-center justify-between rounded-lg py-2"
									onclick={() => (model = m.modelId)}
								>
									<span class="truncate font-mono text-xs">{m.modelId}</span>
									{#if model === m.modelId}
										<div class="h-1.5 w-1.5 shrink-0 rounded-full bg-primary"></div>
									{/if}
								</DropdownMenu.Item>
							{/each}

							{#if model.trim() && !isModelAlreadySaved && canSaveModel}
								<DropdownMenu.Item
									class="mt-1 flex items-center gap-2 rounded-lg py-2 text-primary"
									onclick={() => onSaveModel(model.trim())}
								>
									<PlusIcon size={14} weight="bold" />
									<span class="text-[11px] font-bold tracking-tight uppercase"
										>Save & select "{model}"</span
									>
								</DropdownMenu.Item>
							{/if}
						{/if}
					</div>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			{#if model.trim() && !isSending && !isBootstrapping && activeThread}
				<button
					class="hover:text-destructive-foreground absolute top-1/2 right-2 flex h-4 w-4 -translate-y-1/2 items-center justify-center rounded-full bg-muted-foreground/20 text-muted-foreground transition-all hover:bg-destructive/80"
					onclick={clearModel}
					tabindex="-1"
					aria-label="Clear model"
				>
					<XIcon size={8} weight="bold" />
				</button>
			{/if}
		</div>
	</div>
	<div
		class="mx-auto flex max-w-3xl items-center gap-3 rounded-xl bg-muted/40 p-2.5 shadow-inner ring-1 ring-border/50 transition-all focus-within:bg-background/60 focus-within:ring-primary/30"
	>
		<Textarea
			bind:value={draft}
			placeholder="Message Slopify..."
			class="dark:bg-initial bg-initial field-sizing-content max-h-[calc(7*1.5rem+1rem)] min-h-[2.25rem] resize-none border-0 px-3 py-2 text-sm leading-6 placeholder:text-muted-foreground/40 focus-visible:ring-0 focus-visible:ring-offset-0"
			disabled={isSending || isBootstrapping || !activeThread}
			onkeydown={onComposerKeydown}
			rows={1}
		/>
		<Button
			size="icon-sm"
			variant="default"
			class="h-9 w-9 rounded-lg shadow-lg shadow-primary/20 transition-transform active:scale-95"
			disabled={isSending ||
				isBootstrapping ||
				!activeThread ||
				!draft.trim() ||
				!selectedCredential ||
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
