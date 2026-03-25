import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite-plus';
import { sveltePhosphorOptimize } from 'phosphor-svelte/vite';

export default defineConfig({
	lint: { options: { typeAware: true, typeCheck: true } },
	plugins: [tailwindcss(), sveltekit(), sveltePhosphorOptimize()]
});
