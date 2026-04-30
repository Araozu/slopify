<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import {
		SparkleIcon,
		TrashIcon,
		ClipboardTextIcon,
		ArrowSquareOutIcon,
		SpinnerGapIcon,
		CheckCircleIcon,
		KeyIcon,
		PlusIcon
	} from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import {
		createOpenAiToken,
		deleteOpenAiToken,
		initiateOpenAiDeviceCode,
		pollOpenAiDeviceCode,
		type OpenAiDeviceCodeResponse
	} from '$lib/openai-token-client';
	import {
		invalidateOpenAiTokens,
		openAiTokensQueryOptions
	} from '$lib/queries/openai-token-query';
	import type { OpenAiToken } from '$lib/types';

	const queryClient = useQueryClient();
	const openAiTokensQuery = createQuery(() => openAiTokensQueryOptions());

	let newTokenName = $state('');
	let newTokenValue = $state('');
	let formError = $state('');

	let deviceFlow = $state<{
		phase: 'idle' | 'loading' | 'awaiting' | 'complete' | 'error';
		data?: OpenAiDeviceCodeResponse;
		error?: string;
		name: string;
		copied: boolean;
	}>({ phase: 'idle', name: '', copied: false });

	let pollTimer: ReturnType<typeof setInterval> | null = null;

	const tokens = $derived((openAiTokensQuery.data ?? []) as OpenAiToken[]);

	const createTokenMutation = createMutation(() => ({
		mutationFn: ({ name, token }: { name: string; token: string }) =>
			createOpenAiToken({ name, token }),
		onSuccess: async () => {
			newTokenName = '';
			newTokenValue = '';
			formError = '';
			await invalidateOpenAiTokens(queryClient);
		}
	}));

	const deleteTokenMutation = createMutation(() => ({
		mutationFn: (tokenId: string) => deleteOpenAiToken(tokenId),
		onSuccess: async () => {
			await invalidateOpenAiTokens(queryClient);
		}
	}));

	const queryError = $derived(
		openAiTokensQuery.error instanceof Error ? openAiTokensQuery.error.message : ''
	);
	const mutationError = $derived.by(() => {
		if (createTokenMutation.error instanceof Error) {
			return createTokenMutation.error.message;
		}

		if (deleteTokenMutation.error instanceof Error) {
			return deleteTokenMutation.error.message;
		}

		return '';
	});

	function stopPolling() {
		if (pollTimer) {
			clearInterval(pollTimer);
			pollTimer = null;
		}
	}

	$effect(() => {
		return () => {
			stopPolling();
		};
	});

	function submitCreateToken(event: SubmitEvent) {
		event.preventDefault();
		formError = '';

		const name = newTokenName.trim();
		const token = newTokenValue.trim();

		if (!name) {
			formError = 'Give this connection a name.';
			return;
		}

		if (!token) {
			formError = 'Paste an API key or OAuth refresh token.';
			return;
		}

		createTokenMutation.mutate({ name, token });
	}

	async function startDeviceFlow() {
		const name = deviceFlow.name.trim();
		if (!name) {
			deviceFlow = { ...deviceFlow, phase: 'error', error: 'Give this connection a name first.' };
			return;
		}

		deviceFlow = { ...deviceFlow, phase: 'loading', error: undefined, copied: false };

		try {
			const data = await initiateOpenAiDeviceCode();
			deviceFlow = { ...deviceFlow, phase: 'awaiting', data };

			const interval = (data.interval || 5) * 1000;

			pollTimer = setInterval(async () => {
				try {
					const result = await pollOpenAiDeviceCode(data.deviceAuthId, data.userCode, name);

					if (result.status === 'complete') {
						stopPolling();
						deviceFlow = { phase: 'complete', name: '', copied: false };
						await invalidateOpenAiTokens(queryClient);
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
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<SparkleIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">OpenAI</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Provider configuration
					</p>
					<h2 class="text-base font-bold tracking-tight">OpenAI Connections</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Connect via device sign-in or add a token manually. OpenAI API keys and OAuth refresh
						tokens are both supported.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Device flow
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Sign in at OpenAI in your browser, then Slopify stores a refresh token for automatic
							short-lived access token exchange.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Manual token
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Paste an OpenAI API key (<span class="font-mono">sk-...</span>) or an OAuth refresh
							token if you already have one.
						</p>
					</div>
				</div>
			</section>

			<div class="grid gap-8 lg:grid-cols-[1fr_1.5fr]">
				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Add token manually
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<form class="space-y-4" onsubmit={submitCreateToken}>
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="openai-token-name">Name</label
								>
								<Input
									id="openai-token-name"
									bind:value={newTokenName}
									placeholder="Personal"
									class="h-8 rounded-md bg-background/50"
									disabled={createTokenMutation.isPending}
								/>
							</div>

							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="openai-token-value">Token</label
								>
								<Input
									id="openai-token-value"
									bind:value={newTokenValue}
									placeholder="sk-... or rt_..."
									class="h-8 rounded-md bg-background/50 font-mono"
									type="password"
									disabled={createTokenMutation.isPending}
								/>
							</div>

							{#if formError || mutationError}
								<p
									class="rounded-md border border-destructive/20 bg-destructive/10 px-3 py-2 text-[11px] font-medium text-destructive"
								>
									{formError || mutationError}
								</p>
							{/if}

							<Button
								type="submit"
								class="h-8 w-full rounded-md shadow-sm shadow-primary/10 transition-transform active:scale-[0.98]"
								disabled={createTokenMutation.isPending}
							>
								<PlusIcon size={13} weight="bold" class="mr-1.5" />
								{createTokenMutation.isPending ? 'Saving...' : 'Save token'}
							</Button>
						</form>
					</div>
				</section>

				<section class="space-y-4">
					<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
						Device sign-in
					</h3>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						{#if deviceFlow.phase === 'idle' || deviceFlow.phase === 'error'}
							<div class="space-y-4">
								<div class="space-y-1.5">
									<label
										class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
										for="openai-connection-name">Connection name</label
									>
									<Input
										id="openai-connection-name"
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
									<SparkleIcon size={14} weight="fill" class="mr-2" />
									Sign in with OpenAI
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
									<p class="text-xs text-muted-foreground/60">Enter this code at OpenAI:</p>
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
									Open OpenAI verification page
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
			</div>

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

				{#if queryError || mutationError}
					<p
						class="rounded-xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
					>
						{queryError || mutationError}
					</p>
				{/if}

				<div class="space-y-3">
					{#if openAiTokensQuery.isPending}
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
								<KeyIcon size={20} weight="fill" />
							</div>
							<h4 class="mt-4 text-sm font-bold tracking-tight">No connections yet</h4>
							<p class="mt-1 text-xs text-muted-foreground/50">
								Add a token or sign in with OpenAI to get started.
							</p>
						</div>
					{:else}
						{#each tokens as token (token.id)}
							<div
								class="flex items-center justify-between rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm"
							>
								<div class="min-w-0 space-y-1">
									<div class="flex items-center gap-2">
										<p class="truncate text-sm font-medium">{token.name}</p>
										<span
											class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
										>
											{token.authType === 'api_key' ? 'API key' : 'OAuth'}
										</span>
									</div>
									<p class="font-mono text-[10px] text-muted-foreground/40">
										{token.token.slice(0, 8)}...
									</p>
								</div>
								<Button
									variant="outline"
									size="sm"
									class="ml-4 h-7 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider text-destructive uppercase hover:bg-destructive/10 hover:text-destructive"
									disabled={deleteTokenMutation.isPending}
									onclick={() => deleteTokenMutation.mutate(token.id)}
								>
									<TrashIcon size={13} weight="bold" class="mr-1.5" />
									<span>{deleteTokenMutation.isPending ? 'Removing' : 'Remove'}</span>
								</Button>
							</div>
						{/each}
					{/if}
				</div>
			</section>

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
