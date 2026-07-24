import {
	index,
	layout,
	rootRoute,
	route,
} from '@tanstack/virtual-file-routes';

export const routes = rootRoute('_root.tsx', [
	// App Pages
	layout('_app.tsx', [index('index.tsx')]),
	// Auth Pages
	layout('auth/_auth.tsx', [
		route('singup', 'auth/singup.tsx'),
		route('singin', 'auth/singin.tsx'),
	]),
]);
