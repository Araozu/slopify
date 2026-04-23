import { browser } from '$app/environment';
import { writable } from 'svelte/store';

// ── Message font size ────────────────────────────────────────────────────────

export type MessageFontSize = 'sm' | 'md' | 'lg';

const MESSAGE_FONT_SIZE_KEY = 'slopify-message-font-size';

function readStoredFontSize(): MessageFontSize {
	if (!browser) return 'sm';
	const raw = localStorage.getItem(MESSAGE_FONT_SIZE_KEY);
	if (raw === 'sm' || raw === 'md' || raw === 'lg') return raw;
	return 'sm';
}

function writeStoredFontSize(value: MessageFontSize) {
	if (!browser) return;
	localStorage.setItem(MESSAGE_FONT_SIZE_KEY, value);
}

function createMessageFontSizeStore() {
	const { subscribe, set } = writable<MessageFontSize>(readStoredFontSize());

	return {
		subscribe,
		init: () => set(readStoredFontSize()),
		set: (value: MessageFontSize) => {
			writeStoredFontSize(value);
			set(value);
		}
	};
}

export const messageFontSize = createMessageFontSizeStore();

// ── Content width ────────────────────────────────────────────────────────────

export type ContentWidth = 'default' | 'wide' | 'full';

const CONTENT_WIDTH_KEY = 'slopify-content-width';

function readStoredContentWidth(): ContentWidth {
	if (!browser) return 'default';
	const raw = localStorage.getItem(CONTENT_WIDTH_KEY);
	if (raw === 'default' || raw === 'wide' || raw === 'full') return raw;
	return 'default';
}

function writeStoredContentWidth(value: ContentWidth) {
	if (!browser) return;
	localStorage.setItem(CONTENT_WIDTH_KEY, value);
}

function createContentWidthStore() {
	const { subscribe, set } = writable<ContentWidth>(readStoredContentWidth());

	return {
		subscribe,
		init: () => set(readStoredContentWidth()),
		set: (value: ContentWidth) => {
			writeStoredContentWidth(value);
			set(value);
		}
	};
}

export const contentWidth = createContentWidthStore();
