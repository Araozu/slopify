<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { SparkleIcon, TrashIcon, PlusIcon, KeyIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { createZenKey, deleteZenKey } from '$lib/zen-key-client';
	import { invalidateZenKeys, zenKeysQueryOptions } from '$lib/queries/zen-key-query';
	import type { ZenApiKey } from '$lib/types';

	const queryClient = useQueryClient();
	const keysQuery = createQuery(() => zenKeysQueryOptions());

	const keys = $derived((keysQuery.data ?? []) as ZenApiKey[]);

	let newName = $state('');
	let newApiKey = $state('');
	let formError = $state('');

	const createMut = createMutation(() => ({
		mutationFn: ({ name, apiKey }: { name: string; apiKey: string }) =>
			createZenKey({ name, apiKey }),
		onSuccess: async () => {
			newName = '';
			newApiKey = '';
			formError = '';
			await invalidateZenKeys(queryClient);
		}
	}));

	const deleteMut = createMutation(() => ({
		mutationFn: (keyId: string) => deleteZenKey(keyId),
		onSuccess: async () => {
			await invalidateZenKeys(queryClient);
		}
	}));

	const queryError = $derived(keysQuery.error instanceof Error ? keysQuery.error.message : '');
	const deleteError = $derived(deleteMut.error instanceof Error ? deleteMut.error.message : '');
	const createError = $derived(createMut.error instanceof Error ? createMut.error.message : '');

	function submitCreate(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const name = newName.trim();
		const apiKey = newApiKey.trim();

		if (!name) {
			formError = 'Give this key a name.';
			return;
		}
		if (!apiKey) {
			formError = 'Enter an API key.';
			return;
		}

		createMut.mutate({ name, apiKey });
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<SparkleIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">OpenCode Zen</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Provider configuration
					</p>
					<h2 class="text-base font-bold tracking-tight">API Keys</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Add your OpenCode Zen API keys. Zen is a multi-model gateway that supports Claude, Qwen,
						Kimi, GLM, and more through a single API.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Chat Completions
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Qwen, Kimi, GLM, MiniMax, and other models are routed through the OpenAI-compatible
							Chat Completions endpoint.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Claude models
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Claude models are routed through the Anthropic Messages endpoint. Use the
							<span class="font-mono">claude-*</span> model prefix.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Add a key
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<form class="space-y-4" onsubmit={submitCreate}>
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="zen-key-name">Name</label
								>
								<Input
									id="zen-key-name"
									bind:value={newName}
									placeholder="Personal"
									class="h-8 rounded-md bg-background/50"
									disabled={createMut.isPending}
								/>
							</div>

							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="zen-api-key">API Key</label
								>
								<Input
									id="zen-api-key"
									bind:value={newApiKey}
									placeholder="zen-..."
									class="h-8 rounded-md bg-background/50 font-mono"
									type="password"
									disabled={createMut.isPending}
								/>
							</div>

							{#if formError || createError}
								<p
									class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-[11px] font-medium text-destructive"
								>
									{formError || createError}
								</p>
							{/if}

							<Button
								type="submit"
								class="h-8 w-full rounded-md shadow-sm shadow-primary/10 transition-transform active:scale-[0.98]"
								disabled={createMut.isPending}
							>
								<PlusIcon size={13} weight="bold" class="mr-1.5" />
								{createMut.isPending ? 'Saving...' : 'Save key'}
							</Button>
						</form>
					</div>
				</section>

				<section class="space-y-4">
					<div class="flex items-center justify-between">
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Saved keys
						</h3>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{keys.length} total
						</span>
					</div>

					{#if queryError || deleteError}
						<p
							class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
						>
							{queryError || deleteError}
						</p>
					{/if}

					<div class="space-y-3">
						{#if keysQuery.isPending}
							<div
								class="rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading keys...
							</div>
						{:else if keys.length === 0}
							<div
								class="flex flex-col items-center justify-center rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center"
							>
								<div
									class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/30"
								>
									<KeyIcon size={20} weight="fill" />
								</div>
								<h4 class="mt-4 text-sm font-bold tracking-tight">No keys yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Add your first Zen API key to get started.
								</p>
							</div>
						{:else}
							{#each keys as key (key.id)}
								<div
									class="flex items-center justify-between rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm"
								>
									<div class="min-w-0 space-y-1">
										<p class="truncate text-sm font-medium">{key.name}</p>
										<p class="font-mono text-[10px] text-muted-foreground/40">
											{key.apiKey.slice(0, 8)}...
										</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										class="ml-4 h-7 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
										disabled={deleteMut.isPending}
										onclick={() => deleteMut.mutate(key.id)}
									>
										<TrashIcon size={13} weight="bold" class="mr-1.5" />
										<span>{deleteMut.isPending ? 'Removing' : 'Remove'}</span>
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
