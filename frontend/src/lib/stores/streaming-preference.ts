import { browser } from '$app/environment';
import { writable } from 'svelte/store';

/** When true, assistant output is shown as it streams. Default matches legacy behavior. */
export const SHOW_ASSISTANT_STREAMING_TEXT_KEY = 'slopify-show-assistant-streaming-text';

function readStored(): boolean {
	if (!browser) return true;
	const raw = localStorage.getItem(SHOW_ASSISTANT_STREAMING_TEXT_KEY);
	if (raw === null) return true;
	return raw === 'true' || raw === '1';
}

function writeStored(value: boolean) {
	if (!browser) return;
	localStorage.setItem(SHOW_ASSISTANT_STREAMING_TEXT_KEY, value ? 'true' : 'false');
}

function createShowAssistantStreamingTextStore() {
	const initial = readStored();
	const { subscribe, set } = writable<boolean>(initial);

	return {
		subscribe,
		init: () => {
			set(readStored());
		},
		setShowAssistantStreamingText: (value: boolean) => {
			writeStored(value);
			set(value);
		}
	};
}

export const showAssistantStreamingText = createShowAssistantStreamingTextStore();
