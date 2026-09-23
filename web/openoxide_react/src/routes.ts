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
		// Disabled: overview duplicates the home dashboard (see sidebar enum.ts).
		route('projects', 'projects/index.tsx'),
		route('projects/$id', 'projects/detail.tsx'),
		route('projects/$id/create', 'projects/create.tsx'),
		route('projects/$id/stack', 'projects/stack.tsx'),
		route('projects/$id/app/$appId', 'projects/app-detail.tsx'),
		route(
			'projects/$id/compose/$composeId',
			'projects/compose-detail.tsx',
		),
		route('projects/$id/database/$dbId', 'projects/database-detail.tsx'),
		route('Deployments', 'deployments/index.tsx'),
		route('Deployments/queue', 'deployments/queue.tsx'),
		// Removed: global Monitoring page — each server now has its own overview.
		route('schedules', 'schedules.tsx'),
		// Disabled: host-wide Docker view — org context me expose nahi karna (see sidebar enum.ts).
		route('destinations', 'destinations.tsx'),
		route('ssh-keys', 'ssh-keys.tsx'),
		route('git-providers', 'git-providers.tsx'),
		route('certificates', 'certificates.tsx'),
		route('ai', 'ai.tsx'),
		route('remote-servers', 'remote-servers.tsx'),
		route('remote-servers/new', 'remote-servers/new.tsx'),
		route('remote-servers/$id', 'remote-servers/detail.tsx'),
		// Web Server / Traefik / Swarm / Requests are underline tabs of the single infra page (?tab= deep links).
		route('infra', 'infra.tsx'),
		route('registry', 'registry.tsx'),
		route('vault', 'vault.tsx'),
		route('dns', 'dns.tsx'),
		route('tags', 'tags.tsx'),
		route('users', 'users.tsx'),
		route('settings', 'settings/index.tsx'),
		route('settings/audit-logs', 'settings/audit-logs.tsx'),
		route('settings/groups', 'settings/groups.tsx'),
		route('settings/notifications', 'settings/notifications.tsx'),
		route('settings/users', 'settings/users.tsx'),
		route('accept-invite', 'accept-invite.tsx'),
		route('profile', 'profile.tsx'),
		route('settings/profile', 'settings/profile.tsx'),
	]),
	// Auth Pages
	layout('auth/_auth.tsx', [
		route('singup', 'auth/singup.tsx'),
		route('singin', 'auth/singin.tsx'),
	]),
]);
