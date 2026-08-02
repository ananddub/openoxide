import tailwindcss from '@tailwindcss/vite';
import {devtools} from '@tanstack/devtools-vite';
import {tanstackRouter} from '@tanstack/router-plugin/vite';
import viteReact, {reactCompilerPreset} from '@vitejs/plugin-react';
import babel from '@rolldown/plugin-babel';
import {defineConfig} from 'vite';
import {routes} from './src/routes';

const config = defineConfig({
	resolve: {tsconfigPaths: true},
	plugins: [
		devtools(),
		tailwindcss(),
		tanstackRouter({
			target: 'react',
			autoCodeSplitting: true,
			routesDirectory: './src/pages',
			virtualRouteConfig: routes,
		}),
		viteReact(),
		babel({presets: [reactCompilerPreset()]}),
	],
	server: {
		host: '0.0.0.0',
		// host:"localhost",
		port: 3001,
		strictPort: false,
		allowedHosts: true,
		proxy: {
			'/api': {
				target: 'http://das.tail25b5a0.ts.net:4000',
				changeOrigin: true,
				secure: false,
				rewrite: path => path.replace(/^\/api/, ''),
			},
			'/socket.io': {
				target: 'http://das.tail25b5a0.ts.net:4000',
				ws: true,
				changeOrigin: true,
				secure: false,
			},
		},
	},
});

export default config;
