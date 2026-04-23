/**
 * In-memory per-thread draft store.
 * Not persisted — lives only as long as the page session.
 */
const drafts = new Map<string, string>();

export function getDraft(threadId: string): string {
	return drafts.get(threadId) ?? '';
}

export function setDraft(threadId: string, value: string): void {
	if (value) {
		drafts.set(threadId, value);
	} else {
		drafts.delete(threadId);
	}
}
