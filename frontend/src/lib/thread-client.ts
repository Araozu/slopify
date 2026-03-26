import { browser } from '$app/environment';
import type { Message, Thread } from '$lib/types';

const THREADS_API_ENDPOINT = '/api/v1/threads';
const THREAD_MESSAGES_STORAGE_KEY = 'slopify-thread-messages';

type StoredMessagesByThread = Record<string, Message[]>;

export async function listThreads(): Promise<Thread[]> {
	const response = await fetch(THREADS_API_ENDPOINT);
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
		body: JSON.stringify({ title })
	});
	const payload = (await response.json()) as Thread | { error?: string };

	if (!response.ok || !('id' in payload) || !('title' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to create thread.');
	}

	return payload;
}

export function loadMessagesByThread(): StoredMessagesByThread {
	if (!browser) {
		return {};
	}

	const storedMessages = localStorage.getItem(THREAD_MESSAGES_STORAGE_KEY);
	if (!storedMessages) {
		return {};
	}

	try {
		const parsed = JSON.parse(storedMessages) as StoredMessagesByThread;
		return Object.fromEntries(
			Object.entries(parsed).map(([threadId, messages]) => [
				threadId,
				Array.isArray(messages) ? messages : []
			])
		);
	} catch {
		return {};
	}
}

export function saveMessagesByThread(messagesByThread: StoredMessagesByThread) {
	if (!browser) {
		return;
	}

	localStorage.setItem(THREAD_MESSAGES_STORAGE_KEY, JSON.stringify(messagesByThread));
}
