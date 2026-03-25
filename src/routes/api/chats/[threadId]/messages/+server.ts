import { chatUseCases } from '$lib/server/chat';
import { chatMessageRoleValues, chatMessageStatusValues } from '$lib/server/db/schema';
import { error, json } from '@sveltejs/kit';

export async function POST({ params, request }) {
	const body = (await request.json()) as {
		role?: 'system' | 'user' | 'assistant' | 'tool';
		content?: string;
		status?: 'pending' | 'streaming' | 'complete' | 'failed';
		model?: string | null;
		metadata?: Record<string, unknown>;
	};

	if (!body.content?.trim()) {
		throw error(400, 'Message content is required');
	}

	if (body.role && !chatMessageRoleValues.includes(body.role)) {
		throw error(400, 'Invalid message role');
	}

	if (body.status && !chatMessageStatusValues.includes(body.status)) {
		throw error(400, 'Invalid message status');
	}

	const thread = await chatUseCases.addMessage({
		threadId: params.threadId,
		role: body.role ?? 'user',
		content: body.content,
		status: body.status,
		model: body.model,
		metadata: body.metadata
	});

	if (!thread) {
		throw error(404, 'Chat thread not found');
	}

	return json({ thread }, { status: 201 });
}
