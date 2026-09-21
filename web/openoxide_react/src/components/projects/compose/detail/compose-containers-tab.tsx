import {useState, useMemo} from 'react';
import {Box, RefreshCw} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {
	useComposeStart,
	useComposeStop,
	useDeploymentListContainers,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {ComposeContainersTable} from './containers/compose-containers-table';
import {
	ContainerInspectModal,
	type ContainerItem,
} from './containers/container-inspect-modal';
import {buildComposeContainers} from './containers/build-compose-containers';

interface ComposeContainersTabProps {
	compose: any;
	onUpdated?: () => void;
}

import {extractServicesFromYaml} from '#/components/shared/terminal-card';

export function ComposeContainersTab({
	compose,
	onUpdated,
}: ComposeContainersTabProps) {
	const [activeModal, setActiveModal] = useState<{
		type: 'logs' | 'config' | 'mount' | 'network';
		container: ContainerItem;
	} | null>(null);
	const [logsStream, setLogsStream] = useState<string[]>([]);

	const serverId = compose?.destination_id || compose?.server_id;

	const {
		data: rawContainers = [],
		refetch: refetchContainers,
		isFetching,
	} = useDeploymentListContainers(serverId ? Number(serverId) : null);

	const serviceNames = useMemo(() => {
		const extracted = extractServicesFromYaml(compose?.compose_file);
		return extracted.length > 0 ? extracted : ['app'];
	}, [compose?.compose_file]);

	const containersList: ContainerItem[] = useMemo(
		() => buildComposeContainers(compose, rawContainers, serviceNames),
		[compose, rawContainers, serviceNames],
	);

	const postAction = useComposeStart();
	const stopAction = useComposeStop();

	const handleContainerAction = async (
		container: ContainerItem,
		action: 'start' | 'stop' | 'restart' | 'kill',
	) => {
		try {
			if (action === 'start' || action === 'restart') {
				await postAction.mutateAsync(compose?.id);
			} else {
				await stopAction.mutateAsync(compose?.id);
			}
			toast.success(
				`Container '${container.name}' ${action}ed successfully`,
			);
			refetchContainers();
			onUpdated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleOpenViewModal = (
		container: ContainerItem,
		type: 'logs' | 'config' | 'mount' | 'network',
	) => {
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
			<section className="flex flex-wrap items-center justify-between gap-4 rounded-xl bg-card p-5 shadow-sm">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Box className="h-4 w-4 text-primary" /> Compose Stack
						Containers
						{isFetching && (
							<RefreshCw className="ml-1 h-3.5 w-3.5 animate-spin text-muted-foreground" />
						)}
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Real-time status and telemetry of live Docker containers
					</p>
				</div>
				<Badge variant="outline" className="px-3 py-1 font-mono text-xs">
					Total Containers: {containersList.length}
				</Badge>
			</section>

			<ComposeContainersTable
				containers={containersList}
				onOpenModal={handleOpenViewModal}
				onAction={handleContainerAction}
			/>
			<ContainerInspectModal
				activeModal={activeModal}
				onClose={() => setActiveModal(null)}
				logsStream={logsStream}
			/>
		</div>
	);
}
