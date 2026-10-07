import {useState} from 'react';
import {formatApiError} from '#/api/utils';
import {
	useServerImages,
	useServerInspect,
	type ServerImage,
} from '#/api/live/docker';

import {
	DockerEmptyState,
	DockerInspectDialog,
	DockerPanelShell,
	DockerTable,
	type DockerColumn,
} from './docker-shell';

/** Images tab: `docker images` for this server; a row opens its inspect document. */
export function ServerImagesPanel({serverId}: {serverId: bigint}) {
	const query = useServerImages(serverId);
	const [inspectId, setInspectId] = useState<string | null>(null);
	const inspect = useServerInspect('image', serverId, inspectId);

	const columns: DockerColumn<ServerImage>[] = [
		{
			label: 'Repository',
			cell: i => (
				<span className="font-semibold text-foreground">
					{i.repository || '<none>'}
				</span>
			),
		},
		{
			label: 'Tag',
			cell: i => (
				<span className="text-muted-foreground">{i.tag || '<none>'}</span>
			),
		},
		{
			label: 'ID',
			cell: i => (
				<span className="font-mono text-[11px] text-muted-foreground">
					{i.id}
				</span>
			),
		},
		{
			label: 'Created',
			cell: i => (
				<span className="text-muted-foreground">{i.createdSince}</span>
			),
		},
		{label: 'Size', align: 'right', cell: i => i.size},
		{label: 'Shared', align: 'right', cell: i => i.sharedSize},
		{label: 'Unique', align: 'right', cell: i => i.uniqueSize},
	];

	return (
		<DockerPanelShell
			hint="Images"
			count={query.data?.length ?? 0}
			isLoading={query.isLoading}
			error={query.isError ? formatApiError(query.error) : null}
			isRefreshing={query.isFetching}
			onRefresh={() => void query.refetch()}
			empty={
				<DockerEmptyState label="No images on this server's engine." />
			}>
			<DockerTable
				columns={columns}
				rows={query.data ?? []}
				keyOf={i => `${i.id}-${i.reference}`}
				onSelect={i => setInspectId(i.id)}
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
