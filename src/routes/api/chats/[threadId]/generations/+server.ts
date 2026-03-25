import { chatUseCases } from '$lib/server/chat';
import { chatGenerationStatusValues } from '$lib/server/db/schema';
import { error, json } from '@sveltejs/kit';

export async function POST({ params, request }) {
	const body = (await request.json()) as {
		startedByMessageId?: string | null;
		provider?: string;
		model?: string;
		status?: 'queued' | 'running' | 'complete' | 'failed';
		temperature?: number;
		metadata?: Record<string, unknown>;
	};

	if (!body.provider?.trim() || !body.model?.trim()) {
		throw error(400, 'Provider and model are required');
	}

	if (body.status && !chatGenerationStatusValues.includes(body.status)) {
		throw error(400, 'Invalid generation status');
	}

	const generation = await chatUseCases.recordGeneration({
		threadId: params.threadId,
		startedByMessageId: body.startedByMessageId,
		provider: body.provider.trim(),
		model: body.model.trim(),
		status: body.status,
		temperature: body.temperature,
		metadata: body.metadata
	});

	if (!generation) {
		throw error(404, 'Chat thread not found');
	}

	return json({ generation }, { status: 201 });
}
