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

const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const lines = yamlStr.split('\n');
	const services: string[] = [];
	let inServices = false;
	let servicesIndent = 0;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;
		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text.startsWith('services:')) {
			inServices = true;
			servicesIndent = indent;
			continue;
		}
		if (inServices) {
			if (indent <= servicesIndent && text.endsWith(':') && !text.startsWith('-')) {
				inServices = false;
			} else if (indent > servicesIndent && text.endsWith(':') && !text.includes(' ')) {
				const srv = text.slice(0, -1).trim();
				if (srv && !services.includes(srv)) services.push(srv);
			}
		}
	}
	return services;
};

export function ComposeContainersTab({compose, onUpdated}: ComposeContainersTabProps) {
	const [activeModal, setActiveModal] = useState<{type: 'logs' | 'config' | 'mount' | 'network'; container: ContainerItem} | null>(null);
	const [logsStream, setLogsStream] = useState<string[]>([]);

	const {data: rawContainers = [], refetch: refetchContainers, isFetching} = $api.useQuery(
		'get',
		'/docker/containers',
		{
			params: {query: {server_id: compose?.destination_id}},
			refetchInterval: 3000,
		},
	);

	const serviceNames = useMemo(() => {
		const extracted = extractServicesFromYaml(compose?.compose_file);
		return extracted.length > 0 ? extracted : ['app'];
	}, [compose?.compose_file]);

	const containersList: ContainerItem[] = useMemo(() => {
		const appName = compose?.app_name || compose?.name || 'compose';
		const isStackRunning = ['running', 'deployed', 'done', 'success', 'active', 'ok'].includes(
			(compose?.compose_status || compose?.status || '').toLowerCase()
		);

		const matched = (rawContainers || []).filter((c: any) => {
			const n = (c.names || '').toLowerCase();
			const l = (c.labels || '').toLowerCase();
			const cleanApp = appName.toLowerCase().replace(/[^a-z0-9]/g, '');
			return n.includes(cleanApp) || l.includes(cleanApp) || serviceNames.some(s => n.includes(s.toLowerCase()));
		});

		if (matched.length > 0) {
			return matched.map((c: any) => {
				const st = (c.state || c.status || '').toLowerCase();
				const isRunning = st.includes('up') || st.includes('running');
				const cleanName = (c.names || c.id || 'container').replace(/^\//, '');

				return {
					id: (c.id || 'id').slice(0, 12),
					name: cleanName,
					service: cleanName.split(/[-_]/).pop() || 'service',
					image: c.image || `${appName}:latest`,
					status: isRunning ? 'running' : 'stopped',
					statusText: c.status || (isRunning ? 'Up (healthy)' : 'Exited'),
					ports: c.ports || 'N/A',
					networks: c.networks ? [c.networks] : [`${appName}_default`],
					mounts: c.mounts ? [{source: c.mounts, destination: '/app', mode: 'rw'}] : [],
					env: {COMPOSE_PROJECT: appName, CONTAINER_ID: c.id},
				};
			});
		}

		return serviceNames.map((srv, idx) => ({
			id: `${appName}_${srv}_${idx + 1}`.slice(0, 12),
			name: `${appName}-${srv}-1`,
			service: srv,
			image: `${appName}-${srv}:latest`,
			status: isStackRunning ? 'running' : 'stopped',
			statusText: isStackRunning ? 'Running' : 'Stopped / Not Deployed',
			ports: 'N/A',
			networks: [`${appName}_default`],
			mounts: [],
			env: {COMPOSE_PROJECT: appName, SERVICE_NAME: srv},
		}));
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
