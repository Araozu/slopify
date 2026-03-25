import type {
	ChatThreadDetails,
	ChatThreadListItem,
	CreateChatGenerationInput,
	CreateChatMessageInput,
	CreateChatThreadInput
} from './entities';
import { DEFAULT_CHAT_TITLE } from './entities';
import { createChatRepository } from './repository';

function toPreview(content: string) {
	return content.trim().replace(/\s+/g, ' ').slice(0, 160) || null;
}

function toTitle(title?: string, content?: string) {
	const explicitTitle = title?.trim();

	if (explicitTitle) {
		return explicitTitle;
	}

	const preview = toPreview(content ?? '');

	if (!preview) {
		return DEFAULT_CHAT_TITLE;
	}

	return preview.slice(0, 60);
}

function toThreadListItem(
	thread: ChatThreadDetails | Omit<ChatThreadDetails, 'messages' | 'generations'>
): ChatThreadListItem {
	return {
		id: thread.id,
		title: thread.title,
		status: thread.status,
		pinned: thread.pinned,
		summary: thread.summary,
		lastMessageAt: thread.lastMessageAt,
		updatedAt: thread.updatedAt
	};
}

export function createChatUseCases(chatRepository = createChatRepository()) {
	return {
		async listThreads(ownerId?: string | null) {
			const threads = await chatRepository.listThreads(ownerId);

			return threads.map((thread) => toThreadListItem(thread));
		},
		getThread(threadId: string) {
			return chatRepository.getThreadById(threadId);
		},
		async createThread(input: CreateChatThreadInput = {}) {
			const openingContent = input.openingMessage?.content?.trim();
			const now = new Date();

			const thread = await chatRepository.createThread({
				ownerId: input.ownerId ?? null,
				title: toTitle(input.title, openingContent),
				summary: input.summary ?? toPreview(openingContent ?? ''),
				status: 'active',
				pinned: false,
				createdAt: now,
				updatedAt: now,
				lastMessageAt: now,
				archivedAt: null
			});

			if (!openingContent) {
				return chatRepository.getThreadById(thread.id);
			}

			const message = await chatRepository.createMessage({
				threadId: thread.id,
				sequence: 1,
				role: input.openingMessage?.role ?? 'user',
				content: openingContent,
				model: input.openingMessage?.model ?? null,
				metadata: input.openingMessage?.metadata
			});

			await chatRepository.updateThread(thread.id, {
				summary: toPreview(message.content),
				lastMessageAt: message.createdAt,
				updatedAt: message.createdAt
			});

			return chatRepository.getThreadById(thread.id);
		},
		async addMessage(input: CreateChatMessageInput) {
			const thread = await chatRepository.getThreadById(input.threadId);

			if (!thread) {
				return null;
			}

			const nextSequence =
				(await chatRepository.getLastMessageSequence(input.threadId))?.sequence ?? 0;
			const content = input.content.trim();

			if (!content) {
				return thread;
			}

			const message = await chatRepository.createMessage({
				...input,
				content,
				sequence: nextSequence + 1
			});

			await chatRepository.updateThread(input.threadId, {
				title:
					thread.title === DEFAULT_CHAT_TITLE && input.role === 'user'
						? toTitle(undefined, message.content)
						: thread.title,
				summary: toPreview(message.content),
				lastMessageAt: message.createdAt,
				updatedAt: message.createdAt
			});

			return chatRepository.getThreadById(input.threadId);
		},
		async recordGeneration(input: CreateChatGenerationInput) {
			const thread = await chatRepository.getThreadById(input.threadId);

			if (!thread) {
				return null;
			}

			return chatRepository.createGeneration(input);
		}
	};
}

export type ChatUseCases = ReturnType<typeof createChatUseCases>;
