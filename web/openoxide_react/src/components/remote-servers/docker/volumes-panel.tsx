import {useState} from 'react';
import {formatApiError} from '#/api/utils';
import {
	useServerInspect,
	useServerVolumes,
	type ServerVolume,
} from '#/api/live/docker';

import {
	DockerEmptyState,
	DockerInspectDialog,
	DockerPanelShell,
	DockerTable,
	type DockerColumn,
} from './docker-shell';

/** Volumes tab: `docker volume ls` for this server with size and mount path. */
export function ServerVolumesPanel({serverId}: {serverId: bigint}) {
	const query = useServerVolumes(serverId);
	const [inspectName, setInspectName] = useState<string | null>(null);
	const inspect = useServerInspect('volume', serverId, inspectName);

	const columns: DockerColumn<ServerVolume>[] = [
		{
			label: 'Name',
			cell: v => (
				<span className="font-semibold text-foreground">{v.name}</span>
			),
		},
		{
			label: 'Driver',
			cell: v => <span className="text-muted-foreground">{v.driver}</span>,
		},
		{
			label: 'Scope',
			cell: v => <span className="text-muted-foreground">{v.scope}</span>,
		},
		{
			label: 'Mountpoint',
			cell: v => (
				<span className="font-mono text-[11px] text-muted-foreground">
					{v.mountpoint || '—'}
				</span>
			),
		},
		{
			label: 'Available',
			cell: v => (
				<span
					className={
						v.availability.toLowerCase() === 'available'
							? 'text-emerald-500'
							: 'text-amber-500'
					}>
					{v.availability || '—'}
				</span>
			),
		},
		{label: 'Size', align: 'right', cell: v => v.size || '—'},
	];

	return (
		<DockerPanelShell
			hint="Volumes"
			count={query.data?.length ?? 0}
			isLoading={query.isLoading}
			error={query.isError ? formatApiError(query.error) : null}
			isRefreshing={query.isFetching}
			onRefresh={() => void query.refetch()}
			empty={
				<DockerEmptyState label="No volumes on this server's engine." />
			}>
			<DockerTable
				columns={columns}
				rows={query.data ?? []}
				keyOf={v => v.name}
				onSelect={v => setInspectName(v.name)}
			/>
			{inspectName && (
				<DockerInspectDialog
					title={inspectName}
					documentJson={inspect.data ?? ''}
					isLoading={inspect.isLoading}
					error={inspect.isError ? formatApiError(inspect.error) : null}
					onClose={() => setInspectName(null)}
				/>
			)}
		</DockerPanelShell>
	);
}
