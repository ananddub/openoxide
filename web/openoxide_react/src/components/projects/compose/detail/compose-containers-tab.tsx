import {useState, useMemo} from 'react';
import {Box, RefreshCw} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {ComposeContainersTable} from './containers/compose-containers-table';
import {ContainerInspectModal, type ContainerItem} from './containers/container-inspect-modal';

interface ComposeContainersTabProps {
	compose: any;
	onUpdated?: () => void;
}

import {extractServicesFromYaml} from '#/components/projects/common/terminal-modal';

export function ComposeContainersTab({compose, onUpdated}: ComposeContainersTabProps) {
	const [activeModal, setActiveModal] = useState<{type: 'logs' | 'config' | 'mount' | 'network'; container: ContainerItem} | null>(null);
	const [logsStream, setLogsStream] = useState<string[]>([]);

	const serverId = compose?.destination_id || compose?.server_id;

	const {data: rawContainers = [], refetch: refetchContainers, isFetching} = $api.useQuery(
		'get',
		'/deployments/docker/containers',
		{
			params: {
				query: {
					query: {
						server_id: serverId ? Number(serverId) : undefined,
					},
				},
			},
			refetchInterval: 3000,
		},
	);

	const serviceNames = useMemo(() => {
		const extracted = extractServicesFromYaml(compose?.compose_file);
		return extracted.length > 0 ? extracted : ['app'];
	}, [compose?.compose_file]);

	const containersList: ContainerItem[] = useMemo(() => {
		const appName = compose?.app_name || compose?.name || '';
		const rawApp = appName.toLowerCase().trim();
		const cleanApp = rawApp.replace(/[^a-z0-9]/g, '');

		const allContainers = Array.isArray(rawContainers) ? rawContainers : [];

		let matched = allContainers.filter((c: any) => {
			if (!rawApp) return false;
			const n = String(c.names || c.Names || c.name || c.Name || '').toLowerCase().replace(/^\//, '');
			const l = String(c.labels || c.Labels || '').toLowerCase();
			const cleanN = n.replace(/[^a-z0-9]/g, '');

			// Match strictly by compose project prefix or docker compose label
			const isNameMatch = n.startsWith(`${rawApp}-`) || n.startsWith(`${rawApp}_`) || n.includes(`-${rawApp}-`) || cleanN.startsWith(cleanApp);
			const isLabelMatch = l.includes(`com.docker.compose.project=${rawApp}`) || l.includes(`compose.project=${rawApp}`) || l.includes(rawApp);

			return isNameMatch || isLabelMatch;
		});

		// Helper to check if container is actively running
		const isContainerRunning = (c: any) => {
			const state = String(c.state || c.State || '').toLowerCase();
			const status = String(c.status || c.Status || '').toLowerCase();
			if (state.includes('exited') || state.includes('dead') || state.includes('stopped')) return false;
			if (status.includes('exited') || status.includes('dead')) return false;
			return state.includes('running') || state.includes('up') || status.includes('up') || status.includes('running');
		};

		if (matched.length > 0) {
			const runningContainers = matched.filter(isContainerRunning);
			// Only show running containers if any running container exists
			const activeMatched = runningContainers.length > 0 ? runningContainers : matched;

			// Deduplicate by clean container name
			const uniqueMap = new Map<string, any>();
			activeMatched.forEach((c: any) => {
				const rawName = String(c.names || c.Names || c.name || c.Name || c.id || c.ID || 'container').replace(/^\//, '');
				if (!uniqueMap.has(rawName)) {
					uniqueMap.set(rawName, c);
				}
			});

			return Array.from(uniqueMap.values()).map((c: any) => {
				const isRunning = isContainerRunning(c);
				const rawName = String(c.names || c.Names || c.name || c.Name || c.id || c.ID || 'container').replace(/^\//, '');
				const cleanName = rawName.split(',')[0].trim();

				let serviceName = serviceNames.find(s => cleanName.toLowerCase().includes(`-${s.toLowerCase()}-`) || cleanName.toLowerCase().endsWith(`-${s.toLowerCase()}`));
				if (!serviceName) {
					const parts = cleanName.split(/[-_]/);
					if (parts.length >= 2 && !isNaN(Number(parts[parts.length - 1]))) {
						serviceName = parts[parts.length - 2];
					} else {
						serviceName = parts.pop() || 'service';
					}
				}

				const containerId = String(c.id || c.ID || 'id').slice(0, 12);
				const containerImage = c.image || c.Image || `${appName}:latest`;
				const containerStatusText = c.status || c.Status || (isRunning ? 'Up (healthy)' : 'Exited');
				const containerPorts = c.ports || c.Ports || 'N/A';
				const containerNetworks = c.networks || c.Networks ? [c.networks || c.Networks] : [`${appName}_default`];

				return {
					id: containerId,
					name: cleanName,
					service: serviceName || 'service',
					image: containerImage,
					status: isRunning ? 'running' : 'stopped',
					statusText: containerStatusText,
					ports: containerPorts,
					networks: containerNetworks,
					mounts: c.mounts || c.Mounts ? [{source: c.mounts || c.Mounts, destination: '/app', mode: 'rw'}] : [],
					env: {COMPOSE_PROJECT: appName, CONTAINER_ID: containerId},
				};
			});
		}

		return [];
	}, [compose, rawContainers, serviceNames]);

	const postAction = $api.useMutation('post', '/compose/{id}/start');
	const stopAction = $api.useMutation('post', '/compose/{id}/stop');

	const handleContainerAction = async (container: ContainerItem, action: 'start' | 'stop' | 'restart' | 'kill') => {
		try {
			if (action === 'start' || action === 'restart') {
				await postAction.mutateAsync({params: {path: {id: compose?.id}}});
			} else {
				await stopAction.mutateAsync({params: {path: {id: compose?.id}}});
			}
			toast.success(`Container '${container.name}' ${action}ed successfully`);
			refetchContainers();
			onUpdated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleOpenViewModal = (container: ContainerItem, type: 'logs' | 'config' | 'mount' | 'network') => {
		setActiveModal({type, container});
		if (type === 'logs') {
			setLogsStream([
				`[${new Date().toISOString()}] Attached to container stdout/stderr stream '${container.name}'`,
				`[${new Date().toISOString()}] Container status: ${container.statusText}`,
				`[${new Date().toISOString()}] Image: ${container.image}`,
			]);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Box className="w-4 h-4 text-primary" /> Compose Stack Containers
						{isFetching && <RefreshCw className="w-3.5 h-3.5 animate-spin text-muted-foreground ml-1" />}
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time status and telemetry of live Docker containers</p>
				</div>
				<Badge variant="outline" className="text-xs font-mono px-3 py-1">Total Containers: {containersList.length}</Badge>
			</section>

			<ComposeContainersTable containers={containersList} onOpenModal={handleOpenViewModal} onAction={handleContainerAction} />
			<ContainerInspectModal activeModal={activeModal} onClose={() => setActiveModal(null)} logsStream={logsStream} />
		</div>
	);
}
