import {useState} from 'react';
import {
	Activity,
	Play,
	RotateCw,
	ScrollText,
	Square,
	Terminal as TerminalIcon,
	Trash2,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {TerminalCard} from '#/components/shared/terminal-card';
import {formatApiError} from '#/api/utils';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {
	useContainerActionMutation,
	useContainerRemoveMutation,
	useServerContainers,
	useServerInspect,
	type ContainerActionKind,
	type ServerContainer,
} from '#/api/live/docker';

import {ContainerLogsDialog} from './container-logs-dialog';
import {ContainerStatsDialog} from './container-stats-dialog';
import {
	DockerEmptyState,
	DockerInspectDialog,
	DockerPanelShell,
	DockerStatusPill,
	DockerTable,
	type DockerColumn,
} from './docker-shell';

/** Containers tab: live `docker ps` for this server with lifecycle actions and inspect. */
export function ServerContainersPanel({serverId}: {serverId: bigint}) {
	const {can} = usePermissions();
	const canAct = can('server:create');
	const query = useServerContainers(serverId);
	const [inspectId, setInspectId] = useState<string | null>(null);
	const [logsFor, setLogsFor] = useState<ServerContainer | null>(null);
	const [consoleFor, setConsoleFor] = useState<ServerContainer | null>(
		null,
	);
	const [statsFor, setStatsFor] = useState<ServerContainer | null>(null);
	const inspect = useServerInspect('container', serverId, inspectId);
	const action = useContainerActionMutation(serverId);
	const remove = useContainerRemoveMutation(serverId);

	const run = (container: ServerContainer, kind: ContainerActionKind) => {
		action.mutate(
			{containerId: container.id, action: kind},
			{
				onSuccess: () => toast.success(`${container.name} — ${kind}`),
				onError: error => toast.error(formatApiError(error)),
			},
		);
	};

	const drop = (container: ServerContainer) => {
		remove.mutate(
			{containerId: container.id, force: true, volumes: false},
			{
				onSuccess: () => toast.success(`Removed ${container.name}`),
				onError: error => toast.error(formatApiError(error)),
			},
		);
	};

	const columns: DockerColumn<ServerContainer>[] = [
		{
			label: 'Name',
			cell: c => (
				<span className="font-semibold text-foreground">{c.name}</span>
			),
		},
		{
			label: 'Image',
			cell: c => <span className="text-muted-foreground">{c.image}</span>,
		},
		{
			label: 'Status',
			cell: c => (
				<DockerStatusPill
					running={c.running}
					label={c.state || c.status}
				/>
			),
		},
		{
			label: 'Uptime',
			cell: c => (
				<span className="text-muted-foreground">{c.runningFor}</span>
			),
		},
		{
			label: 'Ports',
			cell: c => (
				<span className="font-mono text-[11px] text-muted-foreground">
					{c.ports || '—'}
				</span>
			),
		},
		{
			label: 'Actions',
			align: 'right',
			cell: c => (
				<span
					className="inline-flex items-center gap-1"
					onClick={event => event.stopPropagation()}>
					<Button
						variant="ghost"
						size="sm"
						disabled={!canAct || action.isPending}
						onClick={() => run(c, c.running ? 'stop' : 'start')}
						title={c.running ? 'Stop' : 'Start'}
						className="size-7 cursor-pointer p-0 disabled:cursor-default">
						{c.running ? (
							<Square className="size-3.5" />
						) : (
							<Play className="size-3.5" />
						)}
					</Button>
					<Button
						variant="ghost"
						size="sm"
						disabled={!canAct || !c.running || action.isPending}
						onClick={() => run(c, 'restart')}
						title="Restart"
						className="size-7 cursor-pointer p-0 disabled:cursor-default">
						<RotateCw className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="sm"
						disabled={!canAct || remove.isPending}
						onClick={() => drop(c)}
						title="Remove"
						className="size-7 cursor-pointer p-0 text-rose-500 disabled:cursor-default">
						<Trash2 className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="sm"
						disabled={!c.running}
						onClick={() => setStatsFor(c)}
						title={
							c.running
								? 'Resource stats'
								: 'Resource stats need the container to be running'
						}
						className="size-7 cursor-pointer p-0 disabled:cursor-default">
						<Activity className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="sm"
						onClick={() => setLogsFor(c)}
						title="Logs"
						className="size-7 cursor-pointer p-0">
						<ScrollText className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="sm"
						disabled={!c.running}
						onClick={() => setConsoleFor(c)}
						title={
							c.running
								? 'Console'
								: 'A shell needs the container to be running'
						}
						className="size-7 cursor-pointer p-0 disabled:cursor-default">
						<TerminalIcon className="size-3.5" />
					</Button>
				</span>
			),
		},
	];

	return (
		<DockerPanelShell
			hint="Containers"
			count={query.data?.length ?? 0}
			isLoading={query.isLoading}
			error={query.isError ? formatApiError(query.error) : null}
			isRefreshing={query.isFetching}
			onRefresh={() => void query.refetch()}
			empty={
				<DockerEmptyState label="No containers on this server's engine." />
			}>
			<DockerTable
				columns={columns}
				rows={query.data ?? []}
				keyOf={c => c.id}
				onSelect={c => setInspectId(c.id)}
			/>
			{inspectId && (
				<DockerInspectDialog
					title={inspectId}
					documentJson={inspect.data ?? ''}
					isLoading={inspect.isLoading}
					error={inspect.isError ? formatApiError(inspect.error) : null}
					onClose={() => setInspectId(null)}
				/>
			)}
			{logsFor && (
				<ContainerLogsDialog
					serverId={serverId}
					container={logsFor}
					onClose={() => setLogsFor(null)}
				/>
			)}
			{statsFor && (
				<ContainerStatsDialog
					serverId={serverId}
					container={statsFor}
					onClose={() => setStatsFor(null)}
				/>
			)}
			{consoleFor && (
				<TerminalCard
					app={{
						app_name: consoleFor.name,
						name: consoleFor.name,
						server_id: Number(serverId),
					}}
					enabled
					onClose={() => setConsoleFor(null)}
				/>
			)}
		</DockerPanelShell>
	);
}
