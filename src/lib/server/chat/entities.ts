import type { InferInsertModel, InferSelectModel } from 'drizzle-orm';
import type { appUser, chatGeneration, chatMessage, chatThread } from '../db/schema';

export const DEFAULT_CHAT_TITLE = 'New chat';

export type AppUserRecord = InferSelectModel<typeof appUser>;
export type ChatThreadRecord = InferSelectModel<typeof chatThread>;
export type ChatMessageRecord = InferSelectModel<typeof chatMessage>;
export type ChatGenerationRecord = InferSelectModel<typeof chatGeneration>;

export type NewAppUser = InferInsertModel<typeof appUser>;
export type NewChatThread = InferInsertModel<typeof chatThread>;
export type NewChatMessage = InferInsertModel<typeof chatMessage>;
export type NewChatGeneration = InferInsertModel<typeof chatGeneration>;

export interface ChatThreadDetails extends ChatThreadRecord {
	messages: ChatMessageRecord[];
	generations: ChatGenerationRecord[];
}

export interface ChatThreadListItem {
	id: string;
	title: string;
	status: ChatThreadRecord['status'];
	pinned: boolean;
	summary: string | null;
	lastMessageAt: Date;
	updatedAt: Date;
}

export interface CreateChatThreadInput {
	ownerId?: string | null;
	title?: string;
	summary?: string | null;
	openingMessage?: {
		role?: ChatMessageRecord['role'];
		content: string;
		model?: string | null;
		metadata?: Record<string, unknown>;
	};
}

export interface CreateChatMessageInput {
	threadId: string;
	role: ChatMessageRecord['role'];
	content: string;
	status?: ChatMessageRecord['status'];
	model?: string | null;
	metadata?: Record<string, unknown>;
}

export interface CreateChatGenerationInput {
	threadId: string;
	startedByMessageId?: string | null;
	provider: string;
	model: string;
	status?: ChatGenerationRecord['status'];
	temperature?: number;
	metadata?: Record<string, unknown>;
}
