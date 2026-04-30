<script lang="ts">
	import {
		CaretDoubleLeftIcon,
		CaretDoubleRightIcon,
		ChatCircleIcon,
		ChatCircleTextIcon,
		CheckIcon,
		CubeIcon,
		KeyIcon,
		PencilSimpleIcon,
		PlusIcon,
		TrashIcon,
		XIcon
	} from 'phosphor-svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import type { ProviderCredential, SystemPrompt, Tag, Thread, OpenRouterModel } from '$lib/types';
	import ThreadTagManager from './thread-tag-manager.svelte';

	interface Props {
		threadTitle: string;
		activeThread: Thread | null;
		threadId: string;
		sidebarCollapsed: boolean;
		isDeletingThread: boolean;
		isRenamingThread: boolean;
		isSending: boolean;
		isBootstrapping: boolean;
		availableTags: Tag[];
		isTagLoading?: boolean;
		credentials: ProviderCredential[];
		selectedCredential: ProviderCredential | null;
		systemPrompts: SystemPrompt[];
		selectedSystemPromptId: string | null;
		savedModels: OpenRouterModel[];
		canSaveModel: boolean;
		model?: string;
		onRenameThread: (title: string) => Promise<void>;
		onDeleteThread: (id: string) => void;
		onToggleSidebar: () => void;
		onAddTag: (tagId: string) => void;
		onRemoveTag: (tagId: string) => void;
		onCreateTag: (name: string) => void;
		onSelectCredential: (id: string) => void;
		onSelectSystemPrompt: (id: string | null) => void;
		onSaveModel: (modelId: string) => void;
	}

	let {
		threadTitle,
		activeThread,
		threadId,
		sidebarCollapsed,
		isDeletingThread,
		isRenamingThread,
		isSending,
		isBootstrapping,
		availableTags,
		isTagLoading = false,
		credentials,
		selectedCredential,
		systemPrompts,
		selectedSystemPromptId,
		savedModels,
		canSaveModel,
		model = $bindable(''),
		onRenameThread,
		onDeleteThread,
		onToggleSidebar,
		onAddTag,
		onRemoveTag,
		onCreateTag,
		onSelectCredential,
		onSelectSystemPrompt,
		onSaveModel
	}: Props = $props();

	let editingTitle = $state(false);
	let titleDraft = $state('');
	let titleInputRef: HTMLInputElement | null = $state(null);

	$effect(() => {
		void threadId;
		editingTitle = false;
	});

	async function beginRename() {
		titleDraft = threadTitle;
		editingTitle = true;
		await tick();
		titleInputRef?.focus();
		titleInputRef?.select();
	}

	async function commitRename() {
		const next = titleDraft.trim();
		if (next === threadTitle) {
			editingTitle = false;
			return;
		}
		try {
			await onRenameThread(next);
			editingTitle = false;
		} catch {
			/* parent surfaces error; stay in edit mode */
		}
	}

	function cancelRename() {
		editingTitle = false;
		titleDraft = threadTitle;
	}

	function onTitleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			void commitRename();
		} else if (event.key === 'Escape') {
			event.preventDefault();
			cancelRename();
		}
	}

	let headerBusy = $derived(isDeletingThread || isRenamingThread || isSending);
	let threadTags = $derived(activeThread?.tags ?? []);

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
		'github-copilot': 'Copilot',
		openai: 'OpenAI'
	};

	const openRouterCredentials = $derived(credentials.filter((c) => c.provider === 'openrouter'));
	const copilotCredentials = $derived(credentials.filter((c) => c.provider === 'github-copilot'));
	const openAiCredentials = $derived(credentials.filter((c) => c.provider === 'openai'));

	let modelDropdownOpen = $state(false);
	let modelSearchRef: HTMLInputElement | null = $state(null);

	function clearModel(e: MouseEvent) {
		e.stopPropagation();
		model = '';
		modelDropdownOpen = true;
		setTimeout(() => modelSearchRef?.focus(), 0);
	}
</script>

