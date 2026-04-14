import type { CopilotToken } from '$lib/types';

const COPILOT_TOKENS_API_BASE = '/api/v1/copilot-tokens';
const COPILOT_DEVICE_CODE_URL = '/api/v1/copilot/device-code';
const COPILOT_DEVICE_CODE_POLL_URL = '/api/v1/copilot/device-code/poll';

interface CopilotTokenPayload {
	id: string;
	name: string;
	github_token: string;
}

// ---------------------------------------------------------------------------
// List & Delete
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Device code flow
// ---------------------------------------------------------------------------

export interface DeviceCodeResponse {
	deviceCode: string;
	userCode: string;
	verificationUri: string;
	expiresIn: number;
	interval: number;
}

interface DeviceCodeApiPayload {
	device_code: string;
	user_code: string;
	verification_uri: string;
	expires_in: number;
	interval: number;
}

export type DevicePollStatus = 'pending' | 'slow_down' | 'complete' | 'expired';

export interface DevicePollResult {
	status: DevicePollStatus;
	token?: CopilotToken;
}

interface DevicePollApiPayload {
	status: DevicePollStatus;
	token?: CopilotTokenPayload;
}

export async function initiateDeviceCode(): Promise<DeviceCodeResponse> {
	const response = await fetch(COPILOT_DEVICE_CODE_URL, {
		method: 'POST',
		credentials: 'include'
	});

	const data = (await response.json()) as DeviceCodeApiPayload | { error?: string };

	if (!response.ok || !('device_code' in data)) {
		throw new Error(
			('error' in data && data.error) || 'Failed to start GitHub device authorization.'
		);
	}

	return {
		deviceCode: data.device_code,
		userCode: data.user_code,
		verificationUri: data.verification_uri,
		expiresIn: data.expires_in,
		interval: data.interval
	};
}

export async function pollDeviceCode(deviceCode: string, name: string): Promise<DevicePollResult> {
	const response = await fetch(COPILOT_DEVICE_CODE_POLL_URL, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({ device_code: deviceCode, name })
	});

	const data = (await response.json()) as DevicePollApiPayload | { error?: string };

	if (!response.ok || !('status' in data)) {
		throw new Error(('error' in data && data.error) || 'Failed to poll device authorization.');
	}

	return {
		status: data.status,
		token: data.token ? mapCopilotToken(data.token) : undefined
	};
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function mapCopilotToken(payload: CopilotTokenPayload): CopilotToken {
	return {
		id: payload.id,
		name: payload.name,
		githubToken: payload.github_token
	};
}
