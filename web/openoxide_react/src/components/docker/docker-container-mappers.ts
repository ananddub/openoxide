import type {GlobalContainerItem} from './docker-inspect-modal';

/** Transform raw Docker / running-deployment API rows into the unified GlobalContainerItem shape consumed by the Docker page. */
export function mapGlobalContainers(
	rawDockerContainers: unknown,
	rawRunning: unknown,
): GlobalContainerItem[] {
	const dockerList = Array.isArray(rawDockerContainers)
		? rawDockerContainers
		: [];
	const runningList = Array.isArray(rawRunning) ? rawRunning : [];

	if (dockerList.length > 0) {
		return dockerList.map((rawItem: unknown, idx: number) => {
			const item = rawItem as Record<string, unknown>;
			const stateStr = String(
				item.state || item.State || item.status || '',
			).toLowerCase();
			const statusStr = String(
				item.status || item.Status || '',
			).toLowerCase();
			const isRunning =
				stateStr.includes('running') || statusStr.startsWith('up');

			return {
				id: String(item.id || item.ID || `cnt-${idx + 1}`).slice(0, 12),
				name: String(
					item.names || item.Names || item.name || `container-${idx + 1}`,
				).replace(/^\//, ''),
				image: String(item.image || item.Image || 'docker-image:latest'),
				status: isRunning ? ('running' as const) : ('stopped' as const),
				statusText: String(
					item.status ||
						item.Status ||
						item.running_for ||
						(isRunning ? 'Up (active)' : 'Exited'),
				),
				created: String(
					item.created_at ||
						item.CreatedAt ||
						item.running_for ||
						'Recently',
				),
				ports: String(item.ports || item.Ports || '—'),
				networks: item.networks
					? String(item.networks).split(',')
					: ['bridge'],
				mounts: item.mounts
					? [
							{
								source: String(item.mounts),
								destination: '/data',
								mode: 'rw',
							},
						]
					: [],
				env: {CONTAINER_ID: String(item.id || `cnt-${idx + 1}`)},
			};
		});
	}

	return runningList.map((rawItem: unknown, idx: number) => {
		const item = rawItem as Record<string, unknown>;
		return {
			id: String(item.id || item.container_id || `cnt-${idx + 1}`),
			name: String(item.app_name || item.name || `container-${item.id}`),
			image: String(
				item.docker_image || item.image || 'openoxide-app:latest',
			),
			status:
				String(item.status || item.state || 'running').toLowerCase() ===
				'running'
					? ('running' as const)
					: ('stopped' as const),
			statusText: String(
				item.status_text || item.status || 'Up (healthy)',
			),
			created: item.created_at
				? new Date(Number(item.created_at)).toLocaleString()
				: 'Recently',
			ports: String(item.ports || '80/tcp -> 0.0.0.0:80'),
			networks: (item.networks as string[]) || [
				'openoxide_net',
				'traefik_proxy',
			],
			mounts: (item.mounts as {
				source: string;
				destination: string;
				mode: string;
			}[]) || [
				{
					source: '/var/lib/docker/volumes/app_data/_data',
					destination: '/app/data',
					mode: 'rw',
				},
			],
			env: (item.env as Record<string, string>) || {
				NODE_ENV: 'production',
				PORT: '3000',
				DATABASE_URL: 'postgres://***@db:5432/app',
			},
		};
	});
}
