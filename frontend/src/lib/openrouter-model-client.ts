import type { OpenRouterModel } from '$lib/types';

const OPENROUTER_MODELS_API_BASE = '/api/v1/openrouter-models';

interface OpenRouterModelPayload {
	id: string;
	model_id: string;
}

export async function listOpenRouterModels(signal?: AbortSignal): Promise<OpenRouterModel[]> {
	const response = await fetch(OPENROUTER_MODELS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as OpenRouterModelPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error(
			(!Array.isArray(payload) && payload.error) || 'Failed to load OpenRouter models.'
		);
	}

	return payload.map(mapOpenRouterModel);
}

export async function createOpenRouterModel(modelId: string): Promise<OpenRouterModel> {
	const response = await fetch(OPENROUTER_MODELS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			model_id: modelId
		})
	});
	const data = (await response.json()) as OpenRouterModelPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save OpenRouter model.');
	}

	return mapOpenRouterModel(data);
}

export async function deleteOpenRouterModel(id: string): Promise<void> {
	const response = await fetch(`${OPENROUTER_MODELS_API_BASE}/${id}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete OpenRouter model.');
	}
}

function mapOpenRouterModel(payload: OpenRouterModelPayload): OpenRouterModel {
	return {
		id: payload.id,
		modelId: payload.model_id
	};
}
