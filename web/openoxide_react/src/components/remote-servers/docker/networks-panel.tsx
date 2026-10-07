import {useState} from 'react';
import {formatApiError} from '#/api/utils';
import {
	useServerInspect,
	useServerNetworks,
	type ServerNetwork,
} from '#/api/live/docker';

import {
	DockerEmptyState,
	DockerInspectDialog,
	DockerPanelShell,
	DockerTable,
	type DockerColumn,
} from './docker-shell';

/** Networks tab: `docker network ls` for this server; a row opens its inspect document. */
export function ServerNetworksPanel({serverId}: {serverId: bigint}) {
	const query = useServerNetworks(serverId);
	const [inspectId, setInspectId] = useState<string | null>(null);
	const inspect = useServerInspect('network', serverId, inspectId);

	const columns: DockerColumn<ServerNetwork>[] = [
		{
			label: 'Name',
			cell: n => (
				<span className="font-semibold text-foreground">{n.name}</span>
			),
		},
		{
			label: 'ID',
			cell: n => (
				<span className="font-mono text-[11px] text-muted-foreground">
					{n.id}
				</span>
			),
		},
		{
			label: 'Driver',
			cell: n => <span className="text-muted-foreground">{n.driver}</span>,
		},
		{
			label: 'Scope',
			cell: n => <span className="text-muted-foreground">{n.scope}</span>,
		},
		{
			label: 'Internal',
			cell: n => (
				<span className="text-muted-foreground">
					{n.internal || 'false'}
				</span>
			),
		},
	];

	return (
		<DockerPanelShell
			hint="Networks"
			count={query.data?.length ?? 0}
			isLoading={query.isLoading}
			error={query.isError ? formatApiError(query.error) : null}
			isRefreshing={query.isFetching}
			onRefresh={() => void query.refetch()}
			empty={
				<DockerEmptyState label="No networks on this server's engine." />
			}>
			<DockerTable
				columns={columns}
				rows={query.data ?? []}
				keyOf={n => n.id}
				onSelect={n => setInspectId(n.id)}
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
		</DockerPanelShell>
	);
}
