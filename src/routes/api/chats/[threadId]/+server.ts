import { chatUseCases } from '$lib/server/chat';
import { error, json } from '@sveltejs/kit';

export async function GET({ params }) {
	const thread = await chatUseCases.getThread(params.threadId);

	if (!thread) {
		throw error(404, 'Chat thread not found');
	}

	return json({ thread });
}
