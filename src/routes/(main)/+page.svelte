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
	import { cn } from '$lib/utils';
	import { MOCK_CHATS } from '$lib/mock-data';
	import { untrack } from 'svelte';

	let activeChatId = $state(MOCK_CHATS[0].id);
	let activeChat = $derived(MOCK_CHATS.find((c) => c.id === activeChatId) || MOCK_CHATS[0]);
	let messages = $derived(activeChat.messages);

	let viewportRef: HTMLElement | null = $state(null);

	function scrollToMessage(id: string) {
		const element = document.getElementById(id);
		if (element && viewportRef) {
			element.scrollIntoView({ behavior: 'smooth', block: 'center' });
		}
	}

	function setActiveChat(id: string) {
		activeChatId = id;
		// Reset scroll when switching chats
		if (viewportRef) {
			untrack(() => {
				viewportRef!.scrollTo({ top: 0 });
			});
		}
	}
</script>

<div class="flex h-[calc(100vh-2rem)] w-full overflow-hidden bg-background">
	<!-- Left Sidebar: Recent Chats -->
	<aside class="flex w-64 flex-col border-r bg-muted/30 backdrop-blur-md">
		<div class="flex items-center justify-between p-4">
			<h2 class="text-[10px] font-black tracking-widest text-foreground/40 uppercase">
				Recent Slops
			</h2>
			<Button
				variant="ghost"
				size="icon-xs"
				class="rounded-full hover:bg-primary/10 hover:text-primary"
			>
				<PlusIcon size={14} />
			</Button>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-1 p-2">
				{#each MOCK_CHATS as chat (chat.id)}
					<button
						onclick={() => setActiveChat(chat.id)}
						class={cn(
							'group flex w-full flex-col gap-1 rounded-xl px-3 py-2.5 text-left transition-all',
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
	<main class="flex flex-1 flex-col bg-background/50 backdrop-blur-sm">
		<header class="flex h-12 items-center border-b bg-background/20 px-6 backdrop-blur-xl">
			<div class="flex items-center gap-2">
				<div
					class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary"
				>
					<ChatCircleIcon size={14} weight="fill" />
				</div>
				<h1 class="text-sm font-bold tracking-tight">{activeChat.title}</h1>
			</div>
		</header>

		<ScrollArea.Root class="flex-1" bind:viewportRef>
			<div class="mx-auto max-w-3xl space-y-10 px-6 py-10">
				{#each messages as message (message.id)}
					<div
						id={message.id}
						class={cn(
							'flex w-full animate-in gap-5 transition-all duration-500 fade-in slide-in-from-bottom-2',
							message.role === 'user' ? 'flex-row-reverse' : 'flex-row'
						)}
					>
						<Avatar.Root class="mt-1 h-9 w-9 shrink-0 shadow-sm ring-2 ring-background">
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
										: 'bg-background/80 ring-border backdrop-blur-md'
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
		</ScrollArea.Root>

		<footer class="p-6">
			<div
				class="mx-auto flex max-w-3xl items-center gap-3 rounded-[20px] bg-muted/40 p-2.5 shadow-inner ring-1 ring-border/50 transition-all focus-within:bg-background/60 focus-within:ring-primary/30"
			>
				<Input
					placeholder="Message Slopify..."
					class="h-9 border-0 bg-transparent px-3 text-sm placeholder:text-muted-foreground/40 focus-visible:ring-0 focus-visible:ring-offset-0"
				/>
				<Button
					size="icon-sm"
					variant="default"
					class="h-9 w-9 rounded-[14px] shadow-lg shadow-primary/20 transition-transform active:scale-95"
				>
					<PaperPlaneRightIcon size={18} weight="bold" />
				</Button>
			</div>
			<p
				class="mt-3 text-center text-[10px] font-medium tracking-widest text-muted-foreground/30 uppercase"
			>
				Slopify can make mistakes. Always double cleanse your code.
			</p>
		</footer>
	</main>

	<!-- Right Sidebar: TOC/Message List -->
	<aside class="hidden w-60 flex-col border-l bg-muted/20 backdrop-blur-md lg:flex">
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
