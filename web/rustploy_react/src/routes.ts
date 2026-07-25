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
		route('projects/$id/app/$appId', 'projects/app-detail.tsx'),
		route('projects/$id/compose/$composeId', 'projects/compose-detail.tsx'),
		route('Deployments', 'deployments/index.tsx'),
		route('schedules', 'schedules.tsx'),
		route('docker', 'docker.tsx'),
		route('destinations', 'destinations.tsx'),
		route('ssh-keys', 'ssh-keys.tsx'),
		route('remote-servers', 'remote-servers.tsx'),
	]),
	// Auth Pages
	layout('auth/_auth.tsx', [
		route('singup', 'auth/singup.tsx'),
		route('singin', 'auth/singin.tsx'),
	]),
]);
