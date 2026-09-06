import { defineConfig } from "vite";

// Vite settings Tauri needs: a fixed dev port it can point `devUrl` at,
// and no screen clearing so Rust's compiler output stays readable.
export default defineConfig({
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
  build: {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
  },
});
