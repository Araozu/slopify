<script lang="ts">
	import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import {
		threadKeys,
		threadMessagesQueryOptions,
		threadsQueryOptions
	} from '$lib/queries/thread-query';
	import { openRouterKeysQueryOptions } from '$lib/queries/openrouter-key-query';
	import {
		createThread,
		deleteThread,
		streamChatCompletion,
		type StreamChatEvent
	} from '$lib/thread-client';
	import type { Message, OpenRouterApiKey, Thread } from '$lib/types';
	import { tick, untrack } from 'svelte';
	import ChatComposer from './chat-composer.svelte';
	import ChatHeader from './chat-header.svelte';
	import ChatMessagesViewport from './chat-messages-viewport.svelte';
	import { getMessageReasoning, getMessageText } from './chat-message-utils.js';
	import StreamLogSidebar from './stream-log-sidebar.svelte';
	import ThreadSidebar from './thread-sidebar.svelte';

	type MessagesByThread = Record<string, Message[]>;

	interface Props {
		threadId: string;
	}

	const DEFAULT_MODEL = 'openai/gpt-4o-mini';
	const DEFAULT_THREAD_TITLE = 'New thread';
	const STREAM_LOG_LIMIT = 100;
	const STREAM_FLUSH_INTERVAL_MS = 50;

	let { threadId }: Props = $props();
	const queryClient = useQueryClient();

	let messagesByThread = $state<MessagesByThread>({});
	let draft = $state('');
	let model = $state(DEFAULT_MODEL);
	let isSending = $state(false);
	let hasRequestedInitialThread = $state(false);
	let streamEvents = $state<
		Array<{ id: string; timestamp: string; type: StreamChatEvent['type']; detail: string }>
	>([]);
	let pendingStreamUpdates = $state<
		Record<string, { threadId: string; messageId: string; text: string; reasoning: string }>
	>({});

	let viewportRef: HTMLElement | null = $state(null);
	let flushTimer: ReturnType<typeof setTimeout> | null = null;
	let sidebarCollapsed = $state(false);

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

	const deleteThreadMutation = createMutation(() => ({
		mutationFn: (id: string) => deleteThread(id),
		onSuccess: async (_, deletedId) => {
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
				(current ?? []).filter((t) => t.id !== deletedId)
			);
			queryClient.removeQueries({ queryKey: threadKeys.messages(deletedId) });
			messagesByThread = Object.fromEntries(
				Object.entries(messagesByThread).filter(([key]) => key !== deletedId)
			);
			if (deletedId === threadId) {
				const remaining = (queryClient.getQueryData(threadKeys.all) as Thread[] | undefined) ?? [];
				if (remaining.length > 0) {
					await gotoThread(remaining[0].id, true);
				} else {
					createThreadMutation.mutate({ replaceState: true });
				}
			}
		}
	}));

	let threads = $derived((threadsQuery.data ?? []) as Thread[]);
	let activeThread = $derived(threads.find((thread) => thread.id === threadId) ?? null);
	const threadMessagesQuery = createQuery(() => ({
		...threadMessagesQueryOptions(threadId),
		enabled: Boolean(threadId && activeThread)
	}));
	let messages = $derived(messagesByThread[threadId] ?? []);

	$effect(() => {
		if (activeThread?.model) {
			model = activeThread.model;
		}
	});
	let threadTitle = $derived(activeThread ? getThreadTitle(activeThread, messages) : 'Thread');
	let messageFlowSignature = $derived(
		messages
			.map((message) => {
				const text = getMessageText(message);
				const reasoning = getMessageReasoning(message);
				return `${message.id}:${message.status}:${text.length}:${reasoning.length}`;
			})
			.join('|')
	);
	let isCreatingThread = $derived(createThreadMutation.isPending);
	let isDeletingThread = $derived(deleteThreadMutation.isPending);
	let isLoadingMessages = $derived(
		Boolean(threadId && activeThread) &&
			threadMessagesQuery.isPending &&
			!(threadId in messagesByThread)
	);
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

		const messageError = threadMessagesQuery.error;
		if (messageError instanceof Error) {
			return messageError.message;
		}

		const deleteError = deleteThreadMutation.error;
		if (deleteError instanceof Error) {
			return deleteError.message;
		}

		return '';
	});
	let chatThreads = $derived(
		threads.map((thread) => {
			const threadMessages = messagesByThread[thread.id] ?? [];
			const lastMessage = threadMessages.at(-1);

			return {
				...thread,
				title: getThreadTitle(thread, threadMessages),
				lastMessage: lastMessage ? getMessageText(lastMessage) : 'No messages yet',
				messages: threadMessages
			};
		})
	);

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

	$effect(() => {
		if (!threadId || !threadMessagesQuery.isSuccess) {
			return;
		}

		const fetchedMessages = ((threadMessagesQuery.data ?? []) as Message[]).map(normalizeMessage);
		const currentMessagesByThread = untrack(() => messagesByThread);
		const currentMessages = currentMessagesByThread[threadId];
		if (currentMessages === fetchedMessages) {
			return;
		}

		messagesByThread = {
			...currentMessagesByThread,
			[threadId]: fetchedMessages
		};
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

	function isNearBottom(threshold = 120) {
		if (!viewportRef) {
			return true;
		}
		const distanceToBottom =
			viewportRef.scrollHeight - (viewportRef.scrollTop + viewportRef.clientHeight);
		return distanceToBottom <= threshold;
	}

	function createMessage(role: Message['role'], content: string): Message {
		const trimmedContent = content.trim();
		return {
			id: crypto.randomUUID(),
			role,
			status: 'completed',
			parts: trimmedContent ? [{ kind: 'text', text: trimmedContent }] : [],
			content: trimmedContent,
			timestamp: new Date().toISOString()
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
		return firstUserMessage ? truncateText(getMessageText(firstUserMessage), 36) : thread.title;
	}

	function updateThreadMessages(targetThreadId: string, nextMessages: Message[]) {
		queryClient.setQueryData<Message[]>(threadKeys.messages(targetThreadId), nextMessages);
		messagesByThread = {
			...messagesByThread,
			[targetThreadId]: nextMessages
		};
	}

	function normalizeMessage(message: Message): Message {
		const status = message.status ?? 'completed';
		const incomingParts = Array.isArray(message.parts) ? message.parts : [];
		const fallbackContent = typeof message.content === 'string' ? message.content : '';
		const normalizedParts =
			incomingParts.length > 0
				? incomingParts
				: fallbackContent
					? [{ kind: 'text' as const, text: fallbackContent }]
					: [];
		const content = normalizedParts
			.filter((part) => part.kind === 'text')
			.map((part) => part.text)
			.join('');

		return {
			...message,
			status,
			parts: normalizedParts,
			content: content || fallbackContent
		};
	}

	function pushStreamEvent(type: StreamChatEvent['type'], detail: string) {
		const nextEvent = {
			id: crypto.randomUUID(),
			timestamp: new Date().toISOString(),
			type,
			detail
		};

		streamEvents = [...streamEvents, nextEvent].slice(-STREAM_LOG_LIMIT);
	}

	function upsertAssistantMessage(targetThreadId: string, message: Message) {
		const threadMessages = messagesByThread[targetThreadId] ?? [];
		const normalized = normalizeMessage(message);
		const index = threadMessages.findIndex((existing) => existing.id === normalized.id);
		if (index === -1) {
			updateThreadMessages(targetThreadId, [...threadMessages, normalized]);
			return;
		}

		const nextMessages = [...threadMessages];
		nextMessages[index] = normalized;
		updateThreadMessages(targetThreadId, nextMessages);
	}

	function patchAssistantMessage(
		targetThreadId: string,
		messageId: string,
		update: (message: Message) => Message
	) {
		const threadMessages = messagesByThread[targetThreadId] ?? [];
		const index = threadMessages.findIndex((message) => message.id === messageId);
		if (index === -1) {
			return;
		}

		const nextMessages = [...threadMessages];
		nextMessages[index] = normalizeMessage(update(nextMessages[index]));
		updateThreadMessages(targetThreadId, nextMessages);
	}

	function queueStreamDelta(
		targetThreadId: string,
		messageId: string,
		kind: 'text' | 'reasoning',
		delta: string
	) {
		const updateKey = `${targetThreadId}:${messageId}`;
		const current = pendingStreamUpdates[updateKey] ?? {
			threadId: targetThreadId,
			messageId,
			text: '',
			reasoning: ''
		};
		pendingStreamUpdates = {
			...pendingStreamUpdates,
			[updateKey]: {
				...current,
				[kind]: current[kind] + delta
			}
		};

		if (flushTimer) {
			return;
		}

		flushTimer = setTimeout(() => {
			flushTimer = null;
			flushPendingStreamUpdates();
		}, STREAM_FLUSH_INTERVAL_MS);
	}

	function flushPendingStreamUpdates() {
		const updates = pendingStreamUpdates;
		const updateEntries = Object.entries(updates);
		if (updateEntries.length === 0) {
			return;
		}

		pendingStreamUpdates = {};

		for (const [, update] of updateEntries) {
			patchAssistantMessage(update.threadId, update.messageId, (message) => {
				const previousParts = Array.isArray(message.parts) ? [...message.parts] : [];
				const textPartIndex = previousParts.findIndex((part) => part.kind === 'text');
				const reasoningPartIndex = previousParts.findIndex((part) => part.kind === 'reasoning');

				if (update.text) {
					if (textPartIndex === -1) {
						previousParts.push({ kind: 'text', text: update.text });
					} else {
						previousParts[textPartIndex] = {
							kind: 'text',
							text: previousParts[textPartIndex].text + update.text
						};
					}
				}

				if (update.reasoning) {
					if (reasoningPartIndex === -1) {
						previousParts.push({ kind: 'reasoning', text: update.reasoning });
					} else {
						previousParts[reasoningPartIndex] = {
							kind: 'reasoning',
							text: previousParts[reasoningPartIndex].text + update.reasoning
						};
					}
				}

				const content = previousParts
					.filter((part) => part.kind === 'text')
					.map((part) => part.text)
					.join('');

				return {
					...message,
					status: 'streaming',
					parts: previousParts,
					content
				};
			});
		}
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

	async function handleDeleteThread(targetId: string) {
		if (isDeletingThread || !targetId) {
			return;
		}

		const confirmed = window.confirm(
			'Delete this thread and all its messages? This cannot be undone.'
		);
		if (!confirmed) {
			return;
		}

		deleteThreadMutation.mutate(targetId);
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
		streamEvents = [];
		pendingStreamUpdates = {};

		try {
			await streamChatCompletion(
				{
					model: selectedModel,
					thread_id: requestThreadId,
					prompt
				},
				trimmedApiKey,
				(event) => {
					switch (event.type) {
						case 'message_started': {
							const message = normalizeMessage(event.payload.message);
							upsertAssistantMessage(requestThreadId, {
								...message,
								status: 'streaming'
							});
							pushStreamEvent(event.type, `assistant ${message.id} started`);
							break;
						}
						case 'text_delta': {
							queueStreamDelta(
								requestThreadId,
								event.payload.message_id,
								'text',
								event.payload.delta
							);
							pushStreamEvent(
								event.type,
								`${event.payload.message_id} +${event.payload.delta.length} text`
							);
							break;
						}
						case 'reasoning_delta': {
							queueStreamDelta(
								requestThreadId,
								event.payload.message_id,
								'reasoning',
								event.payload.delta
							);
							pushStreamEvent(
								event.type,
								`${event.payload.message_id} +${event.payload.delta.length} reasoning`
							);
							break;
						}
						case 'message_completed': {
							flushPendingStreamUpdates();
							const completed = normalizeMessage(event.payload.message);
							upsertAssistantMessage(requestThreadId, {
								...completed,
								status: 'completed'
							});
							pushStreamEvent(event.type, `assistant ${completed.id} completed`);
							const completedModel = completed.provider?.model;
							if (completedModel && !activeThread?.model) {
								queryClient.setQueryData<Thread[]>(threadKeys.all, (currentThreads) =>
									(currentThreads ?? []).map((t) =>
										t.id === requestThreadId ? { ...t, model: completedModel } : t
									)
								);
							}
							break;
						}
						case 'message_failed': {
							flushPendingStreamUpdates();
							patchAssistantMessage(requestThreadId, event.payload.message_id, (message) => {
								const errorText = `Error: ${event.payload.error.message}`;
								const nextParts = [...(message.parts ?? [])];
								if (!nextParts.some((part) => part.kind === 'text')) {
									nextParts.push({ kind: 'text', text: errorText });
								}
								return {
									...message,
									status: 'failed',
									parts: nextParts,
									content: getMessageText({ ...message, parts: nextParts })
								};
							});
							pushStreamEvent(
								event.type,
								`${event.payload.message_id} failed: ${event.payload.error.message}`
							);
							break;
						}
					}
				}
			);
		} catch (error) {
			const message = error instanceof Error ? error.message : 'Failed to send prompt.';
			pushStreamEvent('message_failed', `request failed: ${message}`);
			updateThreadMessages(requestThreadId, [
				...nextMessages,
				createMessage('assistant', `Error: ${message}`)
			]);
		} finally {
			flushPendingStreamUpdates();
			isSending = false;
			void queryClient.invalidateQueries({ queryKey: threadKeys.all });
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
		const currentSignature = messageFlowSignature;
		const shouldAutoFollow = isNearBottom();

		void tick().then(() => {
			if (!currentThreadId || !currentSignature || !shouldAutoFollow) {
				return;
			}

			scrollToLatest();
		});
	});

	$effect(() => {
		return () => {
			if (flushTimer) {
				clearTimeout(flushTimer);
				flushTimer = null;
			}
		};
	});
</script>

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full overflow-hidden bg-background">
	<ThreadSidebar
		collapsed={sidebarCollapsed}
		{chatThreads}
		{threadId}
		{isCreatingThread}
		{isDeletingThread}
		onCreateThread={handleCreateThread}
		onSelectThread={(id) => void gotoThread(id)}
		onDeleteThread={handleDeleteThread}
	/>

	<main class="flex min-h-0 flex-1 flex-col bg-background/50 backdrop-blur-sm">
		<ChatHeader
			{threadTitle}
			{activeThread}
			{threadId}
			{sidebarCollapsed}
			{isDeletingThread}
			{isSending}
			onDeleteThread={handleDeleteThread}
			onToggleSidebar={() => (sidebarCollapsed = !sidebarCollapsed)}
		/>

		<ChatMessagesViewport
			bind:viewportRef
			{loadError}
			{isBootstrapping}
			{isLoadingMessages}
			{messages}
		/>

		<ChatComposer
			bind:draft
			bind:model
			{keys}
			{selectedKey}
			{isSending}
			{isBootstrapping}
			{activeThread}
			onSelectKey={(id) => (selectedKeyId = id)}
			onSend={sendMessage}
			onComposerKeydown={handleComposerKeydown}
		/>
	</main>

	<StreamLogSidebar collapsed={sidebarCollapsed} {streamEvents} />
</div>
