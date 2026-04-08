<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { ChatCircleTextIcon, PencilSimpleIcon, PlusIcon, TrashIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import {
		createSystemPrompt,
		deleteSystemPrompt,
		updateSystemPrompt
	} from '$lib/system-prompt-client';
	import {
		invalidateSystemPrompts,
		systemPromptsQueryOptions
	} from '$lib/queries/system-prompt-query';
	import type { SystemPrompt } from '$lib/types';

	interface PromptDraft {
		name: string;
		content: string;
	}

	const queryClient = useQueryClient();
	const systemPromptsQuery = createQuery(() => systemPromptsQueryOptions());

	let newName = $state('');
	let newContent = $state('');
	let formError = $state('');
	let draftsById = $state<Record<string, PromptDraft>>({});

	const createPromptMutation = createMutation(() => ({
		mutationFn: ({ name, content }: PromptDraft) => createSystemPrompt({ name, content }),
		onSuccess: async () => {
			newName = '';
			newContent = '';
			formError = '';
			await invalidateSystemPrompts(queryClient);
		}
	}));

	const updatePromptMutation = createMutation(() => ({
		mutationFn: ({
			promptId,
			name,
			content
		}: {
			promptId: string;
			name: string;
			content: string;
		}) => updateSystemPrompt(promptId, { name, content }),
		onSuccess: async (_updated, variables) => {
			clearDraft(variables.promptId);
			await invalidateSystemPrompts(queryClient);
		}
	}));

	const deletePromptMutation = createMutation(() => ({
		mutationFn: (promptId: string) => deleteSystemPrompt(promptId),
		onSuccess: async (_result, promptId) => {
			clearDraft(promptId);
			await invalidateSystemPrompts(queryClient);
		}
	}));

	const prompts = $derived((systemPromptsQuery.data ?? []) as SystemPrompt[]);
	const queryError = $derived(
		systemPromptsQuery.error instanceof Error ? systemPromptsQuery.error.message : ''
	);
	const mutationError = $derived.by(() => {
		if (createPromptMutation.error instanceof Error) {
			return createPromptMutation.error.message;
		}
		if (updatePromptMutation.error instanceof Error) {
			return updatePromptMutation.error.message;
		}
		if (deletePromptMutation.error instanceof Error) {
			return deletePromptMutation.error.message;
		}
		return '';
	});

	function getDraft(prompt: SystemPrompt): PromptDraft {
		return draftsById[prompt.id] ?? { name: prompt.name, content: prompt.content };
	}

	function updateDraft(promptId: string, field: keyof PromptDraft, value: string) {
		draftsById = {
			...draftsById,
			[promptId]: {
				...draftsById[promptId],
				[field]: value
			}
		};
	}

	function clearDraft(promptId: string) {
		const nextDrafts = { ...draftsById };
		delete nextDrafts[promptId];
		draftsById = nextDrafts;
	}

	function hasDraftChanges(prompt: SystemPrompt) {
		const draft = getDraft(prompt);
		return draft.name !== prompt.name || draft.content !== prompt.content;
	}

	function submitCreate(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const name = newName.trim();
		const content = newContent.trim();

		if (!name || !content) {
			formError = 'Give the preset a name and enter the system prompt content.';
			return;
		}

		createPromptMutation.mutate({ name, content });
	}

	function submitUpdate(prompt: SystemPrompt) {
		const draft = getDraft(prompt);
		updatePromptMutation.mutate({
			promptId: prompt.id,
			name: draft.name.trim(),
			content: draft.content.trim()
		});
	}

	function handleDelete(promptId: string) {
		deletePromptMutation.mutate(promptId);
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<ChatCircleTextIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">System Prompts</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-4xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-[0.2em] text-muted-foreground/50 uppercase">
						Prompt configuration
					</p>
					<h2 class="text-3xl font-bold tracking-tight">System Prompt Presets</h2>
					<p class="max-w-2xl text-sm text-muted-foreground/60">
						Create reusable system prompt presets. Select one in the chat composer to give the AI a
						specific persona or set of instructions.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Unique Names
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Each preset name must be distinct within your account.
						</p>
					</div>
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Applied per message
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							The selected preset is prepended as a system message on each request.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-6">
					<div class="flex items-center gap-3">
						<div class="rounded-xl bg-primary/10 p-2 text-primary shadow-sm ring-1 ring-primary/20">
							<PlusIcon size={16} weight="bold" />
						</div>
						<h3 class="text-sm font-bold tracking-tight uppercase">Add a preset</h3>
					</div>

					<div class="rounded-[2rem] border bg-card/50 p-6 shadow-sm backdrop-blur-sm">
						<form class="space-y-5" onsubmit={submitCreate}>
							<div class="space-y-2">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="prompt-name">Name</label
								>
								<Input
									id="prompt-name"
									bind:value={newName}
									placeholder="Coding assistant"
									class="h-10 rounded-xl bg-background/50"
									disabled={createPromptMutation.isPending}
								/>
							</div>

							<div class="space-y-2">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="prompt-content">Content</label
								>
								<Textarea
									id="prompt-content"
									bind:value={newContent}
									placeholder="You are a helpful coding assistant..."
									class="min-h-32 resize-y rounded-xl bg-background/50 text-sm"
									disabled={createPromptMutation.isPending}
								/>
							</div>

							{#if formError}
								<p
									class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-2 text-[11px] font-medium text-destructive"
								>
									{formError}
								</p>
							{/if}

							<Button
								type="submit"
								class="h-10 w-full rounded-xl shadow-lg shadow-primary/10 transition-transform active:scale-[0.98]"
								disabled={createPromptMutation.isPending}
							>
								{createPromptMutation.isPending ? 'Saving...' : 'Save preset'}
							</Button>
						</form>
					</div>
				</section>

				<section class="space-y-6">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div
								class="rounded-xl bg-primary/10 p-2 text-primary shadow-sm ring-1 ring-primary/20"
							>
								<ChatCircleTextIcon size={16} weight="fill" />
							</div>
							<h3 class="text-sm font-bold tracking-tight uppercase">Saved presets</h3>
						</div>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{prompts.length} total
						</span>
					</div>

					{#if queryError || mutationError}
						<p
							class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
						>
							{queryError || mutationError}
						</p>
					{/if}

					<div class="space-y-4">
						{#if systemPromptsQuery.isPending}
							<div
								class="rounded-3xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading presets...
							</div>
						{:else if prompts.length === 0}
							<div
								class="flex flex-col items-center justify-center rounded-3xl border border-dashed bg-muted/20 px-5 py-12 text-center"
							>
								<div
									class="flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/5 text-primary/30"
								>
									<ChatCircleTextIcon size={24} weight="fill" />
								</div>
								<h4 class="mt-4 text-sm font-bold tracking-tight">No presets saved yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Create your first system prompt preset to get started.
								</p>
							</div>
						{:else}
							{#each prompts as prompt (prompt.id)}
								{@const draft = getDraft(prompt)}
								<div class="rounded-[2rem] border bg-card/50 p-6 shadow-sm backdrop-blur-sm">
									<div class="flex flex-wrap items-start justify-between gap-4">
										<div class="space-y-1">
											<p
												class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
											>
												Record ID
											</p>
											<p class="font-mono text-[10px] text-muted-foreground/60">{prompt.id}</p>
										</div>
										<div class="flex items-center gap-2">
											<Button
												variant="outline"
												size="sm"
												class="h-8 rounded-lg px-3 text-[11px] font-bold tracking-wider uppercase"
												disabled={!hasDraftChanges(prompt) || updatePromptMutation.isPending}
												onclick={() => submitUpdate(prompt)}
											>
												<PencilSimpleIcon size={14} weight="bold" class="mr-1.5" />
												<span>{updatePromptMutation.isPending ? 'Saving' : 'Save'}</span>
											</Button>
											<Button
												variant="outline"
												size="sm"
												class="h-8 rounded-lg px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
												disabled={deletePromptMutation.isPending}
												onclick={() => handleDelete(prompt.id)}
											>
												<TrashIcon size={14} weight="bold" class="mr-1.5" />
												<span>{deletePromptMutation.isPending ? 'Deleting' : 'Delete'}</span>
											</Button>
										</div>
									</div>

									<div class="mt-6 space-y-4">
										<div class="space-y-2">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`name-${prompt.id}`}>Name</label
											>
											<Input
												id={`name-${prompt.id}`}
												value={draft.name}
												placeholder="Preset name"
												class="h-10 rounded-xl bg-background/50"
												oninput={(event) =>
													updateDraft(prompt.id, 'name', event.currentTarget.value)}
											/>
										</div>

										<div class="space-y-2">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`content-${prompt.id}`}>Content</label
											>
											<Textarea
												id={`content-${prompt.id}`}
												value={draft.content}
												placeholder="System prompt content..."
												class="min-h-24 resize-y rounded-xl bg-background/50 text-sm"
												oninput={(event) =>
													updateDraft(prompt.id, 'content', event.currentTarget.value)}
											/>
										</div>
									</div>
								</div>
							{/each}
						{/if}
					</div>
				</section>
			</div>
		</div>
	</div>
</div>
