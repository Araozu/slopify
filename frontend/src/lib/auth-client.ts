import type { AuthUser } from '$lib/types';

const AUTH_API_BASE = '/api/v1/auth';

interface AuthPayload {
	email: string;
	password: string;
	name?: string;
}

export async function getCurrentUser(fetcher: typeof fetch = fetch): Promise<AuthUser> {
	const response = await fetcher(`${AUTH_API_BASE}/me`, {
		credentials: 'include'
	});
	const payload = (await response.json()) as AuthUser | { error?: string };

	if (!response.ok || !('id' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to load your session.');
	}

	return payload;
}

export async function registerUser(payload: AuthPayload): Promise<AuthUser> {
	return authenticate(`${AUTH_API_BASE}/register`, payload);
}

export async function loginUser(payload: AuthPayload): Promise<AuthUser> {
	return authenticate(`${AUTH_API_BASE}/login`, payload);
}

export async function logoutUser(): Promise<void> {
	const response = await fetch(`${AUTH_API_BASE}/logout`, {
		method: 'POST',
		credentials: 'include'
	});

	if (!response.ok && response.status !== 204) {
		const payload = (await response.json()) as { error?: string };
		throw new Error(payload.error || 'Failed to end your session.');
	}
}

async function authenticate(url: string, payload: AuthPayload): Promise<AuthUser> {
	const response = await fetch(url, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(payload)
	});
	const data = (await response.json()) as AuthUser | { error?: string };

	if (!response.ok || !('id' in data)) {
		throw new Error(('error' in data && data.error) || 'Authentication failed.');
	}

	return data;
}
