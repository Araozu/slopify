import type { OpenRouterApiKey } from '$lib/types';

const OPENROUTER_KEYS_API_BASE = '/api/v1/openrouter-keys';

interface OpenRouterApiKeyPayload {
	id: string;
	name: string;
	api_key: string;
}

interface OpenRouterApiKeyMutationPayload {
	name: string;
	apiKey: string;
}

export async function listOpenRouterKeys(signal?: AbortSignal): Promise<OpenRouterApiKey[]> {
	const response = await fetch(OPENROUTER_KEYS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as OpenRouterApiKeyPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error(
			(!Array.isArray(payload) && payload.error) || 'Failed to load OpenRouter keys.'
		);
	}

	return payload.map(mapOpenRouterApiKey);
}

export async function createOpenRouterKey(
	payload: OpenRouterApiKeyMutationPayload
): Promise<OpenRouterApiKey> {
	const response = await fetch(OPENROUTER_KEYS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			name: payload.name,
			api_key: payload.apiKey
		})
	});
	const data = (await response.json()) as OpenRouterApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save OpenRouter key.');
	}

	return mapOpenRouterApiKey(data);
}

export async function updateOpenRouterKey(
	keyId: string,
	payload: Partial<OpenRouterApiKeyMutationPayload>
): Promise<OpenRouterApiKey> {
	const response = await fetch(`${OPENROUTER_KEYS_API_BASE}/${keyId}`, {
		method: 'PATCH',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			name: payload.name,
			api_key: payload.apiKey
		})
	});
	const data = (await response.json()) as OpenRouterApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to update OpenRouter key.');
	}

	return mapOpenRouterApiKey(data);
}

export async function deleteOpenRouterKey(keyId: string): Promise<void> {
	const response = await fetch(`${OPENROUTER_KEYS_API_BASE}/${keyId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete OpenRouter key.');
	}
}

function mapOpenRouterApiKey(payload: OpenRouterApiKeyPayload): OpenRouterApiKey {
	return {
		id: payload.id,
		name: payload.name,
		apiKey: payload.api_key
	};
}
