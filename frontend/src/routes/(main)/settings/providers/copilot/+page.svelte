<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import {
		GithubLogoIcon,
		TrashIcon,
		CubeIcon,
		ClipboardTextIcon,
		ArrowSquareOutIcon,
		SpinnerGapIcon,
		CheckCircleIcon
	} from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import {
		deleteCopilotToken,
		initiateDeviceCode,
		pollDeviceCode,
		type DeviceCodeResponse
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

	const queryClient = useQueryClient();
	const copilotTokensQuery = createQuery(() => copilotTokensQueryOptions());
	const copilotModelsQuery = createQuery(() => copilotModelsQueryOptions());

	// Device code flow state
	let deviceFlow = $state<{
		phase: 'idle' | 'loading' | 'awaiting' | 'complete' | 'error';
		data?: DeviceCodeResponse;
		error?: string;
		name: string;
		copied: boolean;
	}>({ phase: 'idle', name: '', copied: false });

	let pollTimer: ReturnType<typeof setInterval> | null = null;

	const tokens = $derived((copilotTokensQuery.data ?? []) as CopilotToken[]);
	const models = $derived((copilotModelsQuery.data ?? []) as CopilotModel[]);

	// Model form state
	let newModelId = $state('');
	let modelFormError = $state('');

	const deleteTokenMutation = createMutation(() => ({
		mutationFn: (tokenId: string) => deleteCopilotToken(tokenId),
		onSuccess: async () => {
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

	const queryError = $derived(
		copilotTokensQuery.error instanceof Error ? copilotTokensQuery.error.message : ''
	);
	const deleteError = $derived(
		deleteTokenMutation.error instanceof Error ? deleteTokenMutation.error.message : ''
	);

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

	function stopPolling() {
		if (pollTimer) {
			clearInterval(pollTimer);
			pollTimer = null;
		}
	}

	async function startDeviceFlow() {
		const name = deviceFlow.name.trim();
		if (!name) {
			deviceFlow = { ...deviceFlow, phase: 'error', error: 'Give this connection a name first.' };
			return;
		}

		deviceFlow = { ...deviceFlow, phase: 'loading', error: undefined, copied: false };

		try {
			const data = await initiateDeviceCode();
			deviceFlow = { ...deviceFlow, phase: 'awaiting', data };

			const interval = (data.interval || 5) * 1000;

			pollTimer = setInterval(async () => {
				try {
					const result = await pollDeviceCode(data.deviceCode, name);

					if (result.status === 'complete') {
						stopPolling();
						deviceFlow = { phase: 'complete', name: '', copied: false };
						await invalidateCopilotTokens(queryClient);
					} else if (result.status === 'expired') {
						stopPolling();
						deviceFlow = {
							phase: 'error',
							name,
							copied: false,
							error: 'The device code expired. Please try again.'
						};
					}
					// pending / slow_down: keep polling
				} catch (err) {
					stopPolling();
					deviceFlow = {
						phase: 'error',
						name,
						copied: false,
						error: err instanceof Error ? err.message : 'Polling failed.'
					};
				}
			}, interval);
		} catch (err) {
			deviceFlow = {
				...deviceFlow,
				phase: 'error',
				error: err instanceof Error ? err.message : 'Failed to start device flow.'
			};
		}
	}

	function cancelDeviceFlow() {
		stopPolling();
		deviceFlow = { phase: 'idle', name: deviceFlow.name, copied: false };
	}

	async function copyCode() {
		if (deviceFlow.data?.userCode) {
			await navigator.clipboard.writeText(deviceFlow.data.userCode);
			deviceFlow = { ...deviceFlow, copied: true };
			setTimeout(() => {
				deviceFlow = { ...deviceFlow, copied: false };
			}, 2000);
		}
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
					<h2 class="text-base font-bold tracking-tight">Copilot Connections</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Sign in with your GitHub account to authorize Copilot access. The OAuth token is stored
						securely and exchanged for short-lived API tokens automatically.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Device code flow
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Click "Sign in with GitHub" below, then enter the code at github.com/login/device in
							your browser. No API keys needed.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Token management
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Your OAuth token is exchanged for short-lived Copilot API tokens behind the scenes.
							Tokens refresh automatically before they expire.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Connect account
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						{#if deviceFlow.phase === 'idle' || deviceFlow.phase === 'error'}
							<div class="space-y-4">
								<div class="space-y-1.5">
									<label
										class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
										for="connection-name">Connection name</label
									>
									<Input
										id="connection-name"
										bind:value={deviceFlow.name}
										placeholder="Work account"
										class="h-8 rounded-md bg-background/50"
									/>
								</div>

								{#if deviceFlow.phase === 'error' && deviceFlow.error}
									<p
										class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-2 text-[11px] font-medium text-destructive"
									>
										{deviceFlow.error}
									</p>
								{/if}

								<Button
									class="h-8 w-full rounded-md shadow-sm shadow-primary/10 transition-transform active:scale-[0.98]"
									onclick={startDeviceFlow}
								>
									<GithubLogoIcon size={14} weight="fill" class="mr-2" />
									Sign in with GitHub
								</Button>
							</div>
						{:else if deviceFlow.phase === 'loading'}
							<div class="flex items-center justify-center gap-2 py-6">
								<SpinnerGapIcon size={16} weight="bold" class="animate-spin text-primary" />
								<span class="text-xs text-muted-foreground">Starting authorization...</span>
							</div>
						{:else if deviceFlow.phase === 'awaiting' && deviceFlow.data}
							<div class="space-y-4">
								<div class="space-y-2 text-center">
									<p class="text-xs text-muted-foreground/60">Enter this code at GitHub:</p>
									<div class="flex items-center justify-center gap-2">
										<code
											class="rounded-lg border bg-muted/50 px-4 py-2 font-mono text-lg font-black tracking-[0.3em]"
										>
											{deviceFlow.data.userCode}
										</code>
										<Button
											variant="outline"
											size="sm"
											class="h-8 w-8 shrink-0 rounded-md p-0"
											onclick={copyCode}
											title="Copy code"
										>
											{#if deviceFlow.copied}
												<CheckCircleIcon size={14} weight="fill" class="text-green-500" />
											{:else}
												<ClipboardTextIcon size={14} weight="bold" />
											{/if}
										</Button>
									</div>
								</div>

								<!-- eslint-disable svelte/no-navigation-without-resolve -->
								<a
									href={deviceFlow.data.verificationUri}
									target="_blank"
									rel="noopener noreferrer"
									class="flex h-8 w-full items-center justify-center gap-2 rounded-md border bg-background/50 text-xs font-medium text-foreground/80 transition-colors hover:bg-accent"
								>
									<ArrowSquareOutIcon size={13} weight="bold" />
									Open github.com/login/device
								</a>
								<!-- eslint-enable svelte/no-navigation-without-resolve -->

								<div
									class="flex items-center justify-center gap-2 rounded-lg border border-dashed bg-muted/20 px-4 py-3"
								>
									<SpinnerGapIcon size={13} weight="bold" class="animate-spin text-primary/60" />
									<span class="text-[11px] text-muted-foreground/60"
										>Waiting for authorization...</span
									>
								</div>

								<Button
									variant="outline"
									size="sm"
									class="h-7 w-full text-[11px]"
									onclick={cancelDeviceFlow}
								>
									Cancel
								</Button>
							</div>
						{:else if deviceFlow.phase === 'complete'}
							<div class="flex flex-col items-center gap-3 py-4">
								<CheckCircleIcon size={24} weight="fill" class="text-green-500" />
								<p class="text-sm font-medium">Connected successfully</p>
								<Button
									variant="outline"
									size="sm"
									class="h-7 text-[11px]"
									onclick={() => {
										deviceFlow = { phase: 'idle', name: '', copied: false };
									}}
								>
									Connect another account
								</Button>
							</div>
						{/if}
					</div>
				</section>

				<section class="space-y-4">
					<div class="flex items-center justify-between">
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Saved connections
						</h3>
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{tokens.length} total
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
						{#if copilotTokensQuery.isPending}
							<div
								class="rounded-xl border border-dashed bg-muted/20 px-5 py-12 text-center text-xs font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading connections...
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
								<h4 class="mt-4 text-sm font-bold tracking-tight">No connections yet</h4>
								<p class="mt-1 text-xs text-muted-foreground/50">
									Sign in with GitHub to connect your Copilot access.
								</p>
							</div>
						{:else}
							{#each tokens as token (token.id)}
								<div
									class="flex items-center justify-between rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm"
								>
									<div class="min-w-0 space-y-1">
										<p class="truncate text-sm font-medium">{token.name}</p>
										<p class="font-mono text-[10px] text-muted-foreground/40">
											{token.githubToken.slice(0, 8)}...
										</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										class="ml-4 h-7 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
										disabled={deleteTokenMutation.isPending}
										onclick={() => handleDelete(token.id)}
									>
										<TrashIcon size={13} weight="bold" class="mr-1.5" />
										<span>{deleteTokenMutation.isPending ? 'Removing' : 'Remove'}</span>
									</Button>
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
						Save Copilot model IDs to quickly select them from the chat composer. Use simple names
						like <span class="font-mono">gemini-3-flash-preview</span>,
						<span class="font-mono">gpt-4.1</span>, or
						<span class="font-mono">claude-sonnet-4</span>.
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
									placeholder="gemini-3-flash-preview"
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
										<p class="truncate font-mono text-xs text-foreground/80">
											{m.modelId}
										</p>
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
