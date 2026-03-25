import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite-plus";

export default defineConfig({
  staged: {
    "*": "vp check --fix"
  },
  lint: { options: { typeAware: true, typeCheck: true } },
  plugins: [tailwindcss(), sveltekit()],
});
