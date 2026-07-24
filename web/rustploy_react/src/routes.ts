import {
	index,
	layout,
	rootRoute,
	route,
} from '@tanstack/virtual-file-routes';

export const routes = rootRoute('_root.tsx', [
	// App Pages
	layout('_app.tsx', [
		index('index.tsx'),
		route('projects', 'projects/index.tsx'),
		route('projects/$id', 'projects/detail.tsx'),
		route('Deployments', 'deployments.tsx'),
		route('schedules', 'schedules.tsx'),
	]),
	// Auth Pages
	layout('auth/_auth.tsx', [
		route('singup', 'auth/singup.tsx'),
		route('singin', 'auth/singin.tsx'),
	]),
]);
