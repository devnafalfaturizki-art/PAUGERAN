import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import path from 'path';

export default defineConfig({
  plugins: [solidPlugin()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@paugeran/shared': path.resolve(__dirname, '../../packages/shared/src'),
    },
    conditions: ['development', 'browser'],
  },
  server: {
    port: 5173,
    proxy: { '/api': 'http://127.0.0.1:3000' },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    sourcemap: false,
  },
});
