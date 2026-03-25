import { browser } from '$app/environment';
import {
	setTheme as setModeWatcherTheme,
	themeStorageKey as modeWatcherThemeStorageKey
} from 'mode-watcher';
import { writable } from 'svelte/store';

export const themes = [
	{ id: 'corporate-memo', name: 'Corporate Memo' },
	{ id: 'forest-ripple', name: 'Forest Ripple' },
	{ id: 'wishful-aurosa', name: 'Wishful Aurosa' },
	{ id: 'golden-plume', name: 'Golden Plume' },
	{ id: 'blazing-fields', name: 'Blazing Fields' },
	{ id: 'crimson-shade', name: 'Crimson Shade' },
	{ id: 'enchanted-encounter', name: 'Enchanted Encounter' },
	{ id: 'twilight-echoes', name: 'Twilight Echoes' },
	{ id: 'spectral-mist', name: 'Spectral Mist' },
	{ id: 'misty-dream', name: 'Misty Dream' }
] as const;

export type ThemeId = (typeof themes)[number]['id'];

export const DEFAULT_THEME: ThemeId = 'corporate-memo';
export const THEME_STORAGE_KEY = 'slopify-theme';

modeWatcherThemeStorageKey.current = THEME_STORAGE_KEY;

function isThemeId(value: string): value is ThemeId {
	return themes.some((theme) => theme.id === value);
}

function getStoredTheme(): ThemeId | null {
	if (!browser) return null;

	const storedTheme = localStorage.getItem(THEME_STORAGE_KEY);
	return storedTheme && isThemeId(storedTheme) ? storedTheme : null;
}

function applyTheme(theme: ThemeId) {
	if (!browser) return;
	setModeWatcherTheme(theme);
}

function createThemeStore() {
	const initialTheme = getStoredTheme() ?? DEFAULT_THEME;
	const { subscribe, set } = writable<ThemeId>(initialTheme);

	applyTheme(initialTheme);

	return {
		subscribe,
		init: () => {
			const storedTheme = getStoredTheme() ?? DEFAULT_THEME;
			set(storedTheme);
			applyTheme(storedTheme);
		},
		setTheme: (theme: ThemeId) => {
			if (!browser) return;
			set(theme);
			applyTheme(theme);
		}
	};
}

export const theme = createThemeStore();
