import type { Message, Thread } from '$lib/types';

const THREADS_API_ENDPOINT = '/api/v1/threads';

export async function listThreads(signal?: AbortSignal): Promise<Thread[]> {
	const response = await fetch(THREADS_API_ENDPOINT, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as Thread[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load threads.');
	}

	return payload;
}

export async function createThread(title?: string): Promise<Thread> {
	const response = await fetch(THREADS_API_ENDPOINT, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({ title })
	});
	const payload = (await response.json()) as Thread | { error?: string };

	if (!response.ok || !('id' in payload) || !('title' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to create thread.');
	}

	return payload;
}

export async function listThreadMessages(
	threadId: string,
	signal?: AbortSignal
): Promise<Message[]> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}/messages`, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as Message[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load messages.');
	}

	return payload;
}
