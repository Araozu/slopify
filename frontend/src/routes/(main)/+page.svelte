<script lang="ts">
	import {
		ChatCircleIcon,
		PaperPlaneRightIcon,
		PlusIcon,
		RobotIcon,
		UserIcon
	} from 'phosphor-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { MOCK_CHATS } from '$lib/mock-data';
	import type { ChatThread, Message } from '$lib/types';
	import { cn } from '$lib/utils';
	import { tick } from 'svelte';

	const BACKEND_CHAT_ENDPOINT = '/api/v1/chat/completions';
	const DEFAULT_MODEL = 'openai/gpt-4o-mini';
	const INITIAL_CHAT_ID = MOCK_CHATS[0].id;

	let chats = $state(structuredClone(MOCK_CHATS) as ChatThread[]);
	let activeChatId = $state(INITIAL_CHAT_ID);
	let draft = $state('');
	let apiKey = $state('');
	let model = $state(DEFAULT_MODEL);
	let isSending = $state(false);
	let activeChat = $derived(chats.find((chat) => chat.id === activeChatId) ?? chats[0]);
	let messages = $derived(activeChat.messages);
	let messageCount = $derived(messages.length);

	let viewportRef: HTMLElement | null = $state(null);

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

	function setActiveChat(id: string) {
		activeChatId = id;
	}

	function createMessage(role: Message['role'], content: string): Message {
		return {
			id: crypto.randomUUID(),
			role,
			content,
			timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
		};
	}

	function updateActiveChat(chatId: string, nextMessages: Message[]) {
		chats = chats.map((chat) =>
			chat.id === chatId
				? {
						...chat,
						messages: nextMessages,
						lastMessage: nextMessages.at(-1)?.content ?? chat.lastMessage
					}
				: chat
		);
	}

	async function sendMessage() {
		const chatId = activeChatId;
		const prompt = draft.trim();
		const trimmedApiKey = apiKey.trim();
		const selectedModel = model.trim();

		if (!prompt || !trimmedApiKey || !selectedModel || isSending) {
			return;
		}

		const nextMessages = [...messages, createMessage('user', prompt)];
		updateActiveChat(chatId, nextMessages);
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

			updateActiveChat(chatId, [...nextMessages, createMessage('assistant', payload.content)]);
		} catch (error) {
			const message = error instanceof Error ? error.message : 'Failed to send prompt.';
			updateActiveChat(chatId, [...nextMessages, createMessage('assistant', `Error: ${message}`)]);
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
		const currentChatId = activeChatId;
		const currentMessageCount = messageCount;

		void tick().then(() => {
			if (!currentChatId || currentMessageCount < 0) {
				return;
			}

			scrollToLatest();
		});
	});
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full overflow-hidden bg-background">
	<!-- Left Sidebar: Recent Chats -->
	<aside class="flex min-h-0 w-64 flex-col border-r bg-muted/30 backdrop-blur-md">
		<div class="flex items-center justify-between p-4">
			<h2 class="text-[10px] font-black text-foreground/40 uppercase">Recent</h2>
			<Button
				variant="ghost"
				size="icon-xs"
				class="rounded-full hover:bg-primary/10 hover:text-primary"
			>
				<PlusIcon size={14} />
			</Button>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-2 p-2">
				{#each chats as chat (chat.id)}
					<button
						onclick={() => setActiveChat(chat.id)}
						class={cn(
							'group flex w-full flex-col gap-1 rounded-xl px-3 py-1.5 text-left transition-all',
							activeChatId === chat.id
								? 'bg-background shadow-sm ring-1 shadow-primary/5 ring-border'
								: 'hover:bg-muted/80'
						)}
					>
						<div class="flex items-center justify-between">
							<span
								class={cn(
									'truncate text-sm font-semibold transition-colors',
									activeChatId === chat.id
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

	<!-- Center: Chat Messages -->
	<main class="flex min-h-0 flex-1 flex-col bg-background/50 backdrop-blur-sm">
		<header class="flex items-center border-b bg-background/20 px-6 py-1 backdrop-blur-xl">
			<div class="flex items-center gap-2">
				<div
					class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary"
				>
					<ChatCircleIcon size={14} weight="fill" />
				</div>
				<h1 class="text-sm font-bold tracking-tight">{activeChat.title}</h1>
			</div>
		</header>

		<ScrollArea.Root class="min-h-0 flex-1" bind:viewportRef>
			<div class="flex min-h-full flex-col justify-end">
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
			</div>
		</ScrollArea.Root>

		<footer class="p-6">
			<div class="mx-auto mb-2 flex max-w-3xl flex-wrap items-center gap-2 px-2">
				<Input
					bind:value={apiKey}
					type="password"
					placeholder="OpenRouter API key"
					class="h-8 min-w-0 flex-1 border-border/60 bg-background/70 text-xs"
					disabled={isSending}
				/>
				<span class="text-[10px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase"
					>api key</span
				>
				<Input
					bind:value={model}
					placeholder="openai/gpt-4o-mini"
					class="h-8 max-w-56 border-border/60 bg-background/70 text-xs"
					disabled={isSending}
				/>
				<span class="text-[10px] font-bold tracking-[0.15em] text-muted-foreground/40 uppercase"
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
					disabled={isSending}
					onkeydown={handleComposerKeydown}
				/>
				<Button
					size="icon-sm"
					variant="default"
					class="h-9 w-9 rounded-[14px] shadow-lg shadow-primary/20 transition-transform active:scale-95"
					disabled={isSending || !draft.trim() || !apiKey.trim() || !model.trim()}
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

	<!-- Right Sidebar: TOC/Message List -->
	<aside class="hidden min-h-0 w-60 flex-col border-l bg-muted/20 backdrop-blur-md lg:flex">
		<div class="p-4">
			<h2 class="text-[10px] font-black tracking-widest text-foreground/40 uppercase">
				Message Log
			</h2>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-4 p-4">
				{#each messages as message (`toc-${message.id}`)}
					<button
						onclick={() => scrollToMessage(message.id)}
						class="group flex w-full items-start gap-2.5 text-left transition-all"
					>
						<div
							class="mt-1.5 flex h-1.5 w-1.5 shrink-0 rounded-full transition-all group-hover:scale-125 {message.role ===
							'assistant'
								? 'bg-primary'
								: 'bg-muted-foreground/30'}"
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
			</div>
		</ScrollArea.Root>
	</aside>
</div>
