import {Activity, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Skeleton} from '#/components/ui/skeleton';
import {formatApiError} from '#/api/utils';
import {
	useContainerStats,
	type ContainerStatsPayload,
} from '#/api/live/monitoring';
import type {ServerContainer} from '#/api/live/docker';
import {
	DockerEmptyState,
	DockerTable,
	type DockerColumn,
} from './docker-shell';
import {
	formatKilobytesPerSecond,
	formatMegabytes,
	formatMemoryRow,
	formatPercent,
} from './container-stats';

const columns: DockerColumn<ContainerStatsPayload>[] = [
	{
		label: 'Container',
		cell: row => (
			<span className="font-mono text-[11px] text-foreground">
				{row.container_name}
			</span>
		),
	},
	{
		label: 'CPU',
		align: 'right',
		cell: row => formatPercent(row.cpu_percent),
	},
	{
		label: 'Memory',
		align: 'right',
		cell: row => (
			<span>
				{formatMemoryRow(row)} · {formatPercent(row.memory_percent)}
			</span>
		),
	},
	{
		label: 'Net in',
		align: 'right',
		cell: row => formatKilobytesPerSecond(row.network_in_kbps),
	},
	{
		label: 'Net out',
		align: 'right',
		cell: row => formatKilobytesPerSecond(row.network_out_kbps),
	},
	{
		label: 'Block r/w',
		align: 'right',
		cell: row => (
			<span>
				{formatMegabytes(row.disk_read_mb)} /{' '}
				{formatMegabytes(row.disk_write_mb)}
			</span>
		),
	},
];

/**
 * Docker's current reading for one container, asked for when the dialog opens
 * and re-read only on the button. There is no `refetchInterval` here: a tab left
 * open over lunch must not keep a $5 host sampling its own cgroups.
 */
export function ContainerStatsDialog({
	serverId,
	container,
	onClose,
}: {
	serverId: bigint;
	container: ServerContainer;
	onClose: () => void;
}) {
	const stats = useContainerStats(serverId, container.name);

	return (
		<Dialog open onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-w-3xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-sm">
						<Activity className="size-4" />
						Resource stats — {container.name}
						<Button
							variant="outline"
							size="sm"
							disabled={stats.isFetching}
							onClick={() => void stats.refetch()}
							className="ml-auto h-7 cursor-pointer gap-1.5 text-xs">
							{stats.isFetching ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<Activity className="size-3.5" />
							)}
							Re-read
						</Button>
					</DialogTitle>
					<DialogDescription className="text-xs">
						CPU percent, memory against the container limit, and network
						and block IO counters as Docker reports them right now.
					</DialogDescription>
				</DialogHeader>
				{stats.isError ? (
					<p className="py-8 text-center text-xs text-rose-500">
						{formatApiError(stats.error)}
					</p>
				) : stats.isLoading ? (
					<div className="space-y-2 py-2">
						<Skeleton className="h-8 w-full" />
					</div>
				) : (stats.data?.length ?? 0) === 0 ? (
					<DockerEmptyState label="The agent returned no reading for this container." />
				) : (
					<DockerTable
						columns={columns}
						rows={stats.data ?? []}
						keyOf={row => row.container_name}
					/>
				)}
			</DialogContent>
		</Dialog>
	);
}
