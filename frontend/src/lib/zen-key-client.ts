import type { ZenApiKey } from '$lib/types';

const ZEN_KEYS_API_BASE = '/api/v1/zen-keys';

interface ZenApiKeyPayload {
	id: string;
	name: string;
	api_key: string;
}

interface ZenApiKeyMutationPayload {
	name: string;
	apiKey: string;
}

export async function listZenKeys(signal?: AbortSignal): Promise<ZenApiKey[]> {
	const response = await fetch(ZEN_KEYS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as ZenApiKeyPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load Zen keys.');
	}

	return payload.map(mapZenApiKey);
}

export async function createZenKey(payload: ZenApiKeyMutationPayload): Promise<ZenApiKey> {
	const response = await fetch(ZEN_KEYS_API_BASE, {
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
	const data = (await response.json()) as ZenApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save Zen key.');
	}

	return mapZenApiKey(data);
}

export async function updateZenKey(
	keyId: string,
	payload: Partial<ZenApiKeyMutationPayload>
): Promise<ZenApiKey> {
	const response = await fetch(`${ZEN_KEYS_API_BASE}/${keyId}`, {
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
	const data = (await response.json()) as ZenApiKeyPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to update Zen key.');
	}

	return mapZenApiKey(data);
}

export async function deleteZenKey(keyId: string): Promise<void> {
	const response = await fetch(`${ZEN_KEYS_API_BASE}/${keyId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete Zen key.');
	}
}

function mapZenApiKey(payload: ZenApiKeyPayload): ZenApiKey {
	return {
		id: payload.id,
		name: payload.name,
		apiKey: payload.api_key
	};
}
