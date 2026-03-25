import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite-plus';
import { sveltePhosphorOptimize } from 'phosphor-svelte/vite';

export default defineConfig({
	server: {
		port: 5177
	},
	lint: { options: { typeAware: true, typeCheck: true } },
	plugins: [tailwindcss(), sveltekit(), sveltePhosphorOptimize()]
});
