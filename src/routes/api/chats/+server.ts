import { chatUseCases } from '$lib/server/chat';
import { chatMessageRoleValues } from '$lib/server/db/schema';
import { error, json } from '@sveltejs/kit';

export async function GET() {
	const threads = await chatUseCases.listThreads();

	return json({ threads });
}

export async function POST({ request }) {
	const body = (await request.json()) as {
		ownerId?: string | null;
		title?: string;
		summary?: string | null;
		openingMessage?: {
			role?: 'system' | 'user' | 'assistant' | 'tool';
			content?: string;
			model?: string | null;
			metadata?: Record<string, unknown>;
		};
	};

	if (body.openingMessage?.role && !chatMessageRoleValues.includes(body.openingMessage.role)) {
		throw error(400, 'Invalid opening message role');
	}

	const thread = await chatUseCases.createThread({
		ownerId: body.ownerId,
		title: body.title,
		summary: body.summary,
		openingMessage: body.openingMessage?.content
			? {
					role: body.openingMessage.role,
					content: body.openingMessage.content,
					model: body.openingMessage.model,
					metadata: body.openingMessage.metadata
				}
			: undefined
	});

	if (!thread) {
		throw error(500, 'Failed to create chat thread');
	}

	return json({ thread }, { status: 201 });
}
