import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import wasmPackWatchPlugin from "vite-plugin-wasm-pack-watcher";
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  root: "app",
  build: {
    watch: {
      include: ["src/**/*.ts", "src/**/*.svelte", "../wasm/src/**/*.rs", "../core/src/**/*.rs"],
    },
  },
  plugins: [svelte(), wasmPackWatchPlugin(), wasm(), topLevelAwait()],
});
