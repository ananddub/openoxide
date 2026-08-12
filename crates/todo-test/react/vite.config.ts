import react from '@vitejs/plugin-react';
import {defineConfig} from 'vite';
import {openoxide} from '@openoxide/vite';

export default defineConfig({
  plugins: [
    react(),
    openoxide({
      manifestPath: '../Cargo.toml',
      declarations: 'src/openoxide-live.generated.d.ts',
    }),
  ],
  server: {
    proxy: {
      '/api': 'http://127.0.0.1:3100',
      '/socket.io': {target: 'http://127.0.0.1:3100', ws: true},
    },
  },
});
