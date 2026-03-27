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

<div
	class="min-h-[calc(100vh-2rem)] bg-[radial-gradient(circle_at_top_left,theme(colors.primary/.14),transparent_32%),linear-gradient(180deg,theme(colors.background),theme(colors.muted/.35))] px-6 py-8 sm:px-8 lg:px-10"
>
	<div class="mx-auto flex w-full max-w-6xl flex-col gap-6">
		<section
			class="grid gap-6 rounded-[2rem] border bg-card/90 p-6 shadow-xl shadow-primary/5 backdrop-blur-xl lg:grid-cols-[1.05fr_0.95fr] lg:p-8"
		>
			<div class="space-y-4">
				<p class="text-[11px] font-black tracking-[0.3em] text-primary uppercase">
					OpenRouter keys
				</p>
				<h1 class="max-w-xl text-4xl leading-tight font-semibold tracking-tight">
					Keep your provider keys organized, not feral.
				</h1>
				<p class="max-w-2xl text-sm leading-6 text-muted-foreground">
					Each saved key belongs to your user account, so you can label them per workspace, rotate
					them later, and stop pasting secrets into the composer every single time.
				</p>
			</div>

			<div class="grid gap-3 sm:grid-cols-2">
				<div class="rounded-[1.5rem] border bg-background/80 p-4 shadow-sm">
					<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
						Names are unique
					</p>
					<p class="mt-3 text-sm text-foreground/80">
						One user can keep many keys, but each key name needs to be distinct.
					</p>
				</div>
				<div class="rounded-[1.5rem] border bg-background/80 p-4 shadow-sm">
					<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
						Powered by query cache
					</p>
					<p class="mt-3 text-sm text-foreground/80">
						Creates, updates, and deletes all round-trip through TanStack Query.
					</p>
				</div>
			</div>
		</section>

		<div class="grid gap-6 xl:grid-cols-[0.95fr_1.05fr]">
			<section class="rounded-[2rem] border bg-card/90 p-6 shadow-sm">
				<div class="flex items-center gap-3">
					<div class="rounded-2xl bg-primary/10 p-2 text-primary">
						<PlusIcon size={18} weight="bold" />
					</div>
					<div>
						<h2 class="text-xl font-semibold tracking-tight">Add a key</h2>
						<p class="text-sm text-muted-foreground">
							Name it like a grownup so future-you says thanks.
						</p>
					</div>
				</div>

				<form class="mt-6 space-y-4" onsubmit={submitCreate}>
					<div class="space-y-2">
						<label class="text-sm font-medium" for="key-name">Label</label>
						<Input
							id="key-name"
							bind:value={newKeyName}
							placeholder="Personal workspace"
							class="h-11 rounded-2xl bg-background/70"
							disabled={createKeyMutation.isPending}
						/>
					</div>

					<div class="space-y-2">
						<label class="text-sm font-medium" for="api-key">API key</label>
						<Input
							id="api-key"
							bind:value={newApiKey}
							type="password"
							placeholder="sk-or-v1-..."
							class="h-11 rounded-2xl bg-background/70"
							disabled={createKeyMutation.isPending}
						/>
					</div>

					{#if formError}
						<p
							class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
						>
							{formError}
						</p>
					{/if}

					<Button
						type="submit"
						class="h-11 w-full rounded-2xl"
						disabled={createKeyMutation.isPending}
					>
						{createKeyMutation.isPending ? 'Saving...' : 'Save key'}
					</Button>
				</form>
			</section>

			<section class="rounded-[2rem] border bg-card/90 p-6 shadow-sm">
				<div class="flex items-center justify-between gap-4">
					<div>
						<h2 class="text-xl font-semibold tracking-tight">Saved keys</h2>
						<p class="text-sm text-muted-foreground">
							Edit labels, rotate values, or delete old secrets.
						</p>
					</div>
					<div
						class="rounded-full bg-muted px-3 py-1 text-xs font-black tracking-[0.18em] text-muted-foreground uppercase"
					>
						{keys.length} total
					</div>
				</div>

				{#if queryError || mutationError}
					<p
						class="mt-4 rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
					>
						{queryError || mutationError}
					</p>
				{/if}

				<div class="mt-6 space-y-4">
					{#if openRouterKeysQuery.isPending}
						<div
							class="rounded-[1.5rem] border border-dashed bg-muted/30 px-5 py-10 text-center text-sm text-muted-foreground"
						>
							Loading your keys...
						</div>
					{:else if keys.length === 0}
						<div class="rounded-[1.5rem] border border-dashed bg-muted/30 px-5 py-10 text-center">
							<div
								class="mx-auto flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/10 text-primary"
							>
								<KeyIcon size={22} weight="fill" />
							</div>
							<h3 class="mt-4 text-lg font-semibold tracking-tight">No keys saved yet</h3>
							<p class="mt-2 text-sm leading-6 text-muted-foreground">
								Add your first OpenRouter key on the left and it will land here.
							</p>
						</div>
					{:else}
						{#each keys as key (key.id)}
							{@const draft = getDraft(key)}
							<div class="rounded-[1.5rem] border bg-background/70 p-5 shadow-sm">
								<div class="flex flex-wrap items-start justify-between gap-3">
									<div>
										<p
											class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase"
										>
											Key record
										</p>
										<p class="mt-2 font-mono text-xs text-muted-foreground">{key.id}</p>
									</div>
									<div class="flex items-center gap-2">
										<Button
											variant="outline"
											class="h-9 rounded-xl px-3"
											disabled={!hasDraftChanges(key) || updateKeyMutation.isPending}
											onclick={() => submitUpdate(key)}
										>
											<PencilSimpleIcon size={16} weight="bold" />
											<span>{updateKeyMutation.isPending ? 'Saving...' : 'Save'}</span>
										</Button>
										<Button
											variant="outline"
											class="h-9 rounded-xl px-3 text-destructive"
											disabled={deleteKeyMutation.isPending}
											onclick={() => handleDelete(key.id)}
										>
											<TrashIcon size={16} weight="bold" />
											<span>{deleteKeyMutation.isPending ? 'Deleting...' : 'Delete'}</span>
										</Button>
									</div>
								</div>

								<div class="mt-5 grid gap-4 md:grid-cols-2">
									<div class="space-y-2">
										<label class="text-sm font-medium" for={`name-${key.id}`}>Label</label>
										<Input
											id={`name-${key.id}`}
											value={draft.name}
											placeholder="Workspace label"
											class="h-11 rounded-2xl bg-card"
											oninput={(event) => updateDraft(key.id, 'name', event.currentTarget.value)}
										/>
									</div>

									<div class="space-y-2">
										<label class="text-sm font-medium" for={`value-${key.id}`}>API key</label>
										<Input
											id={`value-${key.id}`}
											value={draft.apiKey}
											type="password"
											placeholder="sk-or-v1-..."
											class="h-11 rounded-2xl bg-card"
											oninput={(event) => updateDraft(key.id, 'apiKey', event.currentTarget.value)}
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
