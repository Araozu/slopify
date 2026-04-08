import type { Message, Thread } from '$lib/types';

const THREADS_API_ENDPOINT = '/api/v1/threads';
const CHAT_API_ENDPOINT = '/api/v1/chat/completions';

export type StreamChatPayload = {
	model: string;
	thread_id: string;
	prompt: string;
	system_prompt_id?: string;
};

export type StreamChatEvent =
	| { type: 'message_started'; payload: { message: Message } }
	| { type: 'text_delta'; payload: { message_id: string; delta: string } }
	| { type: 'reasoning_delta'; payload: { message_id: string; delta: string } }
	| { type: 'message_completed'; payload: { message: Message } }
	| {
			type: 'message_failed';
			payload: { message_id: string; error: { code: string; message: string; retryable: boolean } };
	  };

export async function deleteMessagePair(threadId: string, messageId: string): Promise<void> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}/messages/${messageId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (response.status === 204) {
		return;
	}

	const payload = (await response.json().catch(() => null)) as { error?: string } | null;
	throw new Error(payload?.error ?? 'Failed to delete message pair.');
}

export async function forkThread(threadId: string, messageId: string): Promise<Thread> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}/fork`, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({ message_id: messageId })
	});
	const payload = (await response.json()) as Thread | { error?: string };

	if (!response.ok || !('id' in payload) || !('title' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to fork thread.');
	}

	return payload;
}

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

export async function updateThreadTitle(threadId: string, title: string): Promise<Thread> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}`, {
		method: 'PATCH',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({ title })
	});
	const payload = (await response.json()) as Thread | { error?: string };

	if (!response.ok || !('id' in payload) || !('title' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to rename thread.');
	}

	return payload;
}

export async function deleteThread(threadId: string): Promise<void> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (response.status === 204) {
		return;
	}

	const payload = (await response.json().catch(() => null)) as { error?: string } | null;
	throw new Error(payload?.error ?? 'Failed to delete thread.');
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

export async function streamChatCompletion(
	body: StreamChatPayload,
	apiKey: string,
	onEvent: (event: StreamChatEvent) => void
): Promise<void> {
	const response = await fetch(CHAT_API_ENDPOINT, {
		method: 'POST',
		headers: {
			authorization: `Bearer ${apiKey}`,
			'content-type': 'application/json'
		},
		body: JSON.stringify(body)
	});

	if (!response.ok || !response.body) {
		const payload = (await response.json().catch(() => null)) as { error?: string } | null;
		throw new Error(payload?.error ?? 'Failed to stream completion.');
	}

	const decoder = new TextDecoder();
	const reader = response.body.getReader();
	let buffer = '';

	while (true) {
		const { value, done } = await reader.read();
		if (done) {
			break;
		}
		buffer += decoder.decode(value, { stream: true });

		while (true) {
			const frame = popSseFrame(buffer);
			if (!frame) {
				break;
			}
			buffer = frame.rest;
			const event = parseStreamEvent(frame.frame);
			if (event) {
				onEvent(event);
			}
		}
	}
}

function popSseFrame(buffer: string): { frame: string; rest: string } | null {
	const crlfDelimiter = '\r\n\r\n';
	const lfDelimiter = '\n\n';
	const crlfIndex = buffer.indexOf(crlfDelimiter);
	const lfIndex = buffer.indexOf(lfDelimiter);
	const useCrlf = crlfIndex !== -1 && (lfIndex === -1 || crlfIndex < lfIndex);

	if (useCrlf) {
		return {
			frame: buffer.slice(0, crlfIndex),
			rest: buffer.slice(crlfIndex + crlfDelimiter.length)
		};
	}

	if (lfIndex !== -1) {
		return {
			frame: buffer.slice(0, lfIndex),
			rest: buffer.slice(lfIndex + lfDelimiter.length)
		};
	}

	return null;
}

function parseStreamEvent(frame: string): StreamChatEvent | null {
	let eventName = '';
	const dataLines: string[] = [];

	for (const line of frame.split(/\r?\n/)) {
		if (line.startsWith('event:')) {
			eventName = line.slice(6).trim();
		} else if (line.startsWith('data:')) {
			dataLines.push(line.slice(5).trimStart());
		}
	}

	if (!eventName || dataLines.length === 0) {
		return null;
	}

	try {
		const parsed = JSON.parse(dataLines.join('\n')) as { payload?: unknown };
		const payload = parsed.payload;
		if (!payload || typeof payload !== 'object') {
			return null;
		}

		return { type: eventName as StreamChatEvent['type'], payload: payload as never };
	} catch {
		return null;
	}
}
