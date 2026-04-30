<script lang="ts">
	import { resolve } from '$app/paths';
	import { KeyIcon, CubeIcon, GithubLogoIcon, ArrowRightIcon, SparkleIcon } from 'phosphor-svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import { openRouterKeysQueryOptions } from '$lib/queries/openrouter-key-query';
	import { copilotTokensQueryOptions } from '$lib/queries/copilot-token-query';
	import { openAiTokensQueryOptions } from '$lib/queries/openai-token-query';
	import { zenKeysQueryOptions } from '$lib/queries/zen-key-query';
	import type { OpenRouterApiKey, CopilotToken, OpenAiToken, ZenApiKey } from '$lib/types';

	const keysQuery = createQuery(() => openRouterKeysQueryOptions());
	const copilotQuery = createQuery(() => copilotTokensQueryOptions());
	const openAiQuery = createQuery(() => openAiTokensQueryOptions());
	const zenQuery = createQuery(() => zenKeysQueryOptions());

	const openRouterKeys = $derived((keysQuery.data ?? []) as OpenRouterApiKey[]);
	const copilotTokens = $derived((copilotQuery.data ?? []) as CopilotToken[]);
	const openAiTokens = $derived((openAiQuery.data ?? []) as OpenAiToken[]);
	const zenKeys = $derived((zenQuery.data ?? []) as ZenApiKey[]);
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-4 py-1 backdrop-blur-xl md:px-6">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<CubeIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">Providers</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-10 px-4 py-6 md:px-6 md:py-10">
			<section class="space-y-4">
				<div class="space-y-1">
					<p class="text-[10px] font-black tracking-widest text-muted-foreground/50 uppercase">
						Provider hub
					</p>
					<h2 class="text-base font-bold tracking-tight">LLM Providers</h2>
					<p class="max-w-2xl text-xs text-muted-foreground/60">
						Manage your API keys and tokens for each provider. Select a provider to configure
						credentials and saved models.
					</p>
				</div>
			</section>

			<div class="grid gap-4 sm:grid-cols-2">
				<a
					href={resolve('/(main)/settings/keys')}
					class="group rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm transition-all hover:border-primary/30 hover:bg-card/70"
				>
					<div class="flex items-start justify-between">
						<div
							class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/60 transition-colors group-hover:bg-primary/10 group-hover:text-primary"
						>
							<KeyIcon size={20} weight="fill" />
						</div>
						<ArrowRightIcon
							size={14}
							class="text-muted-foreground/0 transition-all group-hover:text-muted-foreground/40"
						/>
					</div>
					<h3 class="mt-4 text-sm font-bold tracking-tight">OpenRouter</h3>
					<p class="mt-1 text-xs text-muted-foreground/60">
						Access hundreds of models through a single API key.
					</p>
					<div class="mt-4 flex items-center gap-3">
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{openRouterKeys.length} key{openRouterKeys.length !== 1 ? 's' : ''}
						</span>
						<span class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Keys & Models
						</span>
					</div>
				</a>

				<a
					href={resolve('/(main)/settings/providers/copilot')}
					class="group rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm transition-all hover:border-primary/30 hover:bg-card/70"
				>
					<div class="flex items-start justify-between">
						<div
							class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/60 transition-colors group-hover:bg-primary/10 group-hover:text-primary"
						>
							<GithubLogoIcon size={20} weight="fill" />
						</div>
						<ArrowRightIcon
							size={14}
							class="text-muted-foreground/0 transition-all group-hover:text-muted-foreground/40"
						/>
					</div>
					<h3 class="mt-4 text-sm font-bold tracking-tight">GitHub Copilot</h3>
					<p class="mt-1 text-xs text-muted-foreground/60">
						Use your GitHub Copilot subscription with a personal access token.
					</p>
					<div class="mt-4">
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{copilotTokens.length} token{copilotTokens.length !== 1 ? 's' : ''}
						</span>
					</div>
				</a>

				<a
					href={resolve('/(main)/settings/providers/openai')}
					class="group rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm transition-all hover:border-primary/30 hover:bg-card/70"
				>
					<div class="flex items-start justify-between">
						<div
							class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/60 transition-colors group-hover:bg-primary/10 group-hover:text-primary"
						>
							<SparkleIcon size={20} weight="fill" />
						</div>
						<ArrowRightIcon
							size={14}
							class="text-muted-foreground/0 transition-all group-hover:text-muted-foreground/40"
						/>
					</div>
					<h3 class="mt-4 text-sm font-bold tracking-tight">OpenAI</h3>
					<p class="mt-1 text-xs text-muted-foreground/60">
						Connect with an API key or use OpenAI device sign-in.
					</p>
					<div class="mt-4">
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{openAiTokens.length} token{openAiTokens.length !== 1 ? 's' : ''}
						</span>
					</div>
				</a>

				<a
					href={resolve('/(main)/settings/providers/zen')}
					class="group rounded-xl border bg-card/50 p-5 shadow-sm backdrop-blur-sm transition-all hover:border-primary/30 hover:bg-card/70"
				>
					<div class="flex items-start justify-between">
						<div
							class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/5 text-primary/60 transition-colors group-hover:bg-primary/10 group-hover:text-primary"
						>
							<SparkleIcon size={20} weight="fill" />
						</div>
						<ArrowRightIcon
							size={14}
							class="text-muted-foreground/0 transition-all group-hover:text-muted-foreground/40"
						/>
					</div>
					<h3 class="mt-4 text-sm font-bold tracking-tight">OpenCode Zen</h3>
					<p class="mt-1 text-xs text-muted-foreground/60">
						Multi-model gateway: Claude, Qwen, Kimi, GLM, MiniMax and more.
					</p>
					<div class="mt-4">
						<span
							class="rounded-full bg-muted/50 px-2.5 py-1 text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase ring-1 ring-border/50"
						>
							{zenKeys.length} key{zenKeys.length !== 1 ? 's' : ''}
						</span>
					</div>
				</a>
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
