import { relations, sql } from 'drizzle-orm';
import {
	boolean,
	index,
	integer,
	jsonb,
	pgEnum,
	pgTable,
	real,
	text,
	timestamp,
	uniqueIndex,
	uuid,
	varchar
} from 'drizzle-orm/pg-core';

export const chatThreadStatusValues = ['draft', 'active', 'archived'] as const;
export const chatMessageRoleValues = ['system', 'user', 'assistant', 'tool'] as const;
export const chatMessageStatusValues = ['pending', 'streaming', 'complete', 'failed'] as const;
export const chatGenerationStatusValues = ['queued', 'running', 'complete', 'failed'] as const;

export const chatThreadStatus = pgEnum('chat_thread_status', chatThreadStatusValues);
export const chatMessageRole = pgEnum('chat_message_role', chatMessageRoleValues);
export const chatMessageStatus = pgEnum('chat_message_status', chatMessageStatusValues);
export const chatGenerationStatus = pgEnum('chat_generation_status', chatGenerationStatusValues);

export type ChatContentPart =
	| { type: 'text'; text: string }
	| { type: 'image'; url: string; mimeType?: string }
	| { type: 'file'; url: string; mimeType?: string; name?: string }
	| { type: 'tool-result'; text: string; metadata?: Record<string, unknown> };

export const appUser = pgTable(
	'app_user',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		email: varchar('email', { length: 320 }),
		displayName: text('display_name').notNull(),
		avatarUrl: text('avatar_url'),
		createdAt: timestamp('created_at', { withTimezone: true }).defaultNow().notNull(),
		updatedAt: timestamp('updated_at', { withTimezone: true }).defaultNow().notNull()
	},
	(table) => [uniqueIndex('app_user_email_unique').on(table.email)]
);

export const chatThread = pgTable(
	'chat_thread',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		ownerId: uuid('owner_id').references(() => appUser.id, { onDelete: 'set null' }),
		title: text('title').notNull(),
		summary: text('summary'),
		status: chatThreadStatus('status').notNull().default('active'),
		pinned: boolean('pinned').notNull().default(false),
		createdAt: timestamp('created_at', { withTimezone: true }).defaultNow().notNull(),
		updatedAt: timestamp('updated_at', { withTimezone: true }).defaultNow().notNull(),
		lastMessageAt: timestamp('last_message_at', { withTimezone: true }).defaultNow().notNull(),
		archivedAt: timestamp('archived_at', { withTimezone: true })
	},
	(table) => [
		index('chat_thread_status_idx').on(table.status),
		index('chat_thread_owner_last_message_idx').on(table.ownerId, table.lastMessageAt)
	]
);

export const chatMessage = pgTable(
	'chat_message',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		threadId: uuid('thread_id')
			.notNull()
			.references(() => chatThread.id, { onDelete: 'cascade' }),
		sequence: integer('sequence').notNull(),
		role: chatMessageRole('role').notNull(),
		status: chatMessageStatus('status').notNull().default('complete'),
		content: text('content').notNull(),
		contentParts: jsonb('content_parts')
			.$type<ChatContentPart[]>()
			.notNull()
			.default(sql`'[]'::jsonb`),
		metadata: jsonb('metadata')
			.$type<Record<string, unknown>>()
			.notNull()
			.default(sql`'{}'::jsonb`),
		model: varchar('model', { length: 120 }),
		createdAt: timestamp('created_at', { withTimezone: true }).defaultNow().notNull()
	},
	(table) => [
		index('chat_message_thread_created_idx').on(table.threadId, table.createdAt),
		uniqueIndex('chat_message_thread_sequence_unique').on(table.threadId, table.sequence)
	]
);

export const chatGeneration = pgTable(
	'chat_generation',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		threadId: uuid('thread_id')
			.notNull()
			.references(() => chatThread.id, { onDelete: 'cascade' }),
		startedByMessageId: uuid('started_by_message_id').references(() => chatMessage.id, {
			onDelete: 'set null'
		}),
		provider: varchar('provider', { length: 120 }).notNull(),
		model: varchar('model', { length: 120 }).notNull(),
		status: chatGenerationStatus('status').notNull().default('queued'),
		temperature: real('temperature').notNull().default(0.7),
		promptTokens: integer('prompt_tokens').notNull().default(0),
		completionTokens: integer('completion_tokens').notNull().default(0),
		totalTokens: integer('total_tokens').notNull().default(0),
		failureReason: text('failure_reason'),
		metadata: jsonb('metadata')
			.$type<Record<string, unknown>>()
			.notNull()
			.default(sql`'{}'::jsonb`),
		createdAt: timestamp('created_at', { withTimezone: true }).defaultNow().notNull(),
		completedAt: timestamp('completed_at', { withTimezone: true })
	},
	(table) => [
		index('chat_generation_thread_created_idx').on(table.threadId, table.createdAt),
		index('chat_generation_status_idx').on(table.status)
	]
);

export const appUserRelations = relations(appUser, ({ many }) => ({
	threads: many(chatThread)
}));

export const chatThreadRelations = relations(chatThread, ({ one, many }) => ({
	owner: one(appUser, {
		fields: [chatThread.ownerId],
		references: [appUser.id]
	}),
	messages: many(chatMessage),
	generations: many(chatGeneration)
}));

export const chatMessageRelations = relations(chatMessage, ({ one, many }) => ({
	thread: one(chatThread, {
		fields: [chatMessage.threadId],
		references: [chatThread.id]
	}),
	generations: many(chatGeneration)
}));

export const chatGenerationRelations = relations(chatGeneration, ({ one }) => ({
	thread: one(chatThread, {
		fields: [chatGeneration.threadId],
		references: [chatThread.id]
	}),
	startedByMessage: one(chatMessage, {
		fields: [chatGeneration.startedByMessageId],
		references: [chatMessage.id]
	})
}));
