import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    coverage: { reporter: ['json'] },
    environment: 'happy-dom',
  },
});
