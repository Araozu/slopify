import type { SystemPrompt } from '$lib/types';

const SYSTEM_PROMPTS_API_BASE = '/api/v1/system-prompts';

interface SystemPromptPayload {
	id: string;
	name: string;
	content: string;
}

interface SystemPromptMutationPayload {
	name: string;
	content: string;
}

export async function listSystemPrompts(signal?: AbortSignal): Promise<SystemPrompt[]> {
	const response = await fetch(SYSTEM_PROMPTS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as SystemPromptPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load system prompts.');
	}

	return payload;
}

export async function createSystemPrompt(
	payload: SystemPromptMutationPayload
): Promise<SystemPrompt> {
	const response = await fetch(SYSTEM_PROMPTS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({ name: payload.name, content: payload.content })
	});
	const data = (await response.json()) as SystemPromptPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save system prompt.');
	}

	return data;
}

export async function updateSystemPrompt(
	promptId: string,
	payload: Partial<SystemPromptMutationPayload>
): Promise<SystemPrompt> {
	const response = await fetch(`${SYSTEM_PROMPTS_API_BASE}/${promptId}`, {
		method: 'PATCH',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({ name: payload.name, content: payload.content })
	});
	const data = (await response.json()) as SystemPromptPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to update system prompt.');
	}

	return data;
}

export async function deleteSystemPrompt(promptId: string): Promise<void> {
	const response = await fetch(`${SYSTEM_PROMPTS_API_BASE}/${promptId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete system prompt.');
	}
}
