import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import * as path from 'path';

/// <reference types="vitest" />
export default defineConfig({
  plugins: [svelte()],
  base: './',
  test: {
    root: path.resolve(__dirname, './src'),
  },
})
