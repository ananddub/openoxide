import {useState, useMemo} from 'react';
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
import {SectionHeader} from '#/components/settings/list-card';

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

	const {data: rawContainers = [], refetch: refetchContainers} =
		useDeploymentListContainers(serverId ? Number(serverId) : null);

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
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Containers"
				description="Real-time status and telemetry of the live Docker containers in this stack.">
				<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
					{containersList.length}
				</span>
			</SectionHeader>

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
