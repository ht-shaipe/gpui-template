import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [wasm()],
  build: {
    target: 'esnext',
    minify: 'esbuild',
    sourcemap: false,
  },
  server: {
    port: 3000,
    open: true,
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
    proxy: {
      // Proxy gpui-component icon assets through the dev server to avoid
      // COEP/CORS issues when loading from an external CDN on localhost.
      '/gpui-component/gallery/assets': {
        target: 'https://longbridge.github.io',
        changeOrigin: true,
        headers: {
          'Cross-Origin-Resource-Policy': 'cross-origin',
        },
      },
    },
  },
  optimizeDeps: {
    exclude: ['./src/wasm'],
  },
  base: '/',
});
