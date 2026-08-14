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
		route('projects/$id/stack', 'projects/stack.tsx'),
		route('projects/$id/app/$appId', 'projects/app-detail.tsx'),
		route('projects/$id/compose/$composeId', 'projects/compose-detail.tsx'),
		route('projects/$id/database/$dbId', 'projects/database-detail.tsx'),
		route('Deployments', 'deployments/index.tsx'),
		route('monitoring', 'monitoring.tsx'),
		route('schedules', 'schedules.tsx'),
		route('docker', 'docker.tsx'),
		route('destinations', 'destinations.tsx'),
		route('ssh-keys', 'ssh-keys.tsx'),
		route('remote-servers', 'remote-servers.tsx'),
		route('swarm', 'swarm.tsx'),
		route('registry', 'registry.tsx'),
		route('vault', 'vault.tsx'),
		route('dns', 'dns.tsx'),
		route('traefik', 'traefik.tsx'),
		route('requests', 'requests.tsx'),
		route('tags', 'tags.tsx'),
		route('users', 'users.tsx'),
		route('settings/users', 'settings/users.tsx'),
		route('profile', 'profile.tsx'),
		route('settings/profile', 'settings/profile.tsx'),
	]),
	// Auth Pages
	layout('auth/_auth.tsx', [
		route('singup', 'auth/singup.tsx'),
		route('singin', 'auth/singin.tsx'),
	]),
]);
