import type { CopilotModel } from '$lib/types';

const COPILOT_MODELS_API_BASE = '/api/v1/copilot-models';

interface CopilotModelPayload {
	id: string;
	model_id: string;
}

export async function listCopilotModels(signal?: AbortSignal): Promise<CopilotModel[]> {
	const response = await fetch(COPILOT_MODELS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as CopilotModelPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load Copilot models.');
	}

	return payload.map(mapCopilotModel);
}

export async function createCopilotModel(modelId: string): Promise<CopilotModel> {
	const response = await fetch(COPILOT_MODELS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			model_id: modelId
		})
	});
	const data = (await response.json()) as CopilotModelPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save Copilot model.');
	}

	return mapCopilotModel(data);
}

export async function deleteCopilotModel(id: string): Promise<void> {
	const response = await fetch(`${COPILOT_MODELS_API_BASE}/${id}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete Copilot model.');
	}
}

function mapCopilotModel(payload: CopilotModelPayload): CopilotModel {
	return {
		id: payload.id,
		modelId: payload.model_id
	};
}
