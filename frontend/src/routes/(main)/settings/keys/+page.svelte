<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { KeyIcon, PencilSimpleIcon, PlusIcon, TrashIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import {
		createOpenRouterKey,
		deleteOpenRouterKey,
		updateOpenRouterKey
	} from '$lib/openrouter-key-client';
	import {
		invalidateOpenRouterKeys,
		openRouterKeysQueryOptions
	} from '$lib/queries/openrouter-key-query';
	import type { OpenRouterApiKey } from '$lib/types';

	interface KeyDraft {
		name: string;
		apiKey: string;
	}

	const queryClient = useQueryClient();
	const openRouterKeysQuery = createQuery(() => openRouterKeysQueryOptions());

	let newKeyName = $state('');
	let newApiKey = $state('');
	let formError = $state('');
	let draftsById = $state<Record<string, KeyDraft>>({});

	const createKeyMutation = createMutation(() => ({
		mutationFn: ({ name, apiKey }: KeyDraft) => createOpenRouterKey({ name, apiKey }),
		onSuccess: async () => {
			newKeyName = '';
			newApiKey = '';
			formError = '';
			await invalidateOpenRouterKeys(queryClient);
		}
	}));

	const updateKeyMutation = createMutation(() => ({
		mutationFn: ({ keyId, name, apiKey }: { keyId: string; name: string; apiKey: string }) =>
			updateOpenRouterKey(keyId, { name, apiKey }),
		onSuccess: async (_updatedKey, variables) => {
			clearDraft(variables.keyId);
			await invalidateOpenRouterKeys(queryClient);
		}
	}));

	const deleteKeyMutation = createMutation(() => ({
		mutationFn: (keyId: string) => deleteOpenRouterKey(keyId),
		onSuccess: async (_result, keyId) => {
			clearDraft(keyId);
			await invalidateOpenRouterKeys(queryClient);
		}
	}));

	const keys = $derived((openRouterKeysQuery.data ?? []) as OpenRouterApiKey[]);
	const queryError = $derived(
		openRouterKeysQuery.error instanceof Error ? openRouterKeysQuery.error.message : ''
	);
	const mutationError = $derived.by(() => {
		if (createKeyMutation.error instanceof Error) {
			return createKeyMutation.error.message;
		}

		if (updateKeyMutation.error instanceof Error) {
			return updateKeyMutation.error.message;
		}

		if (deleteKeyMutation.error instanceof Error) {
			return deleteKeyMutation.error.message;
		}

		return '';
	});

	function getDraft(key: OpenRouterApiKey): KeyDraft {
		return draftsById[key.id] ?? { name: key.name, apiKey: key.apiKey };
	}

	function updateDraft(keyId: string, field: keyof KeyDraft, value: string) {
		draftsById = {
			...draftsById,
			[keyId]: {
				...draftsById[keyId],
				[field]: value
			}
		};
	}

	function clearDraft(keyId: string) {
		const nextDrafts = { ...draftsById };
		delete nextDrafts[keyId];
		draftsById = nextDrafts;
	}

	function hasDraftChanges(key: OpenRouterApiKey) {
		const draft = getDraft(key);
		return draft.name !== key.name || draft.apiKey !== key.apiKey;
	}

	function submitCreate(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const name = newKeyName.trim();
		const apiKey = newApiKey.trim();

		if (!name || !apiKey) {
			formError = 'Give the key a name and paste the API key.';
			return;
		}

		createKeyMutation.mutate({ name, apiKey });
	}

	function submitUpdate(key: OpenRouterApiKey) {
		const draft = getDraft(key);
		updateKeyMutation.mutate({
			keyId: key.id,
			name: draft.name.trim(),
			apiKey: draft.apiKey.trim()
		});
	}

	function handleDelete(keyId: string) {
		deleteKeyMutation.mutate(keyId);
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-6 py-1 backdrop-blur-xl">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<KeyIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">API Keys</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-4xl space-y-10 px-6 py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-[0.2em] text-muted-foreground/50 uppercase">
						Provider configuration
					</p>
					<h2 class="text-3xl font-bold tracking-tight">OpenRouter Keys</h2>
					<p class="max-w-2xl text-sm text-muted-foreground/60">
						Saved keys are attached to your account. Label them clearly to stay organized across
						different workspaces.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Unique Labels
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Each key name must be distinct within your account to prevent collision.
						</p>
					</div>
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Query Cache
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							All changes are synchronized instantly via TanStack Query.
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
						<h3 class="text-sm font-bold tracking-tight uppercase">Add a key</h3>
					</div>

					<div class="rounded-[2rem] border bg-card/50 p-6 shadow-sm backdrop-blur-sm">
						<form class="space-y-5" onsubmit={submitCreate}>
							<div class="space-y-2">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="key-name">Label</label
								>
								<Input
									id="key-name"
									bind:value={newKeyName}
									placeholder="Personal workspace"
									class="h-10 rounded-xl bg-background/50"
									disabled={createKeyMutation.isPending}
								/>
							</div>

							<div class="space-y-2">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="api-key">API key</label
								>
								<Input
									id="api-key"
									bind:value={newApiKey}
									type="password"
									placeholder="sk-or-v1-..."
									class="h-10 rounded-xl bg-background/50"
									disabled={createKeyMutation.isPending}
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
								disabled={createKeyMutation.isPending}
							>
								{createKeyMutation.isPending ? 'Saving...' : 'Save key'}
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
								<KeyIcon size={16} weight="fill" />
							</div>
							<h3 class="text-sm font-bold tracking-tight uppercase">Saved keys</h3>
						</div>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{keys.length} total
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
						{#if openRouterKeysQuery.isPending}
							<div
								class="rounded-3xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading keys...
							</div>
						{:else if keys.length === 0}
							<div
								class="flex flex-col items-center justify-center rounded-3xl border border-dashed bg-muted/20 px-5 py-12 text-center"
							>
								<div
									class="flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/5 text-primary/30"
								>
									<KeyIcon size={24} weight="fill" />
								</div>
								<h4 class="mt-4 text-sm font-bold tracking-tight">No keys saved yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Add your first OpenRouter key to get started.
								</p>
							</div>
						{:else}
							{#each keys as key (key.id)}
								{@const draft = getDraft(key)}
								<div class="rounded-[2rem] border bg-card/50 p-6 shadow-sm backdrop-blur-sm">
									<div class="flex flex-wrap items-start justify-between gap-4">
										<div class="space-y-1">
											<p
												class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
											>
												Record ID
											</p>
											<p class="font-mono text-[10px] text-muted-foreground/60">{key.id}</p>
										</div>
										<div class="flex items-center gap-2">
											<Button
												variant="outline"
												size="sm"
												class="h-8 rounded-lg px-3 text-[11px] font-bold tracking-wider uppercase"
												disabled={!hasDraftChanges(key) || updateKeyMutation.isPending}
												onclick={() => submitUpdate(key)}
											>
												<PencilSimpleIcon size={14} weight="bold" class="mr-1.5" />
												<span>{updateKeyMutation.isPending ? 'Saving' : 'Save'}</span>
											</Button>
											<Button
												variant="outline"
												size="sm"
												class="h-8 rounded-lg px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
												disabled={deleteKeyMutation.isPending}
												onclick={() => handleDelete(key.id)}
											>
												<TrashIcon size={14} weight="bold" class="mr-1.5" />
												<span>{deleteKeyMutation.isPending ? 'Deleting' : 'Delete'}</span>
											</Button>
										</div>
									</div>

									<div class="mt-6 grid gap-4 sm:grid-cols-2">
										<div class="space-y-2">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`name-${key.id}`}>Label</label
											>
											<Input
												id={`name-${key.id}`}
												value={draft.name}
												placeholder="Workspace label"
												class="h-10 rounded-xl bg-background/50"
												oninput={(event) => updateDraft(key.id, 'name', event.currentTarget.value)}
											/>
										</div>

										<div class="space-y-2">
											<label
												class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
												for={`value-${key.id}`}>API key</label
											>
											<Input
												id={`value-${key.id}`}
												value={draft.apiKey}
												type="password"
												placeholder="sk-or-v1-..."
												class="h-10 rounded-xl bg-background/50"
												oninput={(event) =>
													updateDraft(key.id, 'apiKey', event.currentTarget.value)}
											/>
										</div>
									</div>
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
