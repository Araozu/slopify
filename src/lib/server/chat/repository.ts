import { desc, eq, isNull } from 'drizzle-orm';
import { db } from '../db';
import { chatGeneration, chatMessage, chatThread } from '../db/schema';
import type { CreateChatGenerationInput, CreateChatMessageInput, NewChatThread } from './entities';

type DatabaseClient = typeof db;

export function createChatRepository(database: DatabaseClient = db) {
	return {
		listThreads(ownerId?: string | null) {
			return database.query.chatThread.findMany({
				where:
					ownerId === undefined
						? undefined
						: ownerId === null
							? isNull(chatThread.ownerId)
							: eq(chatThread.ownerId, ownerId),
				orderBy: [desc(chatThread.pinned), desc(chatThread.lastMessageAt)]
			});
		},
		getThreadById(threadId: string) {
			return database.query.chatThread.findFirst({
				where: eq(chatThread.id, threadId),
				with: {
					generations: {
						orderBy: [desc(chatGeneration.createdAt)]
					},
					messages: {
						orderBy: (message, { asc }) => [asc(message.sequence)]
					}
				}
			});
		},
		createThread(input: NewChatThread) {
			return database
				.insert(chatThread)
				.values(input)
				.returning()
				.then(([thread]) => thread);
		},
		updateThread(
			threadId: string,
			values: Partial<
				Pick<
					NewChatThread,
					'title' | 'summary' | 'status' | 'updatedAt' | 'lastMessageAt' | 'archivedAt'
				>
			>
		) {
			return database
				.update(chatThread)
				.set(values)
				.where(eq(chatThread.id, threadId))
				.returning()
				.then(([thread]) => thread);
		},
		getLastMessageSequence(threadId: string) {
			return database.query.chatMessage.findFirst({
				where: eq(chatMessage.threadId, threadId),
				orderBy: [desc(chatMessage.sequence)],
				columns: {
					sequence: true
				}
			});
		},
		createMessage(input: CreateChatMessageInput & { sequence: number }) {
			return database
				.insert(chatMessage)
				.values({
					threadId: input.threadId,
					sequence: input.sequence,
					role: input.role,
					status: input.status ?? 'complete',
					content: input.content,
					contentParts: [{ type: 'text', text: input.content }],
					metadata: input.metadata ?? {},
					model: input.model ?? null
				})
				.returning()
				.then(([message]) => message);
		},
		createGeneration(input: CreateChatGenerationInput) {
			return database
				.insert(chatGeneration)
				.values({
					threadId: input.threadId,
					startedByMessageId: input.startedByMessageId ?? null,
					provider: input.provider,
					model: input.model,
					status: input.status ?? 'queued',
					temperature: input.temperature ?? 0.7,
					metadata: input.metadata ?? {}
				})
				.returning()
				.then(([generation]) => generation);
		}
	};
}

export type ChatRepository = ReturnType<typeof createChatRepository>;
