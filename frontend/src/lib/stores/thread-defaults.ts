import { browser } from '$app/environment';
import { writable } from 'svelte/store';

export const THREAD_DEFAULTS_STORAGE_KEY = 'slopify-thread-defaults';

export interface ThreadDefaultsValue {
	credentialId: string | null;
	systemPromptId: string | null;
	model: string;
}

const EMPTY_DEFAULTS: ThreadDefaultsValue = {
	credentialId: null,
	systemPromptId: null,
	model: ''
};

function readStored(): ThreadDefaultsValue {
	if (!browser) return EMPTY_DEFAULTS;
	try {
		const raw = localStorage.getItem(THREAD_DEFAULTS_STORAGE_KEY);
		if (!raw) return EMPTY_DEFAULTS;
		const parsed = JSON.parse(raw) as Partial<ThreadDefaultsValue>;
		return {
			credentialId: typeof parsed.credentialId === 'string' ? parsed.credentialId : null,
			systemPromptId: typeof parsed.systemPromptId === 'string' ? parsed.systemPromptId : null,
			model: typeof parsed.model === 'string' ? parsed.model : ''
		};
	} catch {
		return EMPTY_DEFAULTS;
	}
}

function writeStored(value: ThreadDefaultsValue) {
	if (!browser) return;
	localStorage.setItem(THREAD_DEFAULTS_STORAGE_KEY, JSON.stringify(value));
}

function createThreadDefaultsStore() {
	const initial = readStored();
	const { subscribe, set, update } = writable<ThreadDefaultsValue>(initial);

	return {
		subscribe,
		init: () => {
			set(readStored());
		},
		setCredentialId: (credentialId: string | null) => {
			update((current) => {
				const next = { ...current, credentialId };
				writeStored(next);
				return next;
			});
		},
		setSystemPromptId: (systemPromptId: string | null) => {
			update((current) => {
				const next = { ...current, systemPromptId };
				writeStored(next);
				return next;
			});
		},
		setModel: (model: string) => {
			update((current) => {
				const next = { ...current, model };
				writeStored(next);
				return next;
			});
		}
	};
}

export const threadDefaults = createThreadDefaultsStore();
