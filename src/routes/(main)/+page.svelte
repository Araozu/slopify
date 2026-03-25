<script lang="ts">
	import {
		ChatCircleIcon,
		PaperPlaneRightIcon,
		CaretRightIcon,
		PlusIcon,
		RobotIcon,
		UserIcon
	} from 'phosphor-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { cn } from '$lib/utils';

	type Message = {
		id: string;
		role: 'user' | 'assistant';
		content: string;
		timestamp: string;
	};

	type Chat = {
		id: string;
		title: string;
		lastMessage: string;
		active?: boolean;
	};

	const chats: Chat[] = [
		{
			id: '1',
			title: 'The skincare routine',
			lastMessage: 'The double cleanse is key!',
			active: true
		},
		{
			id: '2',
			title: 'TS vs JS debate',
			lastMessage: 'Types are just better for the soul.',
			active: false
		},
		{
			id: '3',
			title: 'Latte art tips',
			lastMessage: 'Wait, is it oat milk or almond?',
			active: false
		}
	];

	const messages: Message[] = [
		{
			id: 'm1',
			role: 'user',
			content: "I'm looking for a way to make my code as glowy as my skin. Any tips?",
			timestamp: '10:00 AM'
		},
		{
			id: 'm2',
			role: 'assistant',
			content:
				"Bestie, it's all about that *refactor serum*. Clean imports, type safety, and absolutely NO inline hex codes. That's how you get that natural glow! ✨",
			timestamp: '10:01 AM'
		},
		{
			id: 'm3',
			role: 'user',
			content: 'Omg, thank you! What about the sidebar though? It feels a bit dry.',
			timestamp: '10:02 AM'
		},
		{
			id: 'm4',
			role: 'assistant',
			content:
				"We're adding that glassmorphism hydration right now. Think backdrop-blur-xl and some juicy Phosphor icons. It's going to be a total look. 💅",
			timestamp: '10:03 AM'
		}
	];
</script>

<div class="flex h-[calc(100vh-2rem)] w-full overflow-hidden bg-background">
	<!-- Left Sidebar: Recent Chats -->
	<aside class="flex w-64 flex-col border-r bg-muted/30 backdrop-blur-md">
		<div class="flex items-center justify-between p-4">
			<h2 class="text-sm font-bold tracking-tight text-foreground/70 uppercase">Recent Slops</h2>
			<Button variant="ghost" size="icon-xs" class="rounded-full">
				<PlusIcon size={14} />
			</Button>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-1 p-2">
				{#each chats as chat (chat.id)}
					<button
						class={cn(
							'flex w-full flex-col gap-1 rounded-lg px-3 py-2 text-left transition-all hover:bg-muted/80',
							chat.active && 'bg-muted shadow-sm ring-1 ring-border'
						)}
					>
						<div class="flex items-center justify-between">
							<span class="truncate text-sm font-semibold">{chat.title}</span>
						</div>
						<p class="line-clamp-1 text-xs text-muted-foreground">{chat.lastMessage}</p>
					</button>
				{/each}
			</div>
		</ScrollArea.Root>
	</aside>

	<!-- Center: Chat Messages -->
	<main class="flex flex-1 flex-col bg-background/50 backdrop-blur-sm">
		<header class="flex h-12 items-center border-b px-6">
			<div class="flex items-center gap-2">
				<ChatCircleIcon size={18} class="text-primary" weight="fill" />
				<h1 class="text-sm font-bold">The skincare routine</h1>
			</div>
		</header>

		<ScrollArea.Root class="flex-1 px-4 py-6">
			<div class="mx-auto max-w-3xl space-y-8">
				{#each messages as message (message.id)}
					<div
						class={cn(
							'flex w-full gap-4 px-2 py-1',
							message.role === 'user' ? 'flex-row-reverse' : 'flex-row'
						)}
					>
						<Avatar.Root class="mt-1 h-8 w-8 shrink-0">
							<Avatar.Fallback class={message.role === 'assistant' ? 'bg-primary/10' : 'bg-muted'}>
								{#if message.role === 'assistant'}
									<RobotIcon size={16} class="text-primary" />
								{:else}
									<UserIcon size={16} />
								{/if}
							</Avatar.Fallback>
						</Avatar.Root>
						<div
							class={cn(
								'flex flex-col gap-2',
								message.role === 'user' ? 'items-end' : 'items-start'
							)}
						>
							<div
								class={cn(
									'rounded-2xl px-4 py-2 text-sm leading-relaxed shadow-sm ring-1',
									message.role === 'user'
										? 'bg-primary text-primary-foreground ring-primary/20'
										: 'bg-muted/50 ring-border'
								)}
							>
								{message.content}
							</div>
							<span
								class="text-[10px] font-medium tracking-widest text-muted-foreground/60 uppercase"
							>
								{message.timestamp}
							</span>
						</div>
					</div>
				{/each}
			</div>
		</ScrollArea.Root>

		<footer class="p-4">
			<div
				class="mx-auto flex max-w-3xl items-center gap-2 rounded-2xl bg-muted/50 p-2 ring-1 ring-border transition-all focus-within:ring-primary/40"
			>
				<Input
					placeholder="Message Slopify..."
					class="border-0 bg-transparent focus-visible:ring-0 focus-visible:ring-offset-0"
				/>
				<Button size="icon-sm" variant="default" class="rounded-xl shadow-md">
					<PaperPlaneRightIcon size={16} weight="bold" />
				</Button>
			</div>
			<p class="mt-2 text-center text-[10px] text-muted-foreground/50">
				Slopify can make mistakes. Always double cleanse your code.
			</p>
		</footer>
	</main>

	<!-- Right Sidebar: TOC/Message List -->
	<aside class="hidden w-56 flex-col border-l bg-muted/20 backdrop-blur-md lg:flex">
		<div class="p-4">
			<h2 class="text-sm font-bold tracking-tight text-foreground/70 uppercase">Message Log</h2>
		</div>
		<ScrollArea.Root class="flex-1">
			<div class="space-y-4 p-4">
				{#each messages as message (`toc-${message.id}`)}
					<button class="group flex w-full items-start gap-2 text-left">
						<CaretRightIcon
							size={12}
							class="mt-1 shrink-0 text-muted-foreground/40 transition-transform group-hover:translate-x-0.5 group-hover:text-primary"
						/>
						<div class="flex flex-col gap-0.5">
							<span
								class="text-[11px] font-bold tracking-tighter text-muted-foreground/70 uppercase"
							>
								{message.role}
							</span>
							<p class="line-clamp-2 text-xs text-foreground/60 group-hover:text-foreground">
								{message.content}
							</p>
						</div>
					</button>
				{/each}
			</div>
		</ScrollArea.Root>
	</aside>
</div>
