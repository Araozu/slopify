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
	import { copilotTokensQueryOptions } from '$lib/queries/copilot-token-query';
	import {
		openRouterModelsQueryOptions,
		invalidateOpenRouterModels
	} from '$lib/queries/openrouter-model-query';
	import {
		copilotModelsQueryOptions,
		invalidateCopilotModels
	} from '$lib/queries/copilot-model-query';
	import { systemPromptsQueryOptions } from '$lib/queries/system-prompt-query';
	import {
		createThread,
		deleteThread,
		deleteMessagePair,
		forkThread,
		streamChatCompletion,
		updateThreadTitle
	} from '$lib/thread-client';
	import { createOpenRouterModel } from '$lib/openrouter-model-client';
	import { createCopilotModel } from '$lib/copilot-model-client';
	import type {
		Message,
		OpenRouterApiKey,
		CopilotToken,
		ProviderCredential,
		SystemPrompt,
		Thread,
		OpenRouterModel,
		CopilotModel
	} from '$lib/types';
	import { tick, untrack, onMount } from 'svelte';
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

	const DEFAULT_MODEL = '';
	const DEFAULT_THREAD_TITLE = 'New thread';
	const STREAM_FLUSH_INTERVAL_MS = 50;

	let { threadId }: Props = $props();
	const queryClient = useQueryClient();

	let messagesByThread = $state<MessagesByThread>({});
	let draft = $state('');
	let model = $state(DEFAULT_MODEL);
	let isSending = $state(false);
	let hasRequestedInitialThread = $state(false);
	let pendingStreamUpdates = $state<
		Record<string, { threadId: string; messageId: string; text: string; reasoning: string }>
	>({});

	const MOBILE_BREAKPOINT_PX = 768;

	let viewportRef: HTMLElement | null = $state(null);
	let flushTimer: ReturnType<typeof setTimeout> | null = null;
	let sidebarCollapsed = $state(false);
	let initialScrollDoneForThreadId = $state<string | null>(null);

	onMount(() => {
		if (window.innerWidth < MOBILE_BREAKPOINT_PX) {
			sidebarCollapsed = true;
		}
	});

	const threadsQuery = createQuery(() => threadsQueryOptions());
	const keysQuery = createQuery(() => openRouterKeysQueryOptions());
	const copilotTokensQuery = createQuery(() => copilotTokensQueryOptions());
	const modelsQuery = createQuery(() => openRouterModelsQueryOptions());
	const copilotModelsQuery = createQuery(() => copilotModelsQueryOptions());
	const systemPromptsQuery = createQuery(() => systemPromptsQueryOptions());

	const openRouterKeys = $derived((keysQuery.data ?? []) as OpenRouterApiKey[]);
	const copilotTokens = $derived((copilotTokensQuery.data ?? []) as CopilotToken[]);

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
		)
	]);

	type ThreadPrefs = { credentialId: string | null; systemPromptId: string | null };
	let threadPrefs = $state<Record<string, ThreadPrefs>>({});

	let selectedCredentialId = $state<string | null>(null);
	const selectedCredential = $derived(
		credentials.find((c) => c.id === selectedCredentialId) ?? credentials[0] ?? null
	);

	const canSaveModel = $derived(selectedCredential !== null);

	const systemPrompts = $derived((systemPromptsQuery.data ?? []) as SystemPrompt[]);
	let selectedSystemPromptId = $state<string | null>(null);

	// Restore per-thread credential and system prompt when the active thread changes.
	// `untrack` prevents `threadPrefs` writes from re-triggering this effect.
	$effect(() => {
		const tid = threadId;
		const prefs = untrack(() => threadPrefs)[tid];
		selectedCredentialId = prefs?.credentialId ?? null;
		selectedSystemPromptId = prefs?.systemPromptId ?? null;
	});

	const savedOpenRouterModels = $derived((modelsQuery.data ?? []) as OpenRouterModel[]);
	const savedCopilotModels = $derived((copilotModelsQuery.data ?? []) as CopilotModel[]);
	const activeSavedModels = $derived.by(() => {
		const provider = selectedCredential?.provider;
		if (provider === 'openrouter') return savedOpenRouterModels;
		if (provider === 'github-copilot') return savedCopilotModels;
		return [];
	});

	const createModelMutation = createMutation(() => ({
		mutationFn: (modelId: string) => createOpenRouterModel(modelId),
		onSuccess: async () => {
			await invalidateOpenRouterModels(queryClient);
		}
	}));

	const createCopilotModelMutation = createMutation(() => ({
		mutationFn: (modelId: string) => createCopilotModel(modelId),
		onSuccess: async () => {
			await invalidateCopilotModels(queryClient);
		}
	}));

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

	const renameThreadMutation = createMutation(() => ({
		mutationFn: ({ id, title }: { id: string; title: string }) => updateThreadTitle(id, title),
		onSuccess: (updated) => {
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
				(current ?? []).map((t) => (t.id === updated.id ? { ...t, ...updated } : t))
			);
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
			threadPrefs = Object.fromEntries(
				Object.entries(threadPrefs).filter(([key]) => key !== deletedId)
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

	const deleteMessagePairMutation = createMutation(() => ({
		mutationFn: ({ targetThreadId, messageId }: { targetThreadId: string; messageId: string }) =>
			deleteMessagePair(targetThreadId, messageId),
		onSuccess: (_, { targetThreadId, messageId }) => {
			const threadMessages = messagesByThread[targetThreadId] ?? [];
			const index = threadMessages.findIndex((m) => m.id === messageId);
			if (index === -1) return;
			const nextMessages = threadMessages.slice(0, index);
			updateThreadMessages(targetThreadId, nextMessages);
			queryClient.invalidateQueries({ queryKey: threadKeys.messages(targetThreadId) });
		}
	}));

	const forkThreadMutation = createMutation(() => ({
		mutationFn: ({ targetThreadId, messageId }: { targetThreadId: string; messageId: string }) =>
			forkThread(targetThreadId, messageId),
		onSuccess: async (newThread) => {
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) => [
				newThread,
				...(current ?? [])
			]);
			updateThreadMessages(newThread.id, []);
			await gotoThread(newThread.id, false);
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
		if (activeThread) {
			model = activeThread.model ?? DEFAULT_MODEL;
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
	let isRenamingThread = $derived(renameThreadMutation.isPending);
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

		const renameError = renameThreadMutation.error;
		if (renameError instanceof Error) {
			return renameError.message;
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

	$effect(() => {
		const tid = threadId;
		if (!tid || !activeThread || isBootstrapping || isLoadingMessages) {
			return;
		}
		if (initialScrollDoneForThreadId === tid) {
			return;
		}

		void tick().then(() => {
			if (tid !== threadId) {
				return;
			}
			scrollToLatest();
			initialScrollDoneForThreadId = tid;
		});
	});

	$effect(() => {
		if (!isSending) {
			return;
		}
		const tid = threadId;
		void tick().then(() => {
			if (tid !== threadId) {
				return;
			}
			scrollToLatest('smooth');
		});
	});

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

	function patchThreadMessage(
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
		patchThreadMessage(targetThreadId, messageId, update);
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

	async function handleRenameThread(title: string) {
		if (!threadId || isRenamingThread) {
			return;
		}

		await renameThreadMutation.mutateAsync({ id: threadId, title });
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

	function handleDeleteMessagePair(messageId: string) {
		if (!threadId) return;
		deleteMessagePairMutation.mutate({ targetThreadId: threadId, messageId });
	}

	function handleForkFromMessage(messageId: string) {
		if (!threadId) return;
		forkThreadMutation.mutate({ targetThreadId: threadId, messageId });
	}

	async function sendMessage() {
		if (!activeThread) {
			return;
		}

		const requestThreadId = activeThread.id;
		const prompt = draft.trim();
		const trimmedToken = selectedCredential?.token.trim() ?? '';
		const provider = selectedCredential?.provider;
		const selectedModel = model.trim();

		if (!prompt || !trimmedToken || !selectedModel || !provider || isSending) {
			return;
		}

		const userMessage: Message = {
			...createMessage('user', prompt),
			deliveryStatus: 'sent'
		};
		updateThreadMessages(requestThreadId, [...messages, userMessage]);
		draft = '';
		isSending = true;
		pendingStreamUpdates = {};

		try {
			const selectedSystemPrompt = selectedSystemPromptId
				? systemPrompts.find((sp) => sp.id === selectedSystemPromptId)
				: null;

			await streamChatCompletion(
				{
					model: selectedModel,
					thread_id: requestThreadId,
					prompt,
					system_prompt_id: selectedSystemPromptId ?? undefined,
					system_prompt: selectedSystemPrompt?.content,
					provider
				},
				trimmedToken,
				(event) => {
					switch (event.type) {
						case 'message_started': {
							const message = normalizeMessage(event.payload.message);
							patchThreadMessage(requestThreadId, userMessage.id, (currentMessage) => ({
								...currentMessage,
								deliveryStatus: 'delivered'
							}));
							upsertAssistantMessage(requestThreadId, {
								...message,
								status: 'streaming'
							});
							break;
						}
						case 'text_delta': {
							queueStreamDelta(
								requestThreadId,
								event.payload.message_id,
								'text',
								event.payload.delta
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
							break;
						}
						case 'message_completed': {
							flushPendingStreamUpdates();
							const completed = normalizeMessage(event.payload.message);
							upsertAssistantMessage(requestThreadId, {
								...completed,
								status: 'completed'
							});
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
							break;
						}
					}
				}
			);
		} catch (error) {
			const message = error instanceof Error ? error.message : 'Failed to send prompt.';
			updateThreadMessages(requestThreadId, [
				...(messagesByThread[requestThreadId] ?? []),
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

<div class="relative flex h-[calc(100vh-2rem)] min-h-0 w-full overflow-hidden bg-background">
	{#if !sidebarCollapsed}
		<div
			class="absolute inset-0 z-30 bg-background/60 backdrop-blur-sm md:hidden"
			onclick={() => (sidebarCollapsed = true)}
			onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && (sidebarCollapsed = true)}
			role="button"
			tabindex="0"
			aria-label="Close sidebar"
		></div>
	{/if}
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
			{isRenamingThread}
			{isSending}
			onRenameThread={handleRenameThread}
			onDeleteThread={handleDeleteThread}
			onToggleSidebar={() => (sidebarCollapsed = !sidebarCollapsed)}
		/>

		<ChatMessagesViewport
			bind:viewportRef
			{loadError}
			{isBootstrapping}
			{isLoadingMessages}
			{messages}
			onDeletePair={handleDeleteMessagePair}
			onFork={handleForkFromMessage}
		/>

		<ChatComposer
			bind:draft
			bind:model
			{credentials}
			{selectedCredential}
			{systemPrompts}
			{selectedSystemPromptId}
			savedModels={activeSavedModels}
			{canSaveModel}
			{isSending}
			{isBootstrapping}
			{activeThread}
			onSelectCredential={(id) => {
				selectedCredentialId = id;
				threadPrefs = {
					...threadPrefs,
					[threadId]: {
						credentialId: id,
						systemPromptId: threadPrefs[threadId]?.systemPromptId ?? null
					}
				};
			}}
			onSelectSystemPrompt={(id) => {
				selectedSystemPromptId = id;
				threadPrefs = {
					...threadPrefs,
					[threadId]: {
						credentialId: threadPrefs[threadId]?.credentialId ?? null,
						systemPromptId: id
					}
				};
			}}
			onSaveModel={(modelId) => {
				if (selectedCredential?.provider === 'github-copilot') {
					createCopilotModelMutation.mutate(modelId);
				} else {
					createModelMutation.mutate(modelId);
				}
			}}
			onSend={sendMessage}
			onComposerKeydown={handleComposerKeydown}
		/>
	</main>

	<StreamLogSidebar collapsed={sidebarCollapsed} />
</div>
