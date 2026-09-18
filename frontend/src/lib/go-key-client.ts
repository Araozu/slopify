import type { GoApiKey } from '$lib/types';

const GO_KEYS_API_BASE = '/api/v1/go-keys';

interface GoApiKeyPayload {
	id: string;
	name: string;
	api_key: string;
}

interface GoApiKeyMutationPayload {
	name: string;
	apiKey: string;
}

export async function listGoKeys(signal?: AbortSignal): Promise<GoApiKey[]> {
	const response = await fetch(GO_KEYS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as GoApiKeyPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load Go keys.');
	}

	return payload.map(mapGoApiKey);
}

export async function createGoKey(payload: GoApiKeyMutationPayload): Promise<GoApiKey> {
	const response = await fetch(GO_KEYS_API_BASE, {
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
	const data = (await response.json()) as GoApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save Go key.');
	}

	return mapGoApiKey(data);
}

export async function updateGoKey(
	keyId: string,
	payload: Partial<GoApiKeyMutationPayload>
): Promise<GoApiKey> {
	const response = await fetch(`${GO_KEYS_API_BASE}/${keyId}`, {
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
	const data = (await response.json()) as GoApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to update Go key.');
	}

	return mapGoApiKey(data);
}

export async function deleteGoKey(keyId: string): Promise<void> {
	const response = await fetch(`${GO_KEYS_API_BASE}/${keyId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete Go key.');
	}
}

function mapGoApiKey(payload: GoApiKeyPayload): GoApiKey {
	return {
		id: payload.id,
		name: payload.name,
		apiKey: payload.api_key
	};
}
