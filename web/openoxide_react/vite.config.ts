import tailwindcss from '@tailwindcss/vite';
import {devtools} from '@tanstack/devtools-vite';
import {tanstackRouter} from '@tanstack/router-plugin/vite';
import viteReact, {reactCompilerPreset} from '@vitejs/plugin-react';
import babel from '@rolldown/plugin-babel';
import {defineConfig, type Plugin} from 'vite';
import {openoxide} from '@openoxide/vite';
import {routes} from './src/routes';

// Cookies are shared by every local app using the same hostname. Keep the
// development server tolerant of an accumulated localhost cookie jar.
const DEV_MAX_HEADER_SIZE = 1024 * 1024;

const devHeaderLimit = (): Plugin => ({
	name: 'openoxide-dev-header-limit',
	configureServer(server) {
		if (server.httpServer) {
			const httpServer = server.httpServer as typeof server.httpServer & {
				maxHeaderSize: number;
			};
			httpServer.maxHeaderSize = DEV_MAX_HEADER_SIZE;
		}
	},
});

const config = defineConfig({
	resolve: {tsconfigPaths: true},
	plugins: [
		devHeaderLimit(),
		openoxide({
			manifestPath: '../../Cargo.toml',
			manifestBin: 'openoxide-live-manifest',
			declarations: 'src/openoxide-live.generated.d.ts',
		}),
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
		// host: '0.0.0.0',
		host: 'localhost',
		port: 3001,
		strictPort: false,
		allowedHosts: true,
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:4000',
				changeOrigin: true,
				secure: false,
				rewrite: path => path.replace(/^\/api/, ''),
			},
			'/socket.io': {
				target: 'http://127.0.0.1:4000',
				ws: true,
				changeOrigin: true,
				secure: false,
			},
		},
	},
});

export default config;
