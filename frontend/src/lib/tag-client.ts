import type { Tag } from '$lib/types';

const TAGS_API_ENDPOINT = '/api/v1/tags';
const THREADS_API_ENDPOINT = '/api/v1/threads';

export async function listTags(): Promise<Tag[]> {
	const response = await fetch(TAGS_API_ENDPOINT, { credentials: 'include' });
	const payload = (await response.json()) as Tag[] | { error?: string };
	if (!response.ok || !Array.isArray(payload)) {
		throw new Error((!Array.isArray(payload) && payload.error) || 'Failed to load tags.');
	}
	return payload;
}

export async function createTag(name: string, color: string): Promise<Tag> {
	const response = await fetch(TAGS_API_ENDPOINT, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({ name, color })
	});
	const payload = (await response.json()) as Tag | { error?: string };
	if (!response.ok || !('id' in payload)) {
		throw new Error(('error' in payload && payload.error) || 'Failed to create tag.');
	}
	return payload;
}

export async function deleteTag(tagId: string): Promise<void> {
	const response = await fetch(`${TAGS_API_ENDPOINT}/${tagId}`, {
		method: 'DELETE',
		credentials: 'include'
	});
	if (response.status === 204) return;
	const payload = (await response.json().catch(() => null)) as { error?: string } | null;
	throw new Error(payload?.error ?? 'Failed to delete tag.');
}

export async function addTagToThread(threadId: string, tagId: string): Promise<void> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}/tags`, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({ tag_id: tagId })
	});
	if (response.status === 204) return;
	const payload = (await response.json().catch(() => null)) as { error?: string } | null;
	throw new Error(payload?.error ?? 'Failed to add tag to thread.');
}

export async function removeTagFromThread(threadId: string, tagId: string): Promise<void> {
	const response = await fetch(`${THREADS_API_ENDPOINT}/${threadId}/tags/${tagId}`, {
		method: 'DELETE',
		credentials: 'include'
	});
	if (response.status === 204) return;
	const payload = (await response.json().catch(() => null)) as { error?: string } | null;
	throw new Error(payload?.error ?? 'Failed to remove tag from thread.');
}
