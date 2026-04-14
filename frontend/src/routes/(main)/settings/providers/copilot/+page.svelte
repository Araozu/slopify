<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { GithubLogoIcon, PencilSimpleIcon, TrashIcon, CubeIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import {
		createCopilotToken,
		deleteCopilotToken,
		updateCopilotToken
	} from '$lib/copilot-token-client';
	import {
		invalidateCopilotTokens,
		copilotTokensQueryOptions
	} from '$lib/queries/copilot-token-query';
	import { createCopilotModel, deleteCopilotModel } from '$lib/copilot-model-client';
	import {
		invalidateCopilotModels,
		copilotModelsQueryOptions
	} from '$lib/queries/copilot-model-query';
	import type { CopilotToken, CopilotModel } from '$lib/types';

	interface TokenDraft {
		name: string;
		githubToken: string;
	}

	const queryClient = useQueryClient();
	const copilotTokensQuery = createQuery(() => copilotTokensQueryOptions());
	const copilotModelsQuery = createQuery(() => copilotModelsQueryOptions());

	let newTokenName = $state('');
	let newGithubToken = $state('');
	let formError = $state('');
	let draftsById = $state<Record<string, TokenDraft>>({});

	let newModelId = $state('');
	let modelFormError = $state('');

	const createTokenMutation = createMutation(() => ({
		mutationFn: ({ name, githubToken }: TokenDraft) => createCopilotToken({ name, githubToken }),
		onSuccess: async () => {
			newTokenName = '';
			newGithubToken = '';
			formError = '';
			await invalidateCopilotTokens(queryClient);
		}
	}));

	const updateTokenMutation = createMutation(() => ({
		mutationFn: ({
			tokenId,
			name,
			githubToken
		}: {
			tokenId: string;
			name: string;
			githubToken: string;
		}) => updateCopilotToken(tokenId, { name, githubToken }),
		onSuccess: async (_updatedToken, variables) => {
			clearDraft(variables.tokenId);
			await invalidateCopilotTokens(queryClient);
		}
	}));

	const deleteTokenMutation = createMutation(() => ({
		mutationFn: (tokenId: string) => deleteCopilotToken(tokenId),
		onSuccess: async (_result, tokenId) => {
			clearDraft(tokenId);
			await invalidateCopilotTokens(queryClient);
		}
	}));

	const createModelMutation = createMutation(() => ({
		mutationFn: (modelId: string) => createCopilotModel(modelId),
		onSuccess: async () => {
			newModelId = '';
			modelFormError = '';
			await invalidateCopilotModels(queryClient);
		}
	}));

	const deleteModelMutation = createMutation(() => ({
		mutationFn: (id: string) => deleteCopilotModel(id),
		onSuccess: async () => {
			await invalidateCopilotModels(queryClient);
		}
	}));

	const tokens = $derived((copilotTokensQuery.data ?? []) as CopilotToken[]);
	const models = $derived((copilotModelsQuery.data ?? []) as CopilotModel[]);

	const queryError = $derived(
		copilotTokensQuery.error instanceof Error ? copilotTokensQuery.error.message : ''
	);
	const mutationError = $derived.by(() => {
		if (createTokenMutation.error instanceof Error) {
			return createTokenMutation.error.message;
		}

		if (updateTokenMutation.error instanceof Error) {
			return updateTokenMutation.error.message;
		}

		if (deleteTokenMutation.error instanceof Error) {
			return deleteTokenMutation.error.message;
		}

		return '';
	});

	const modelQueryError = $derived(
		copilotModelsQuery.error instanceof Error ? copilotModelsQuery.error.message : ''
	);
	const modelMutationError = $derived.by(() => {
		if (createModelMutation.error instanceof Error) {
			return createModelMutation.error.message;
		}
		if (deleteModelMutation.error instanceof Error) {
			return deleteModelMutation.error.message;
		}
		return '';
	});

	function getDraft(token: CopilotToken): TokenDraft {
		return draftsById[token.id] ?? { name: token.name, githubToken: token.githubToken };
	}

	function updateDraft(tokenId: string, field: keyof TokenDraft, value: string) {
		draftsById = {
			...draftsById,
			[tokenId]: {
				...draftsById[tokenId],
				[field]: value
			}
		};
	}

	function clearDraft(tokenId: string) {
		const nextDrafts = { ...draftsById };
		delete nextDrafts[tokenId];
		draftsById = nextDrafts;
	}

	function hasDraftChanges(token: CopilotToken) {
		const draft = getDraft(token);
		return draft.name !== token.name || draft.githubToken !== token.githubToken;
	}

	function submitCreate(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const name = newTokenName.trim();
		const githubToken = newGithubToken.trim();

		if (!name || !githubToken) {
			formError = 'Give the token a name and paste your GitHub PAT.';
			return;
		}

		createTokenMutation.mutate({ name, githubToken });
	}

	function submitUpdate(token: CopilotToken) {
		const draft = getDraft(token);
		updateTokenMutation.mutate({
			tokenId: token.id,
			name: draft.name.trim(),
			githubToken: draft.githubToken.trim()
		});
	}

	function handleDelete(tokenId: string) {
		deleteTokenMutation.mutate(tokenId);
	}

	function submitCreateModel(event: SubmitEvent) {
		event.preventDefault();
		modelFormError = '';

		const modelId = newModelId.trim();

		if (!modelId) {
			modelFormError = 'Enter a model ID to save.';
			return;
		}

		createModelMutation.mutate(modelId);
	}

	function handleDeleteModel(id: string) {
		deleteModelMutation.mutate(id);
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<GithubLogoIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">GitHub Copilot</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<!-- Tokens section -->
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Provider configuration
					</p>
					<h2 class="text-base font-bold tracking-tight">Copilot Tokens</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Save GitHub personal access tokens (classic) with <span class="font-mono">copilot</span>
						scope. The backend exchanges them for short-lived Copilot API tokens automatically.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							GitHub PAT
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Create a classic PAT at github.com/settings/tokens with the <span class="font-mono"
								>copilot</span
							> scope enabled.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Token exchange
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Your PAT is exchanged for a short-lived Copilot token on each request. The PAT itself
							is never sent to the model endpoint.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Add a token
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<form class="space-y-4" onsubmit={submitCreate}>
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="token-name">Label</label
								>
								<Input
									id="token-name"
									bind:value={newTokenName}
									placeholder="Work account"
									class="h-8 rounded-md bg-background/50"
									disabled={createTokenMutation.isPending}
								/>
							</div>

							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="github-token">GitHub PAT</label
								>
								<Input
									id="github-token"
									bind:value={newGithubToken}
									type="password"
									placeholder="ghp_..."
									class="h-8 rounded-md bg-background/50"
									disabled={createTokenMutation.isPending}
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
								class="h-8 w-full rounded-md shadow-sm shadow-primary/10 transition-transform active:scale-[0.98]"
								disabled={createTokenMutation.isPending}
							>
								{createTokenMutation.isPending ? 'Saving...' : 'Save token'}
							</Button>
						</form>
					</div>
				</section>

				<section class="space-y-4">
					<div class="flex items-center justify-between">
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Saved tokens
						</h3>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{tokens.length} total
						</span>
					</div>

					{#if queryError || mutationError}
						<p
							class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
						>
							{queryError || mutationError}
						</p>
					{/if}

					<div class="space-y-3">
						{#if copilotTokensQuery.isPending}
							<div
								class="rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading tokens...
							</div>
						{:else if tokens.length === 0}
							<div
								class="flex flex-col items-center justify-center rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center"
							>
								<div
									class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/30"
								>
									<GithubLogoIcon size={20} weight="fill" />
								</div>
								<h4 class="mt-4 text-sm font-bold tracking-tight">No tokens saved yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Add your first GitHub PAT to get started with Copilot.
								</p>
							</div>
						{:else}
							{#each tokens as token (token.id)}
								{@const draft = getDraft(token)}
								<div class="rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm">
									<div class="flex flex-wrap items-start justify-between gap-3">
										<div class="space-y-0.5">
											<p
												class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
											>
												Record ID
											</p>
											<p class="font-mono text-[10px] text-muted-foreground/60">{token.id}</p>
										</div>
										<div class="flex items-center gap-2">
											<Button
												variant="outline"
												size="sm"
												class="h-7 rounded-md px-3 text-[11px] font-bold tracking-wider uppercase"
												disabled={!hasDraftChanges(token) || updateTokenMutation.isPending}
												onclick={() => submitUpdate(token)}
											>
												<PencilSimpleIcon size={13} weight="bold" class="mr-1.5" />
												<span>{updateTokenMutation.isPending ? 'Saving' : 'Save'}</span>
											</Button>
											<Button
												variant="outline"
												size="sm"
												class="h-7 rounded-md px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
												disabled={deleteTokenMutation.isPending}
												onclick={() => handleDelete(token.id)}
											>
												<TrashIcon size={13} weight="bold" class="mr-1.5" />
												<span>{deleteTokenMutation.isPending ? 'Deleting' : 'Delete'}</span>
											</Button>
										</div>
									</div>

									<div class="mt-4 grid gap-3 sm:grid-cols-2">
										<div class="space-y-1.5">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`name-${token.id}`}>Label</label
											>
											<Input
												id={`name-${token.id}`}
												value={draft.name}
												placeholder="Token label"
												class="h-8 rounded-md bg-background/50"
												oninput={(event) =>
													updateDraft(token.id, 'name', event.currentTarget.value)}
											/>
										</div>

										<div class="space-y-1.5">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`value-${token.id}`}>GitHub PAT</label
											>
											<Input
												id={`value-${token.id}`}
												value={draft.githubToken}
												type="password"
												placeholder="ghp_..."
												class="h-8 rounded-md bg-background/50"
												oninput={(event) =>
													updateDraft(token.id, 'githubToken', event.currentTarget.value)}
											/>
										</div>
									</div>
								</div>
							{/each}
						{/if}
					</div>
				</section>
			</div>

			<!-- Models section -->
			<section class="space-y-4 border-t pt-10">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Model configuration
					</p>
					<h2 class="text-base font-bold tracking-tight">Saved Models</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Save Copilot model IDs to quickly select them from the chat composer. Use the model IDs
						supported by your Copilot subscription.
					</p>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Add a model
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<form class="space-y-4" onsubmit={submitCreateModel}>
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="copilot-model-id">Model ID</label
								>
								<Input
									id="copilot-model-id"
									bind:value={newModelId}
									placeholder="gpt-4o"
									class="h-8 rounded-md bg-background/50 font-mono"
									disabled={createModelMutation.isPending}
								/>
							</div>

							{#if modelFormError}
								<p
									class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-[11px] font-medium text-destructive"
								>
									{modelFormError}
								</p>
							{/if}

							<Button
								type="submit"
								class="h-8 w-full rounded-md shadow-sm shadow-primary/10 transition-transform active:scale-[0.98]"
								disabled={createModelMutation.isPending}
							>
								{createModelMutation.isPending ? 'Saving...' : 'Save model'}
							</Button>
						</form>
					</div>
				</section>

				<section class="space-y-4">
					<div class="flex items-center justify-between">
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Saved models
						</h3>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{models.length} total
						</span>
					</div>

					{#if modelQueryError || modelMutationError}
						<p
							class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-xs font-medium text-destructive"
						>
							{modelQueryError || modelMutationError}
						</p>
					{/if}

					<div class="space-y-3">
						{#if copilotModelsQuery.isPending}
							<div
								class="rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading models...
							</div>
						{:else if models.length === 0}
							<div
								class="flex flex-col items-center justify-center rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center"
							>
								<div
									class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/30"
								>
									<CubeIcon size={20} weight="fill" />
								</div>
								<h4 class="mt-4 text-sm font-bold tracking-tight">No models saved yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Add your first Copilot model ID to get started.
								</p>
							</div>
						{:else}
							{#each models as m (m.id)}
								<div
									class="flex items-center justify-between rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm"
								>
									<div class="min-w-0 space-y-0.5">
										<p
											class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
										>
											Model ID
										</p>
										<p class="truncate font-mono text-xs text-foreground/80">{m.modelId}</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										class="ml-4 h-7 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
										disabled={deleteModelMutation.isPending}
										onclick={() => handleDeleteModel(m.id)}
									>
										<TrashIcon size={13} weight="bold" class="mr-1.5" />
										<span>{deleteModelMutation.isPending ? 'Deleting' : 'Delete'}</span>
									</Button>
								</div>
							{/each}
						{/if}
					</div>
				</section>
			</div>

			<footer class="pt-10">
				<p
					class="text-center text-[10px] font-medium tracking-widest text-muted-foreground/30 uppercase"
				>
					Provider secrets are encrypted at rest. Be careful where you paste them.
				</p>
			</footer>
		</div>
	</div>
</div>
