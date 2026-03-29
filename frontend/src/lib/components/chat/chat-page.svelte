<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import {
		ChatCircleIcon,
		KeyIcon,
		PaperPlaneRightIcon,
		PlusIcon,
		RobotIcon,
		UserIcon
	} from 'phosphor-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { threadKeys, threadsQueryOptions } from '$lib/queries/thread-query';
	import { openRouterKeysQueryOptions } from '$lib/queries/openrouter-key-query';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { createThread, loadMessagesByThread, saveMessagesByThread } from '$lib/thread-client';
	import type { AuthUser, Message, OpenRouterApiKey, Thread } from '$lib/types';
	import { cn } from '$lib/utils';
	import { onMount, tick } from 'svelte';

	type MessagesByThread = Record<string, Message[]>;

	interface Props {
		threadId: string;
	}

	const BACKEND_CHAT_ENDPOINT = '/api/v1/chat/completions';
	const DEFAULT_MODEL = 'openai/gpt-4o-mini';
	const DEFAULT_THREAD_TITLE = 'New thread';

	let { threadId }: Props = $props();
	const queryClient = useQueryClient();

	let messagesByThread = $state<MessagesByThread>({});
	let draft = $state('');
	let model = $state(DEFAULT_MODEL);
	let isSending = $state(false);
	let hasRequestedInitialThread = $state(false);

	let viewportRef: HTMLElement | null = $state(null);
	let currentUser = $derived(page.data.user as AuthUser);

	const threadsQuery = createQuery(() => threadsQueryOptions());
	const keysQuery = createQuery(() => openRouterKeysQueryOptions());

	const keys = $derived((keysQuery.data ?? []) as OpenRouterApiKey[]);
	let selectedKeyId = $state<string | null>(null);
	const selectedKey = $derived(keys.find((k) => k.id === selectedKeyId) ?? keys[0] ?? null);

	const createThreadMutation = createMutation(() => ({
		mutationFn: ({ title }: { title?: string; replaceState?: boolean }) => createThread(title),
		onSuccess: async (newThread, variables) => {
			queryClient.setQueryData<Thread[]>(threadKeys.all, (currentThreads) => [
				newThread,
				...(currentThreads ?? [])
			]);
			updateThreadMessages(newThread.id, []);
			draft = '';
			await gotoThread(newThread.id, variables.replaceState ?? false);
		}
	}));

	let threads = $derived((threadsQuery.data ?? []) as Thread[]);
	let activeThread = $derived(threads.find((thread) => thread.id === threadId) ?? null);
	let messages = $derived(messagesByThread[threadId] ?? []);
	let threadTitle = $derived(activeThread ? getThreadTitle(activeThread, messages) : 'Thread');
	let messageCount = $derived(messages.length);
	let isCreatingThread = $derived(createThreadMutation.isPending);
	let isBootstrapping = $derived(
		threadsQuery.isPending || (threads.length === 0 && isCreatingThread)
	);
	let loadError = $derived.by(() => {
		const queryError = threadsQuery.error;
		if (queryError instanceof Error) {
			return queryError.message;
		}

		const mutationError = createThreadMutation.error;
		if (mutationError instanceof Error) {
			return mutationError.message;
		}

		return '';
	});
	let chatThreads = $derived(
		threads.map((thread) => {
			const threadMessages = messagesByThread[thread.id] ?? [];

			return {
				...thread,
				title: getThreadTitle(thread, threadMessages),
				lastMessage: threadMessages.at(-1)?.content ?? 'No messages yet',
				messages: threadMessages
			};
		})
	);

	onMount(() => {
		messagesByThread = loadMessagesByThread(currentUser.id);
	});

	$effect(() => {
		if (
			threadsQuery.isSuccess &&
			threads.length === 0 &&
			!hasRequestedInitialThread &&
			!isCreatingThread
		) {
			hasRequestedInitialThread = true;
			createThreadMutation.mutate({ replaceState: true });
		}
	});

	$effect(() => {
		if (!isBootstrapping && threads.length > 0 && !activeThread) {
			void gotoThread(threads[0].id, true);
		}
	});

	function scrollToLatest(behavior: ScrollBehavior = 'auto') {
		if (!viewportRef) {
			return;
		}

		viewportRef.scrollTo({
			top: viewportRef.scrollHeight,
			behavior
		});
	}

	function scrollToMessage(id: string) {
		const element = document.getElementById(id);
		if (element && viewportRef) {
			element.scrollIntoView({ behavior: 'smooth', block: 'center' });
		}
	}

	function createMessage(role: Message['role'], content: string): Message {
		return {
			id: crypto.randomUUID(),
			role,
			content,
			timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
		};
	}

	function truncateText(value: string, maxLength: number) {
		if (value.length <= maxLength) {
			return value;
		}

		return `${value.slice(0, maxLength - 3).trimEnd()}...`;
	}

	function getThreadTitle(thread: Thread, threadMessages: Message[]) {
		if (thread.title !== DEFAULT_THREAD_TITLE) {
			return thread.title;
		}

		const firstUserMessage = threadMessages.find((message) => message.role === 'user');
		return firstUserMessage ? truncateText(firstUserMessage.content, 36) : thread.title;
	}

	function updateThreadMessages(targetThreadId: string, nextMessages: Message[]) {
		const nextMessagesByThread = {
			...messagesByThread,
			[targetThreadId]: nextMessages
		};

		messagesByThread = nextMessagesByThread;
		saveMessagesByThread(currentUser.id, nextMessagesByThread);
	}

	async function gotoThread(id: string, replaceState = false) {
		await goto(resolve(`/thread/${id}`), { replaceState });
	}

	async function handleCreateThread() {
		if (isCreatingThread || threadsQuery.isPending) {
			return;
		}

		createThreadMutation.mutate({ replaceState: false });
	}

	async function sendMessage() {
		if (!activeThread) {
			return;
		}

		const requestThreadId = activeThread.id;
		const prompt = draft.trim();
		const trimmedApiKey = selectedKey?.apiKey.trim() ?? '';
		const selectedModel = model.trim();

		if (!prompt || !trimmedApiKey || !selectedModel || isSending) {
			return;
		}

		const nextMessages = [...messages, createMessage('user', prompt)];
		updateThreadMessages(requestThreadId, nextMessages);
		draft = '';
		isSending = true;

		try {
			const response = await fetch(BACKEND_CHAT_ENDPOINT, {
				method: 'POST',
				headers: {
					authorization: `Bearer ${trimmedApiKey}`,
					'content-type': 'application/json'
				},
				body: JSON.stringify({ prompt, model: selectedModel })
			});

			const payload = (await response.json()) as { content?: string; error?: string };

			if (!response.ok || !payload.content) {
				throw new Error(payload.error ?? 'The model returned an empty response.');
			}

			updateThreadMessages(requestThreadId, [
				...nextMessages,
				createMessage('assistant', payload.content)
			]);
		} catch (error) {
			const message = error instanceof Error ? error.message : 'Failed to send prompt.';
			updateThreadMessages(requestThreadId, [
				...nextMessages,
				createMessage('assistant', `Error: ${message}`)
			]);
		} finally {
			isSending = false;
		}
	}

	function handleComposerKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			void sendMessage();
		}
	}

	$effect(() => {
		const currentThreadId = threadId;
		const currentMessageCount = messageCount;

		void tick().then(() => {
			if (!currentThreadId || currentMessageCount < 0) {
				return;
			}

			scrollToLatest();
		});
	});
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full overflow-hidden bg-background">
	<aside class="flex min-h-0 w-64 flex-col border-r bg-muted/30 backdrop-blur-md">
		<div class="flex items-center justify-between p-4">
			<h2 class="text-[10px] font-black text-foreground/40 uppercase">Recent</h2>
			<Button
				variant="ghost"
				size="icon-xs"
				class="rounded-full hover:bg-primary/10 hover:text-primary"
				disabled={isCreatingThread}
				onclick={handleCreateThread}
			>
				<PlusIcon size={14} />
			</Button>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-2 p-2">
				{#each chatThreads as chat (chat.id)}
					<button
						onclick={() => void gotoThread(chat.id)}
						class={cn(
							'group flex w-full flex-col gap-1 rounded-xl px-3 py-1.5 text-left transition-all',
							threadId === chat.id
								? 'bg-background shadow-sm ring-1 shadow-primary/5 ring-border'
								: 'hover:bg-muted/80'
						)}
					>
						<div class="flex items-center justify-between">
							<span
								class={cn(
									'truncate text-sm font-semibold transition-colors',
									threadId === chat.id
										? 'text-primary'
										: 'text-foreground/80 group-hover:text-foreground'
								)}>{chat.title}</span
							>
						</div>
						<p class="line-clamp-1 text-[11px] text-muted-foreground/70">{chat.lastMessage}</p>
					</button>
				{/each}
			</div>
		</ScrollArea.Root>
	</aside>

	<main class="flex min-h-0 flex-1 flex-col bg-background/50 backdrop-blur-sm">
		<header class="flex items-center border-b bg-background/20 px-6 py-1 backdrop-blur-xl">
			<div class="flex items-center gap-2">
				<div
					class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary"
				>
					<ChatCircleIcon size={14} weight="fill" />
				</div>
				<h1 class="text-sm font-bold tracking-tight">{threadTitle}</h1>
			</div>
		</header>

		<ScrollArea.Root class="min-h-0 flex-1" bind:viewportRef>
			<div class="flex min-h-full flex-col justify-end">
				{#if loadError}
					<div class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-6 py-10">
						<p
							class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
						>
							{loadError}
						</p>
					</div>
				{:else if isBootstrapping}
					<div class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-6 py-10">
						<p class="text-sm text-muted-foreground">Loading threads...</p>
					</div>
				{:else if messages.length === 0}
					<div class="mx-auto flex w-full max-w-3xl flex-1 items-center justify-center px-6 py-10">
						<div
							class="max-w-sm rounded-3xl border bg-background/80 px-6 py-8 text-center shadow-sm"
						>
							<h2 class="text-base font-semibold">Empty thread</h2>
							<p class="mt-2 text-sm text-muted-foreground">
								Send the first message to start this chat.
							</p>
						</div>
					</div>
				{:else}
					<div class="mx-auto w-full max-w-3xl space-y-10 px-6 py-10">
						{#each messages as message (message.id)}
							<div
								id={message.id}
								class={cn(
									'flex w-full animate-in gap-5 transition-all duration-500 fade-in slide-in-from-bottom-2',
									message.role === 'user' ? 'flex-row-reverse' : 'flex-row'
								)}
							>
								<Avatar.Root
									title={message.role === 'assistant' ? 'Clanker' : 'Human'}
									class="mt-1 h-9 w-9 shrink-0 shadow-sm ring-2 ring-background"
								>
									<Avatar.Fallback
										class={message.role === 'assistant'
											? 'border border-primary/20 bg-primary/10 text-primary'
											: 'border border-border bg-secondary text-secondary-foreground'}
									>
										{#if message.role === 'assistant'}
											<RobotIcon size={18} />
										{:else}
											<UserIcon size={18} />
										{/if}
									</Avatar.Fallback>
								</Avatar.Root>
								<div
									class={cn(
										'flex max-w-[85%] flex-col gap-2.5',
										message.role === 'user' ? 'items-end' : 'items-start'
									)}
								>
									<div
										class={cn(
											'rounded-2xl px-4 py-3 text-sm leading-relaxed shadow-[0_2px_10px_-3px_rgba(0,0,0,0.07)] ring-1',
											message.role === 'user'
												? 'bg-primary text-primary-foreground ring-primary/20'
												: 'bg-background/80 font-mono text-xs ring-border backdrop-blur-md'
										)}
									>
										{message.content}
									</div>
									<span
										class="px-1 text-[9px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase"
									>
										{message.timestamp}
									</span>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</ScrollArea.Root>

		<footer class="p-6">
			<div class="mx-auto mb-2 flex max-w-3xl flex-wrap items-center gap-2 px-2">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger
						class="flex h-8 min-w-0 flex-1 items-center justify-between gap-2 rounded-md border border-border/60 bg-background/70 px-3 text-left text-xs transition-all hover:bg-background/90 disabled:opacity-50"
						disabled={isSending || isBootstrapping || !activeThread}
					>
						<div class="flex items-center gap-2 truncate">
							<KeyIcon
								size={12}
								weight={selectedKey ? 'fill' : 'regular'}
								class={selectedKey ? 'text-primary' : 'text-muted-foreground/40'}
							/>
							<span class={selectedKey ? 'text-foreground' : 'text-muted-foreground/40'}>
								{selectedKey ? selectedKey.name : 'No keys saved'}
							</span>
						</div>
						<span class="text-[9px] font-black tracking-widest text-muted-foreground/30 uppercase"
							>key</span
						>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="start" class="w-56 rounded-xl shadow-xl">
						<DropdownMenu.Label
							class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase"
							>Select provider key</DropdownMenu.Label
						>
						<DropdownMenu.Separator />
						{#if keys.length === 0}
							<div class="px-2 py-3 text-center">
								<p class="text-[11px] text-muted-foreground/60">No keys found</p>
								<Button
									variant="link"
									class="mt-1 h-auto p-0 text-[10px] font-bold tracking-widest uppercase"
									onclick={() => goto(resolve('/(main)/settings/keys'))}
								>
									Add one in settings
								</Button>
							</div>
						{:else}
							{#each keys as key (key.id)}
								<DropdownMenu.Item
									class="flex items-center justify-between rounded-lg py-2"
									onclick={() => (selectedKeyId = key.id)}
								>
									<div class="flex flex-col gap-0.5">
										<span class="text-xs font-bold">{key.name}</span>
										<span class="font-mono text-[9px] text-muted-foreground/50"
											>{key.apiKey.slice(0, 8)}••••</span
										>
									</div>
									{#if selectedKey?.id === key.id}
										<div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
									{/if}
								</DropdownMenu.Item>
							{/each}
						{/if}
					</DropdownMenu.Content>
				</DropdownMenu.Root>

				<Input
					bind:value={model}
					placeholder="openai/gpt-4o-mini"
					class="h-8 max-w-56 border-border/60 bg-background/70 text-xs"
					disabled={isSending || isBootstrapping || !activeThread}
				/>
				<span class="text-xs font-bold tracking-[0.15em] text-muted-foreground/40 uppercase"
					>model</span
				>
			</div>
			<div
				class="mx-auto flex max-w-3xl items-center gap-3 rounded-[20px] bg-muted/40 p-2.5 shadow-inner ring-1 ring-border/50 transition-all focus-within:bg-background/60 focus-within:ring-primary/30"
			>
				<Input
					bind:value={draft}
					placeholder="Message Slopify..."
					class="h-9 border-0 bg-transparent px-3 text-sm placeholder:text-muted-foreground/40 focus-visible:ring-0 focus-visible:ring-offset-0"
					disabled={isSending || isBootstrapping || !activeThread}
					onkeydown={handleComposerKeydown}
				/>
				<Button
					size="icon-sm"
					variant="default"
					class="h-9 w-9 rounded-[14px] shadow-lg shadow-primary/20 transition-transform active:scale-95"
					disabled={isSending ||
						isBootstrapping ||
						!activeThread ||
						!draft.trim() ||
						!selectedKey ||
						!model.trim()}
					onclick={sendMessage}
				>
					<PaperPlaneRightIcon size={18} weight="bold" />
				</Button>
			</div>
			<p
				class="mt-3 text-center text-[10px] font-medium tracking-widest text-muted-foreground/30 uppercase"
			>
				You know by now the clanker hallucinates... like a lot. Double check.
			</p>
		</footer>
	</main>

	<aside class="hidden min-h-0 w-60 flex-col border-l bg-muted/20 backdrop-blur-md lg:flex">
		<div class="p-4">
			<h2 class="text-[10px] font-black tracking-widest text-foreground/40 uppercase">
				Message Log
			</h2>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-4 p-4">
				{#if messages.length === 0}
					<p class="text-[11px] text-muted-foreground/60">No messages yet.</p>
				{:else}
					{#each messages as message (`toc-${message.id}`)}
						<button
							onclick={() => scrollToMessage(message.id)}
							class="group flex w-full items-start gap-2.5 text-left transition-all"
						>
							<div
								class={cn(
									'mt-1.5 flex h-1.5 w-1.5 shrink-0 rounded-full transition-all group-hover:scale-125',
									message.role === 'assistant' ? 'bg-primary' : 'bg-muted-foreground/30'
								)}
							></div>
							<div class="flex flex-col gap-1">
								<span
									class="text-[10px] font-black tracking-tighter text-muted-foreground/50 uppercase transition-colors group-hover:text-primary/60"
								>
									{message.role}
								</span>
								<p
									class="line-clamp-2 text-[11px] leading-snug text-foreground/60 transition-colors group-hover:text-foreground"
								>
									{message.content}
								</p>
							</div>
						</button>
					{/each}
				{/if}
			</div>
		</ScrollArea.Root>
	</aside>
</div>
