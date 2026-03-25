import { browser } from '$app/environment';
import { writable } from 'svelte/store';

export const themes = [
	{ id: 'corporate-memo', name: 'Corporate Memo' },
	{ id: 'wishful-aurosa', name: 'Wishful Aurosa' },
	{ id: 'misty-dream', name: 'Misty Dream' },
	{ id: 'twilight-echoes', name: 'Twilight Echoes' },
	{ id: 'crimson-shade', name: 'Crimson Shade' }
] as const;

export type ThemeId = (typeof themes)[number]['id'];

const DEFAULT_THEME: ThemeId = 'corporate-memo';

function createThemeStore() {
	const { subscribe, set } = writable<ThemeId>(DEFAULT_THEME);

	return {
		subscribe,
		init: () => {
			if (!browser) return;
			const stored = localStorage.getItem('slopify-theme') as ThemeId;
			if (stored && themes.some((t) => t.id === stored)) {
				set(stored);
				document.documentElement.setAttribute('data-theme', stored);
			} else {
				document.documentElement.setAttribute('data-theme', DEFAULT_THEME);
			}
		},
		setTheme: (theme: ThemeId) => {
			if (!browser) return;
			set(theme);
			localStorage.setItem('slopify-theme', theme);
			document.documentElement.setAttribute('data-theme', theme);
		}
	};
}

export const theme = createThemeStore();
