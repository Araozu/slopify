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
	import { openAiTokensQueryOptions } from '$lib/queries/openai-token-query';
	import { zenKeysQueryOptions } from '$lib/queries/zen-key-query';
	import {
		openRouterModelsQueryOptions,
		invalidateOpenRouterModels
	} from '$lib/queries/openrouter-model-query';
	import {
		copilotModelsQueryOptions,
		invalidateCopilotModels
	} from '$lib/queries/copilot-model-query';
	import { systemPromptsQueryOptions } from '$lib/queries/system-prompt-query';
	import { tagsQueryOptions, tagKeys } from '$lib/queries/tag-query';
	import {
		createThread,
		deleteThread,
		deleteMessagePair,
		forkThread,
		streamChatCompletion,
		updateThreadTitle
	} from '$lib/thread-client';
	import { createTag as createTagApi, addTagToThread, removeTagFromThread } from '$lib/tag-client';
	import { createOpenRouterModel } from '$lib/openrouter-model-client';
	import { createCopilotModel } from '$lib/copilot-model-client';
	import type {
		Message,
		OpenRouterApiKey,
		CopilotToken,
		OpenAiToken,
		ZenApiKey,
		ProviderCredential,
		SystemPrompt,
		Tag,
		Thread,
		OpenRouterModel,
		CopilotModel
	} from '$lib/types';
	import { tick, untrack, onMount } from 'svelte';
	import { get } from 'svelte/store';
	import ChatComposer from './chat-composer.svelte';
	import ChatHeader from './chat-header.svelte';
	import ChatMessagesViewport from './chat-messages-viewport.svelte';
	import { getMessageReasoning, getMessageText } from './chat-message-utils.js';
	import StreamLogSidebar from './stream-log-sidebar.svelte';
	import ThreadSidebar from './thread-sidebar.svelte';
	import ConfirmDialog from './confirm-dialog.svelte';
	import { threadDefaults } from '$lib/stores/thread-defaults';
	import { getDraft, setDraft } from '$lib/stores/thread-drafts';

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

	// Load the saved draft whenever the active thread changes.
	$effect(() => {
		const id = threadId;
		draft = untrack(() => getDraft(id));
	});

	// Keep the in-memory store in sync as the user types.
	$effect(() => {
		setDraft(
			untrack(() => threadId),
			draft
		);
	});
	let isSending = $state(false);
	let hasRequestedInitialThread = $state(false);
	let initializedThreadIds = $state(new Set<string>());
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
	const openAiTokensQuery = createQuery(() => openAiTokensQueryOptions());
	const zenKeysQuery = createQuery(() => zenKeysQueryOptions());
	const modelsQuery = createQuery(() => openRouterModelsQueryOptions());
	const copilotModelsQuery = createQuery(() => copilotModelsQueryOptions());
	const systemPromptsQuery = createQuery(() => systemPromptsQueryOptions());
	const tagsQuery = createQuery(() => tagsQueryOptions());

	const openRouterKeys = $derived((keysQuery.data ?? []) as OpenRouterApiKey[]);
	const copilotTokens = $derived((copilotTokensQuery.data ?? []) as CopilotToken[]);
	const openAiTokens = $derived((openAiTokensQuery.data ?? []) as OpenAiToken[]);
	const zenKeys = $derived((zenKeysQuery.data ?? []) as ZenApiKey[]);

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
		),
		...zenKeys.map(
			(k): ProviderCredential => ({
				id: k.id,
				name: k.name,
				provider: 'opencode-zen',
				token: k.apiKey
			})
		)
	]);

	type ThreadPrefs = { credentialId: string | null; systemPromptId: string | null };
	let threadPrefs = $state<Record<string, ThreadPrefs>>({});

	let selectedCredentialId = $state<string | null>(null);
	const selectedCredential = $derived(
		credentials.find((c) => c.id === selectedCredentialId) ?? credentials[0] ?? null
	);

	const canSaveModel = $derived.by(() => {
		const provider = selectedCredential?.provider;
		return provider === 'openrouter' || provider === 'github-copilot';
	});

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
		mutationFn: ({ title }: { title?: string; replaceState?: boolean; optimisticId?: string }) =>
			createThread(title),
		onMutate: async ({ replaceState, optimisticId }) => {
			if (!optimisticId) return;
			// Optimistically insert a placeholder thread and navigate to it
			const placeholder: Thread = {
				id: optimisticId,
				title: DEFAULT_THREAD_TITLE,
				tags: []
			};
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) => [
				placeholder,
				...(current ?? [])
			]);
			updateThreadMessages(optimisticId, []);
			draft = '';
			await gotoThread(optimisticId, replaceState ?? false);
		},
		onSuccess: async (newThread, variables) => {
			const { optimisticId, replaceState } = variables;
			if (optimisticId) {
				// Replace placeholder with real thread
				queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
					(current ?? []).map((t) => (t.id === optimisticId ? newThread : t))
				);
				// Move messages to real thread ID
				const msgs = messagesByThread[optimisticId] ?? [];
				updateThreadMessages(newThread.id, msgs);
				messagesByThread = Object.fromEntries(
					Object.entries(messagesByThread).filter(([key]) => key !== optimisticId)
				);
				// Navigate to real thread
				await gotoThread(newThread.id, true);
			} else {
				queryClient.setQueryData<Thread[]>(threadKeys.all, (currentThreads) => [
					newThread,
					...(currentThreads ?? [])
				]);
				updateThreadMessages(newThread.id, []);
				draft = '';
				await gotoThread(newThread.id, replaceState ?? false);
			}
		},
		onError: (_error, variables) => {
			const { optimisticId } = variables;
			if (optimisticId) {
				// Roll back optimistic thread
				queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
					(current ?? []).filter((t) => t.id !== optimisticId)
				);
				messagesByThread = Object.fromEntries(
					Object.entries(messagesByThread).filter(([key]) => key !== optimisticId)
				);
				// Navigate back to first remaining thread if we're on the failed one
				const remaining = (queryClient.getQueryData(threadKeys.all) as Thread[] | undefined) ?? [];
				if (remaining.length > 0) {
					void gotoThread(remaining[0].id, true);
				}
			}
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

	const createTagMutation = createMutation(() => ({
		mutationFn: ({ name, color }: { name: string; color: string }) => createTagApi(name, color),
		onSuccess: (newTag) => {
			queryClient.setQueryData<Tag[]>(tagKeys.all, (current) => [...(current ?? []), newTag]);
		}
	}));

	const addTagToThreadMutation = createMutation(() => ({
		mutationFn: ({ tagId }: { tagId: string }) => addTagToThread(threadId, tagId),
		onSuccess: (_, { tagId }) => {
			const tag = (queryClient.getQueryData<Tag[]>(tagKeys.all) ?? []).find((t) => t.id === tagId);
			if (!tag) return;
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
				(current ?? []).map((t) =>
					t.id === threadId ? { ...t, tags: [...(t.tags ?? []), tag] } : t
				)
			);
		}
	}));

	const removeTagFromThreadMutation = createMutation(() => ({
		mutationFn: ({ tagId }: { tagId: string }) => removeTagFromThread(threadId, tagId),
		onSuccess: (_, { tagId }) => {
			queryClient.setQueryData<Thread[]>(threadKeys.all, (current) =>
				(current ?? []).map((t) =>
					t.id === threadId ? { ...t, tags: (t.tags ?? []).filter((tag) => tag.id !== tagId) } : t
				)
			);
		}
	}));

	let threads = $derived((threadsQuery.data ?? []) as Thread[]);
	let activeThread = $derived(threads.find((thread) => thread.id === threadId) ?? null);
	const threadMessagesQuery = createQuery(() => ({
		...threadMessagesQueryOptions(threadId),
		enabled: Boolean(threadId && activeThread)
	}));
	let messages = $derived(messagesByThread[threadId] ?? []);
	let availableTags = $derived((tagsQuery.data ?? []) as Tag[]);
	let isTagLoading = $derived(
		createTagMutation.isPending ||
			addTagToThreadMutation.isPending ||
			removeTagFromThreadMutation.isPending
	);

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
	let isComposerBootstrapping = $derived(
		isBootstrapping ||
			!keysQuery.isSuccess ||
			!copilotTokensQuery.isSuccess ||
			!openAiTokensQuery.isSuccess ||
			!zenKeysQuery.isSuccess ||
			!systemPromptsQuery.isSuccess
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

		const deletePairError = deleteMessagePairMutation.error;
		if (deletePairError instanceof Error) {
			return deletePairError.message;
		}

		return '';
	});
	let chatThreads = $derived(
		threads.map((thread) => {
			const threadMessages = messagesByThread[thread.id] ?? [];
			const pendingOptimisticId = createThreadMutation.isPending
				? createThreadMutation.variables?.optimisticId
				: undefined;

			return {
				...thread,
				title: getThreadTitle(thread, threadMessages),
				messages: threadMessages,
				isOptimistic: thread.id === pendingOptimisticId
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

		const currentlySending = untrack(() => isSending);
		const fetchedMessages = ((threadMessagesQuery.data ?? []) as Message[]).map((m) => {
			const normalized = normalizeMessage(m);
			// If we are not actively streaming and a message has status "streaming",
			// it was left behind by a previous interrupted session. Resolve it based
			// on whether it has content.
			if (!currentlySending && normalized.status === 'streaming') {
				const hasContent = (normalized.parts ?? []).some(
					(p) => p.kind === 'text' && p.text.length > 0
				);
				return { ...normalized, status: hasContent ? ('completed' as const) : ('failed' as const) };
			}
			return normalized;
		});
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

	// Once per session per thread: pre-populate composer selectors from the last message.
	// Waits until thread messages and all credential/prompt data are fully loaded.
	$effect(() => {
		const tid = threadId;
		const currentMessages = messages;

		if (
			!tid ||
			!threadMessagesQuery.isSuccess ||
			!keysQuery.isSuccess ||
			!copilotTokensQuery.isSuccess ||
			!openAiTokensQuery.isSuccess ||
			!systemPromptsQuery.isSuccess
		) {
			return;
		}

		if (untrack(() => initializedThreadIds.has(tid))) {
			return;
		}

		const currentCredentials = credentials;
		const currentSystemPrompts = systemPrompts;

		untrack(() => {
			let nextCredentialId: string | null = null;
			let nextSystemPromptId: string | null = null;

			if (currentMessages.length === 0) {
				// New/empty thread — apply stored defaults when they still exist
				const defaults = get(threadDefaults);
				if (
					defaults.credentialId &&
					currentCredentials.some((c) => c.id === defaults.credentialId)
				) {
					nextCredentialId = defaults.credentialId;
				}
				if (
					defaults.systemPromptId &&
					currentSystemPrompts.some((p) => p.id === defaults.systemPromptId)
				) {
					nextSystemPromptId = defaults.systemPromptId;
				}
				if (defaults.model) {
					model = defaults.model;
				}
			} else {
				const lastWithProvider = currentMessages.findLast((m) => m.provider != null);
				if (lastWithProvider?.provider?.model) {
					model = lastWithProvider.provider.model;
				}
				if (lastWithProvider?.provider?.provider) {
					const matchingCredential = currentCredentials.find(
						(c) => c.provider === lastWithProvider.provider!.provider
					);
					if (matchingCredential) {
						nextCredentialId = matchingCredential.id;
					}
				}

				const lastWithSysPrompt = currentMessages.findLast((m) => m.system_prompt !== undefined);
				if (lastWithSysPrompt) {
					if (lastWithSysPrompt.system_prompt) {
						const matchingPrompt = currentSystemPrompts.find(
							(p) => p.content === lastWithSysPrompt.system_prompt
						);
						nextSystemPromptId = matchingPrompt?.id ?? null;
					} else {
						nextSystemPromptId = null;
					}
				}
			}

			selectedCredentialId = nextCredentialId;
			selectedSystemPromptId = nextSystemPromptId;
			threadPrefs = {
				...threadPrefs,
				[tid]: {
					credentialId: nextCredentialId,
					systemPromptId: nextSystemPromptId
				}
			};
			initializedThreadIds = new Set([...initializedThreadIds, tid]);
		});
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

		createThreadMutation.mutate({ replaceState: false, optimisticId: crypto.randomUUID() });
	}

	async function handleRenameThread(title: string) {
		if (!threadId || isRenamingThread) {
			return;
		}

		await renameThreadMutation.mutateAsync({ id: threadId, title });
	}

	// Confirm dialog state
	let confirmDialogOpen = $state(false);
	let confirmDialogPendingId = $state<string | null>(null);

	async function handleDeleteThread(targetId: string) {
		if (isDeletingThread || !targetId) {
			return;
		}

		confirmDialogPendingId = targetId;
		confirmDialogOpen = true;
	}

	function handleConfirmDeleteThread() {
		confirmDialogOpen = false;
		const targetId = confirmDialogPendingId;
		confirmDialogPendingId = null;
		if (targetId) {
			deleteThreadMutation.mutate(targetId);
		}
	}

	function handleCancelDeleteThread() {
		confirmDialogOpen = false;
		confirmDialogPendingId = null;
	}

	function handleDeleteMessagePair(messageId: string) {
		if (!threadId) return;
		deleteMessagePairMutation.mutate({ targetThreadId: threadId, messageId });
	}

	function handleForkFromMessage(messageId: string) {
		if (!threadId) return;
		forkThreadMutation.mutate({ targetThreadId: threadId, messageId });
	}

	async function handleRetry(assistantMessageId: string) {
		if (!threadId || isSending) return;
		const currentMessages = messages;
		const idx = currentMessages.findIndex((m) => m.id === assistantMessageId);
		if (idx <= 0) return;
		const userMsg = currentMessages[idx - 1];
		if (userMsg.role !== 'user') return;
		const userText = getMessageText(userMsg);
		try {
			await deleteMessagePairMutation.mutateAsync({
				targetThreadId: threadId,
				messageId: userMsg.id
			});
			draft = userText;
			await sendMessage();
		} catch {
			// error surfaced by mutation
		}
	}

	async function handleEditResend(userMessageId: string, newText: string) {
		if (!threadId || isSending) return;
		const trimmed = newText.trim();
		if (!trimmed) return;
		try {
			await deleteMessagePairMutation.mutateAsync({
				targetThreadId: threadId,
				messageId: userMessageId
			});
			draft = trimmed;
			await sendMessage();
		} catch {
			// error surfaced by mutation
		}
	}

	// Default color for new tags - derived from existing tag count for consistency across remounts
	const TAG_COLOR_PALETTE = [
		'#6366f1',
		'#ec4899',
		'#f59e0b',
		'#10b981',
		'#3b82f6',
		'#8b5cf6',
		'#ef4444',
		'#14b8a6'
	];

	function getNextTagColor(): string {
		return TAG_COLOR_PALETTE[availableTags.length % TAG_COLOR_PALETTE.length];
	}

	function handleAddTag(tagId: string) {
		addTagToThreadMutation.mutate({ tagId });
	}

	function handleRemoveTag(tagId: string) {
		removeTagFromThreadMutation.mutate({ tagId });
	}

	function handleCreateTag(name: string) {
		const color = getNextTagColor();
		createTagMutation.mutate(
			{ name, color },
			{
				onSuccess: (newTag) => {
					addTagToThreadMutation.mutate({ tagId: newTag.id });
				}
			}
		);
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
		if (event.key === 'Enter' && event.ctrlKey) {
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
		{availableTags}
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
			isBootstrapping={isComposerBootstrapping}
			{availableTags}
			{isTagLoading}
			{credentials}
			{selectedCredential}
			{systemPrompts}
			{selectedSystemPromptId}
			savedModels={activeSavedModels}
			{canSaveModel}
			bind:model
			onRenameThread={handleRenameThread}
			onDeleteThread={handleDeleteThread}
			onToggleSidebar={() => (sidebarCollapsed = !sidebarCollapsed)}
			onAddTag={handleAddTag}
			onRemoveTag={handleRemoveTag}
			onCreateTag={handleCreateTag}
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
				} else if (selectedCredential?.provider === 'openrouter') {
					createModelMutation.mutate(modelId);
				}
			}}
		/>

		<ChatMessagesViewport
			bind:viewportRef
			{loadError}
			{isBootstrapping}
			{isLoadingMessages}
			{messages}
			onDeletePair={handleDeleteMessagePair}
			onFork={handleForkFromMessage}
			onRetry={handleRetry}
			onEditResend={handleEditResend}
		/>

		<ChatComposer
			bind:draft
			bind:model
			{selectedCredential}
			{isSending}
			isBootstrapping={isComposerBootstrapping}
			{activeThread}
			onSend={sendMessage}
			onComposerKeydown={handleComposerKeydown}
		/>
	</main>

	<StreamLogSidebar collapsed={sidebarCollapsed} {messages} {viewportRef} />
</div>

<ConfirmDialog
	bind:open={confirmDialogOpen}
	title="Delete thread"
	description="Delete this thread and all its messages? This cannot be undone."
	confirmLabel="Delete"
	onConfirm={handleConfirmDeleteThread}
	onCancel={handleCancelDeleteThread}
/>
