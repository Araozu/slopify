<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { CubeIcon, TrashIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { createOpenRouterModel, deleteOpenRouterModel } from '$lib/openrouter-model-client';
	import {
		invalidateOpenRouterModels,
		openRouterModelsQueryOptions
	} from '$lib/queries/openrouter-model-query';
	import type { OpenRouterModel } from '$lib/types';

	const queryClient = useQueryClient();
	const openRouterModelsQuery = createQuery(() => openRouterModelsQueryOptions());

	let newModelId = $state('');
	let formError = $state('');

	const createModelMutation = createMutation(() => ({
		mutationFn: (modelId: string) => createOpenRouterModel(modelId),
		onSuccess: async () => {
			newModelId = '';
			formError = '';
			await invalidateOpenRouterModels(queryClient);
		}
	}));

	const deleteModelMutation = createMutation(() => ({
		mutationFn: (id: string) => deleteOpenRouterModel(id),
		onSuccess: async () => {
			await invalidateOpenRouterModels(queryClient);
		}
	}));

	const models = $derived((openRouterModelsQuery.data ?? []) as OpenRouterModel[]);
	const queryError = $derived(
		openRouterModelsQuery.error instanceof Error ? openRouterModelsQuery.error.message : ''
	);
	const mutationError = $derived.by(() => {
		if (createModelMutation.error instanceof Error) {
			return createModelMutation.error.message;
		}
		if (deleteModelMutation.error instanceof Error) {
			return deleteModelMutation.error.message;
		}
		return '';
	});

	function submitCreate(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const modelId = newModelId.trim();

		if (!modelId) {
			formError = 'Enter a model ID to save.';
			return;
		}

		createModelMutation.mutate(modelId);
	}

	function handleDelete(id: string) {
		deleteModelMutation.mutate(id);
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<CubeIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">OpenRouter Models</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Model configuration
					</p>
					<h2 class="text-base font-bold tracking-tight">Saved Models</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Save OpenRouter model IDs to quickly select them from the chat composer.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Model IDs
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Use the full OpenRouter model ID format, e.g. <span class="font-mono"
								>openai/gpt-4o-mini</span
							>.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Quick select
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Saved models appear in the model picker in the chat composer.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Add a model
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<form class="space-y-4" onsubmit={submitCreate}>
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="model-id">Model ID</label
								>
								<Input
									id="model-id"
									bind:value={newModelId}
									placeholder="openai/gpt-4o-mini"
									class="h-8 rounded-md bg-background/50 font-mono"
									disabled={createModelMutation.isPending}
								/>
							</div>

							{#if formError}
								<p
									class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-[11px] font-medium text-destructive"
								>
									{formError}
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

					{#if queryError || mutationError}
						<p
							class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-xs font-medium text-destructive"
						>
							{queryError || mutationError}
						</p>
					{/if}

					<div class="space-y-3">
						{#if openRouterModelsQuery.isPending}
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
									Add your first model ID to get started.
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
										onclick={() => handleDelete(m.id)}
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
		</div>
	</div>
</div>