<header class="flex flex-col border-b bg-background/20 px-4 py-1 backdrop-blur-xl">
	<div class="flex items-center">
		<div class="flex min-w-0 flex-1 items-center gap-2">
			<div
				class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
			>
				<ChatCircleIcon size={14} weight="fill" />
			</div>
			{#if editingTitle && activeThread}
				<div class="flex min-w-0 flex-1 items-center gap-1">
					<Input
						bind:ref={titleInputRef}
						bind:value={titleDraft}
						class="h-8 min-w-0 flex-1 text-sm font-bold"
						disabled={isRenamingThread}
						aria-label="Thread title"
						onkeydown={onTitleKeydown}
					/>
					<Button
						type="button"
						variant="ghost"
						size="icon-xs"
						class="shrink-0 text-muted-foreground hover:text-foreground"
						disabled={isRenamingThread}
						title="Save title"
						onmousedown={(event) => event.preventDefault()}
						onclick={() => void commitRename()}
					>
						<CheckIcon size={16} />
					</Button>
					<Button
						type="button"
						variant="ghost"
						size="icon-xs"
						class="shrink-0 text-muted-foreground hover:text-foreground"
						disabled={isRenamingThread}
						title="Cancel"
						onmousedown={(event) => event.preventDefault()}
						onclick={cancelRename}
					>
						<XIcon size={16} />
					</Button>
				</div>
			{:else}
				<h1 class="min-w-0 truncate text-sm font-bold tracking-tight">{threadTitle}</h1>
				{#if activeThread}
					<Button
						type="button"
						variant="ghost"
						size="icon-xs"
						class="shrink-0 text-muted-foreground hover:text-foreground"
						disabled={headerBusy}
						title="Rename thread"
						onclick={() => void beginRename()}
					>
						<PencilSimpleIcon size={16} />
					</Button>
				{/if}
			{/if}
		</div>
		<div class="ml-auto flex shrink-0 items-center gap-2">
			{#if activeThread}
				<Button
					variant="ghost"
					size="sm"
					class="gap-1.5 text-muted-foreground hover:text-destructive"
					disabled={headerBusy}
					onclick={() => void onDeleteThread(threadId)}
					title="Delete thread"
				>
					<TrashIcon size={16} />
					<span class="hidden sm:inline">Delete</span>
				</Button>
			{/if}
			<Button
				variant="ghost"
				size="sm"
				class="gap-1.5 hover:text-foreground"
				onclick={onToggleSidebar}
				aria-expanded={sidebarCollapsed}
				aria-controls="thread-sidebar stream-sidebar"
				title={sidebarCollapsed ? 'Show side panels' : 'Hide side panels'}
			>
				{#if sidebarCollapsed}
					<CaretDoubleLeftIcon size={16} />
					<span>Expand</span>
				{:else}
					<CaretDoubleRightIcon size={16} />
					<span>Collapse</span>
				{/if}
			</Button>
		</div>
	</div>
	{#if activeThread}
		<div class="flex flex-wrap items-center gap-2 pt-0.5 pb-1">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="flex h-7 min-w-0 items-center justify-between gap-1.5 rounded-md border border-border/60 bg-background/70 px-2 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
					disabled={isSending || isBootstrapping}
				>
					<div class="flex items-center gap-1.5 truncate">
						<KeyIcon
							size={11}
							weight={selectedCredential ? 'fill' : 'regular'}
							class={selectedCredential ? 'text-primary' : 'text-muted-foreground/40'}
						/>
						{#if selectedCredential}
							<span
								class="rounded bg-muted px-1 py-0.5 text-[9px] font-black tracking-widest text-muted-foreground/60 uppercase"
								>{PROVIDER_LABELS[selectedCredential.provider] ?? selectedCredential.provider}</span
							>
							<span class="max-w-20 truncate text-foreground">{selectedCredential.name}</span>
						{:else}
							<span class="text-muted-foreground/40">No key</span>
						{/if}
					</div>
					<span
						class="ml-1 text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
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
						{#if openAiCredentials.length > 0}
							{#if openRouterCredentials.length > 0 || copilotCredentials.length > 0}
								<DropdownMenu.Separator />
							{/if}
							<DropdownMenu.Label
								class="text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
								>OpenAI</DropdownMenu.Label
							>
							{#each openAiCredentials as cred (cred.id)}
								<DropdownMenu.Item
									class="flex items-center justify-between rounded-md py-2"
									onclick={() => onSelectCredential(cred.id)}
								>
									<div class="flex flex-col gap-0.5">
										<div class="flex items-center gap-1.5">
											<span
												class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
												>OpenAI</span
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
					class="flex h-7 min-w-0 items-center justify-between gap-1.5 rounded-md border border-border/60 bg-background/70 px-2 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
					disabled={isSending || isBootstrapping}
				>
					<div class="flex items-center gap-1.5 truncate">
						<ChatCircleTextIcon
							size={11}
							weight={selectedSystemPrompt ? 'fill' : 'regular'}
							class={selectedSystemPrompt ? 'text-primary' : 'text-muted-foreground/40'}
						/>
						<span
							class="max-w-24 truncate {selectedSystemPrompt
								? 'text-foreground'
								: 'text-muted-foreground/40'}"
						>
							{selectedSystemPrompt ? selectedSystemPrompt.name : 'System'}
						</span>
					</div>
					<span
						class="ml-1 text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
						>sys</span
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
										>{prompt.content.slice(0, PREVIEW_LENGTH)}{prompt.content.length >
										PREVIEW_LENGTH
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

			<div class="relative">
				<DropdownMenu.Root bind:open={modelDropdownOpen}>
					<DropdownMenu.Trigger
						class="flex h-7 min-w-0 items-center gap-1.5 rounded-md border border-border/60 bg-background/70 pr-6 pl-2 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
						disabled={isSending || isBootstrapping}
					>
						<div class="flex min-w-0 items-center gap-1.5 truncate">
							<CubeIcon
								size={11}
								weight={model.trim() ? 'fill' : 'regular'}
								class={model.trim() ? 'text-primary' : 'text-muted-foreground/40'}
							/>
							<span
								class="max-w-32 {model.trim()
									? 'truncate text-foreground'
									: 'text-muted-foreground/40'}"
							>
								{model.trim() || 'Model'}
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
				{#if model.trim() && !isSending && !isBootstrapping}
					<button
						class="hover:text-destructive-foreground absolute top-1/2 right-1.5 flex h-4 w-4 -translate-y-1/2 items-center justify-center rounded-full bg-muted-foreground/20 text-muted-foreground transition-all hover:bg-destructive/80"
						onclick={clearModel}
						tabindex="-1"
						aria-label="Clear model"
					>
						<XIcon size={8} weight="bold" />
					</button>
				{/if}
			</div>

			<div class="h-4 w-px bg-border/50"></div>

			<ThreadTagManager
				{threadTags}
				{availableTags}
				isLoading={isTagLoading}
				{onAddTag}
				{onRemoveTag}
				{onCreateTag}
			/>
		</div>
	{/if}
</header>
