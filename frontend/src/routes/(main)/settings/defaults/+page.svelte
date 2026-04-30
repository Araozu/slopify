<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import {
		SlidersHorizontalIcon,
		KeyIcon,
		ChatCircleTextIcon,
		CubeIcon,
		CheckIcon,
		XIcon
	} from 'phosphor-svelte';
	import { resolve } from '$app/paths';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { openRouterKeysQueryOptions } from '$lib/queries/openrouter-key-query';
	import { copilotTokensQueryOptions } from '$lib/queries/copilot-token-query';
	import { openAiTokensQueryOptions } from '$lib/queries/openai-token-query';
	import { systemPromptsQueryOptions } from '$lib/queries/system-prompt-query';
	import { openRouterModelsQueryOptions } from '$lib/queries/openrouter-model-query';
	import { copilotModelsQueryOptions } from '$lib/queries/copilot-model-query';
	import { threadDefaults } from '$lib/stores/thread-defaults';
	import type {
		OpenRouterApiKey,
		CopilotToken,
		OpenAiToken,
		SystemPrompt,
		OpenRouterModel,
		CopilotModel,
		ProviderCredential
	} from '$lib/types';

	const keysQuery = createQuery(() => openRouterKeysQueryOptions());
	const copilotTokensQuery = createQuery(() => copilotTokensQueryOptions());
	const openAiTokensQuery = createQuery(() => openAiTokensQueryOptions());
	const systemPromptsQuery = createQuery(() => systemPromptsQueryOptions());
	const openRouterModelsQuery = createQuery(() => openRouterModelsQueryOptions());
	const copilotModelsQuery = createQuery(() => copilotModelsQueryOptions());

	const openRouterKeys = $derived((keysQuery.data ?? []) as OpenRouterApiKey[]);
	const copilotTokens = $derived((copilotTokensQuery.data ?? []) as CopilotToken[]);
	const openAiTokens = $derived((openAiTokensQuery.data ?? []) as OpenAiToken[]);

	const credentials = $derived<ProviderCredential[]>([
		...openRouterKeys.map(
			(k): ProviderCredential => ({
				id: k.id,
				name: k.name,
				provider: 'openrouter',
				token: k.apiKey
			})
		),
		...copilotTokens.map(
			(t): ProviderCredential => ({
				id: t.id,
				name: t.name,
				provider: 'github-copilot',
				token: t.githubToken
			})
		),
		...openAiTokens.map(
			(t): ProviderCredential => ({
				id: t.id,
				name: t.name,
				provider: 'openai',
				token: t.token
			})
		)
	]);

	const systemPrompts = $derived((systemPromptsQuery.data ?? []) as SystemPrompt[]);
	const openRouterModels = $derived((openRouterModelsQuery.data ?? []) as OpenRouterModel[]);
	const copilotModels = $derived((copilotModelsQuery.data ?? []) as CopilotModel[]);
	const allSavedModels = $derived([
		...openRouterModels.map((m) => ({ id: m.id, modelId: m.modelId, provider: 'openrouter' })),
		...copilotModels.map((m) => ({ id: m.id, modelId: m.modelId, provider: 'github-copilot' }))
	]);

	const PROVIDER_LABELS: Record<string, string> = {
		openrouter: 'OR',
		'github-copilot': 'Copilot',
		openai: 'OpenAI'
	};

	const openRouterCredentials = $derived(credentials.filter((c) => c.provider === 'openrouter'));
	const copilotCredentials = $derived(credentials.filter((c) => c.provider === 'github-copilot'));
	const openAiCredentials = $derived(credentials.filter((c) => c.provider === 'openai'));

	let modelInput = $state($threadDefaults.model);

	$effect(() => {
		modelInput = $threadDefaults.model;
	});

	function commitModel() {
		threadDefaults.setModel(modelInput.trim());
	}
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<SlidersHorizontalIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">Thread Defaults</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						New thread behavior
					</p>
					<h2 class="text-base font-bold tracking-tight">Defaults</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						These selections are applied automatically when you open a new, empty thread. You can
						always override them in the chat header.
					</p>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Per-thread override
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Defaults only apply to new, empty threads. Existing threads remember their own last
							used settings.
						</p>
					</div>
					<div class="rounded-xl border bg-muted/30 p-4 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Stored locally
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Defaults are saved in your browser and are specific to this device.
						</p>
					</div>
				</div>
			</section>

			<div class="space-y-8">
				<!-- Credential -->
				<section class="space-y-4">
					<div class="flex items-center gap-2">
						<KeyIcon size={14} weight="fill" class="text-primary" />
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Default credential
						</h3>
					</div>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						{#if keysQuery.isPending || copilotTokensQuery.isPending || openAiTokensQuery.isPending}
							<p
								class="text-center text-[11px] font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading...
							</p>
						{:else if credentials.length === 0}
							<p class="text-center text-xs text-muted-foreground/60">
								No credentials found. Add one in
								<a
									href={resolve('/(main)/settings/providers')}
									class="font-bold text-primary underline-offset-2 hover:underline">Providers</a
								>.
							</p>
						{:else}
							<div class="space-y-2">
								<button
									class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.credentialId ===
									null
										? 'bg-primary/5 ring-1 ring-primary/20'
										: ''}"
									onclick={() => threadDefaults.setCredentialId(null)}
								>
									<span class="text-xs text-muted-foreground/60">None</span>
									{#if $threadDefaults.credentialId === null}
										<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
									{/if}
								</button>

								{#if openRouterCredentials.length > 0}
									<p
										class="px-3 pt-2 text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
									>
										OpenRouter
									</p>
									{#each openRouterCredentials as cred (cred.id)}
										<button
											class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.credentialId ===
											cred.id
												? 'bg-primary/5 ring-1 ring-primary/20'
												: ''}"
											onclick={() => threadDefaults.setCredentialId(cred.id)}
										>
											<div class="flex items-center gap-2">
												<span
													class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
												>
													OR
												</span>
												<span class="text-xs font-bold">{cred.name}</span>
												<span class="font-mono text-[9px] text-muted-foreground/50"
													>{cred.token.slice(0, 8)}••••</span
												>
											</div>
											{#if $threadDefaults.credentialId === cred.id}
												<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
											{/if}
										</button>
									{/each}
								{/if}

								{#if copilotCredentials.length > 0}
									<p
										class="px-3 pt-2 text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
									>
										GitHub Copilot
									</p>
									{#each copilotCredentials as cred (cred.id)}
										<button
											class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.credentialId ===
											cred.id
												? 'bg-primary/5 ring-1 ring-primary/20'
												: ''}"
											onclick={() => threadDefaults.setCredentialId(cred.id)}
										>
											<div class="flex items-center gap-2">
												<span
													class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
												>
													Copilot
												</span>
												<span class="text-xs font-bold">{cred.name}</span>
												<span class="font-mono text-[9px] text-muted-foreground/50"
													>{cred.token.slice(0, 8)}••••</span
												>
											</div>
											{#if $threadDefaults.credentialId === cred.id}
												<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
											{/if}
										</button>
									{/each}
								{/if}

								{#if openAiCredentials.length > 0}
									<p
										class="px-3 pt-2 text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
									>
										OpenAI
									</p>
									{#each openAiCredentials as cred (cred.id)}
										<button
											class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.credentialId ===
											cred.id
												? 'bg-primary/5 ring-1 ring-primary/20'
												: ''}"
											onclick={() => threadDefaults.setCredentialId(cred.id)}
										>
											<div class="flex items-center gap-2">
												<span
													class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
												>
													OpenAI
												</span>
												<span class="text-xs font-bold">{cred.name}</span>
												<span class="font-mono text-[9px] text-muted-foreground/50"
													>{cred.token.slice(0, 8)}••••</span
												>
											</div>
											{#if $threadDefaults.credentialId === cred.id}
												<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
											{/if}
										</button>
									{/each}
								{/if}
							</div>
						{/if}
					</div>
				</section>

				<!-- System Prompt -->
				<section class="space-y-4">
					<div class="flex items-center gap-2">
						<ChatCircleTextIcon size={14} weight="fill" class="text-primary" />
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Default system prompt
						</h3>
					</div>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						{#if systemPromptsQuery.isPending}
							<p
								class="text-center text-[11px] font-medium tracking-widest text-muted-foreground/40 uppercase"
							>
								Loading...
							</p>
						{:else}
							<div class="space-y-2">
								<button
									class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.systemPromptId ===
									null
										? 'bg-primary/5 ring-1 ring-primary/20'
										: ''}"
									onclick={() => threadDefaults.setSystemPromptId(null)}
								>
									<span class="text-xs text-muted-foreground/60">None</span>
									{#if $threadDefaults.systemPromptId === null}
										<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
									{/if}
								</button>

								{#if systemPrompts.length === 0}
									<p class="px-3 py-2 text-center text-xs text-muted-foreground/60">
										No presets saved. Create one in
										<a
											href={resolve('/(main)/settings/system-prompts')}
											class="font-bold text-primary underline-offset-2 hover:underline"
											>System Prompts</a
										>.
									</p>
								{:else}
									{#each systemPrompts as prompt (prompt.id)}
										<button
											class="flex w-full items-center justify-between rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-muted/50 {$threadDefaults.systemPromptId ===
											prompt.id
												? 'bg-primary/5 ring-1 ring-primary/20'
												: ''}"
											onclick={() => threadDefaults.setSystemPromptId(prompt.id)}
										>
											<div class="flex flex-col gap-0.5">
												<span class="text-xs font-bold">{prompt.name}</span>
												<span class="truncate text-[9px] text-muted-foreground/50"
													>{prompt.content.slice(0, 60)}{prompt.content.length > 60
														? '...'
														: ''}</span
												>
											</div>
											{#if $threadDefaults.systemPromptId === prompt.id}
												<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
											{/if}
										</button>
									{/each}
								{/if}
							</div>
						{/if}
					</div>
				</section>

				<!-- Model -->
				<section class="space-y-4">
					<div class="flex items-center gap-2">
						<CubeIcon size={14} weight="fill" class="text-primary" />
						<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Default model
						</h3>
					</div>

					<div class="rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm">
						<div class="space-y-4">
							<div class="space-y-1.5">
								<label
									class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
									for="default-model"
								>
									Model ID
								</label>
								<div class="flex gap-2">
									<Input
										id="default-model"
										bind:value={modelInput}
										placeholder="e.g. openai/gpt-4o-mini"
										class="h-8 flex-1 rounded-md bg-background/50 font-mono text-xs"
										onblur={commitModel}
									/>
									{#if modelInput.trim()}
										<Button
											variant="outline"
											size="sm"
											class="h-8 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider uppercase"
											onclick={commitModel}
										>
											<CheckIcon size={13} weight="bold" class="mr-1.5" />
											Save
										</Button>
										<Button
											variant="outline"
											size="sm"
											class="h-8 shrink-0 rounded-md px-3 text-[11px] font-bold tracking-wider text-muted-foreground uppercase"
											onclick={() => {
												modelInput = '';
												threadDefaults.setModel('');
											}}
										>
											<XIcon size={13} weight="bold" />
										</Button>
									{/if}
								</div>
								{#if $threadDefaults.model}
									<p class="text-[10px] text-muted-foreground/50">
										Current default:
										<span class="font-mono text-foreground/70">{$threadDefaults.model}</span>
									</p>
								{/if}
							</div>

							{#if openRouterModelsQuery.isPending || copilotModelsQuery.isPending}
								<p
									class="text-center text-[11px] font-medium tracking-widest text-muted-foreground/40 uppercase"
								>
									Loading saved models...
								</p>
							{:else if allSavedModels.length > 0}
								<div class="space-y-1">
									<p
										class="text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
									>
										Or pick from saved models
									</p>
									{#each allSavedModels as m (m.id)}
										<button
											class="flex w-full items-center justify-between rounded-lg px-3 py-2 text-left transition-colors hover:bg-muted/50 {$threadDefaults.model ===
											m.modelId
												? 'bg-primary/5 ring-1 ring-primary/20'
												: ''}"
											onclick={() => {
												modelInput = m.modelId;
												threadDefaults.setModel(m.modelId);
											}}
										>
											<div class="flex items-center gap-2">
												<span
													class="rounded bg-muted px-1 py-0.5 text-[8px] font-black tracking-widest text-muted-foreground/50 uppercase"
												>
													{PROVIDER_LABELS[m.provider] ?? m.provider}
												</span>
												<span class="font-mono text-xs">{m.modelId}</span>
											</div>
											{#if $threadDefaults.model === m.modelId}
												<CheckIcon size={14} weight="bold" class="shrink-0 text-primary" />
											{/if}
										</button>
									{/each}
								</div>
							{/if}
						</div>
					</div>
				</section>
			</div>
		</div>
	</div>
</div>
