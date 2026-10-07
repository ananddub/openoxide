import {useState} from 'react';
import {Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {formatApiError} from '#/api/utils';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {
	useServerDiskUsage,
	useServerPruneMutation,
	type ServerDiskUsageRow,
} from '#/api/live/docker';

import {
	DockerEmptyState,
	DockerPanelShell,
	DockerTable,
	type DockerColumn,
} from './docker-shell';
import {diskBytes, formatBytes} from './disk-metrics';

const PRUNE_TARGETS = [
	['containers', 'Containers'],
	['images', 'Images'],
	['networks', 'Networks'],
	['volumes', 'Volumes'],
] as const;

/** Disk usage tab: `docker system df` for this server plus a scoped prune. */
export function ServerDiskPanel({serverId}: {serverId: bigint}) {
	const {can} = usePermissions();
	const canPrune = can('server:create');
	const query = useServerDiskUsage(serverId);
	const prune = useServerPruneMutation(serverId);
	const [selection, setSelection] = useState({
		containers: true,
		images: true,
		networks: false,
		volumes: false,
	});

	const rows = query.data ?? [];
	const reclaimable = rows.reduce(
		(sum, row) => sum + diskBytes(row.reclaimable),
		0,
	);

	const runPrune = () => {
		prune.mutate(
			{...selection},
			{
				onSuccess: output =>
					toast.success(output || 'Prune finished — nothing to reclaim'),
				onError: error => toast.error(formatApiError(error)),
			},
		);
	};

	const columns: DockerColumn<ServerDiskUsageRow>[] = [
		{
			label: 'Type',
			cell: row => (
				<span className="font-semibold text-foreground">{row.type}</span>
			),
		},
		{label: 'Total', align: 'right', cell: row => row.total},
		{label: 'Active', align: 'right', cell: row => row.active},
		{label: 'Size', align: 'right', cell: row => row.size},
		{
			label: 'Reclaimable',
			align: 'right',
			cell: row => (
				<span className="text-amber-600 dark:text-amber-500">
					{row.reclaimable}
				</span>
			),
		},
	];

	return (
		<DockerPanelShell
			hint={`Disk usage${reclaimable > 0 ? ` · ${formatBytes(reclaimable)} reclaimable` : ''}`}
			count={rows.length}
			isLoading={query.isLoading}
			error={query.isError ? formatApiError(query.error) : null}
			isRefreshing={query.isFetching}
			onRefresh={() => void query.refetch()}
			empty={
				<DockerEmptyState label="This server's engine reported no disk usage." />
			}>
			<div className="border-b border-border/50">
				<DockerTable
					columns={columns}
					rows={rows}
					keyOf={row => row.type}
				/>
			</div>
			{canPrune && (
				<div className="flex flex-wrap items-center gap-4 bg-muted/10 px-5 py-3.5">
					<span className="inline-flex items-center gap-1.5 text-xs font-semibold text-muted-foreground">
						<Trash2 className="size-3.5" />
						Prune:
					</span>
					{PRUNE_TARGETS.map(([key, label]) => (
						<label
							key={key}
							className="flex cursor-pointer items-center gap-2 text-xs text-foreground/80">
							<Checkbox
								checked={selection[key]}
								onCheckedChange={checked =>
									setSelection(prev => ({
										...prev,
										[key]: checked === true,
									}))
								}
							/>
							{label}
						</label>
					))}
					<Button
						size="sm"
						variant="destructive"
						onClick={runPrune}
						disabled={
							prune.isPending || !Object.values(selection).some(Boolean)
						}
						className="ml-auto h-8 cursor-pointer text-xs">
						{prune.isPending ? 'Pruning…' : 'Run Prune'}
					</Button>
				</div>
			)}
		</DockerPanelShell>
	);
}
