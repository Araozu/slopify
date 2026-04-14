import type { CopilotToken } from '$lib/types';

const COPILOT_TOKENS_API_BASE = '/api/v1/copilot-tokens';

interface CopilotTokenPayload {
	id: string;
	name: string;
	github_token: string;
}

interface CopilotTokenMutationPayload {
	name: string;
	githubToken: string;
}

export async function listCopilotTokens(signal?: AbortSignal): Promise<CopilotToken[]> {
	const response = await fetch(COPILOT_TOKENS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as CopilotTokenPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load Copilot tokens.');
	}

	return payload.map(mapCopilotToken);
}

export async function createCopilotToken(
	payload: CopilotTokenMutationPayload
): Promise<CopilotToken> {
	const response = await fetch(COPILOT_TOKENS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			name: payload.name,
			github_token: payload.githubToken
		})
	});
	const data = (await response.json()) as CopilotTokenPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save Copilot token.');
	}

	return mapCopilotToken(data);
}

export async function updateCopilotToken(
	tokenId: string,
	payload: Partial<CopilotTokenMutationPayload>
): Promise<CopilotToken> {
	const response = await fetch(`${COPILOT_TOKENS_API_BASE}/${tokenId}`, {
		method: 'PATCH',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			name: payload.name,
			github_token: payload.githubToken
		})
	});
	const data = (await response.json()) as CopilotTokenPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to update Copilot token.');
	}

	return mapCopilotToken(data);
}

export async function deleteCopilotToken(tokenId: string): Promise<void> {
	const response = await fetch(`${COPILOT_TOKENS_API_BASE}/${tokenId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete Copilot token.');
	}
}

function mapCopilotToken(payload: CopilotTokenPayload): CopilotToken {
	return {
		id: payload.id,
		name: payload.name,
		githubToken: payload.github_token
	};
}
