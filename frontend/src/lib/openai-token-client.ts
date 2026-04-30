import type { OpenAiAuthType, OpenAiToken } from '$lib/types';

const OPENAI_TOKENS_API_BASE = '/api/v1/openai-tokens';
const OPENAI_DEVICE_CODE_URL = '/api/v1/openai/device-code';
const OPENAI_DEVICE_CODE_POLL_URL = '/api/v1/openai/device-code/poll';

interface OpenAiTokenPayload {
	id: string;
	name: string;
	auth_type: OpenAiAuthType;
	token: string;
}

interface OpenAiTokenCreatePayload {
	name: string;
	token: string;
}

export async function listOpenAiTokens(signal?: AbortSignal): Promise<OpenAiToken[]> {
	const response = await fetch(OPENAI_TOKENS_API_BASE, {
		signal,
		credentials: 'include'
	});
	const payload = (await response.json()) as OpenAiTokenPayload[] | { error?: string };

	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load OpenAI tokens.');
	}

	return payload.map(mapOpenAiToken);
}

export async function createOpenAiToken(payload: OpenAiTokenCreatePayload): Promise<OpenAiToken> {
	const response = await fetch(OPENAI_TOKENS_API_BASE, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			name: payload.name,
			token: payload.token
		})
	});
	const data = (await response.json()) as OpenAiTokenPayload | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to save OpenAI token.');
	}

	return mapOpenAiToken(data);
}

export async function deleteOpenAiToken(tokenId: string): Promise<void> {
	const response = await fetch(`${OPENAI_TOKENS_API_BASE}/${tokenId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to delete OpenAI token.');
	}
}

export interface OpenAiDeviceCodeResponse {
	deviceAuthId: string;
	userCode: string;
	verificationUri: string;
	expiresIn: number;
	interval: number;
}

interface OpenAiDeviceCodeApiPayload {
	device_auth_id: string;
	user_code: string;
	verification_uri: string;
	expires_in: number;
	interval: number;
}

export type OpenAiDevicePollStatus = 'pending' | 'slow_down' | 'complete' | 'expired';

export interface OpenAiDevicePollResult {
	status: OpenAiDevicePollStatus;
	token?: OpenAiToken;
}

interface OpenAiDevicePollApiPayload {
	status: OpenAiDevicePollStatus;
	token?: OpenAiTokenPayload;
}

export async function initiateOpenAiDeviceCode(): Promise<OpenAiDeviceCodeResponse> {
	const response = await fetch(OPENAI_DEVICE_CODE_URL, {
		method: 'POST',
		credentials: 'include'
	});

	const data = (await response.json()) as OpenAiDeviceCodeApiPayload | { error?: string };

	if (!response.ok || !('device_auth_id' in data)) {
		throw new Error(
			('error' in data && data.error) || 'Failed to start OpenAI device authorization.'
		);
	}

	return {
		deviceAuthId: data.device_auth_id,
		userCode: data.user_code,
		verificationUri: data.verification_uri,
		expiresIn: data.expires_in,
		interval: data.interval
	};
}

export async function pollOpenAiDeviceCode(
	deviceAuthId: string,
	userCode: string,
	name: string
): Promise<OpenAiDevicePollResult> {
	const response = await fetch(OPENAI_DEVICE_CODE_POLL_URL, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({
			device_auth_id: deviceAuthId,
			user_code: userCode,
			name
		})
	});

	const data = (await response.json()) as OpenAiDevicePollApiPayload | { error?: string };

	if (!response.ok || !('status' in data)) {
		throw new Error(
			('error' in data && data.error) || 'Failed to poll OpenAI device authorization.'
		);
	}

	return {
		status: data.status,
		token: data.token ? mapOpenAiToken(data.token) : undefined
	};
}

function mapOpenAiToken(payload: OpenAiTokenPayload): OpenAiToken {
	return {
		id: payload.id,
		name: payload.name,
		authType: payload.auth_type,
		token: payload.token
	};
}
