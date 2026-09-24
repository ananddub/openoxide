import {useState} from 'react';
import {Container, RotateCw, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {Skeleton} from '#/components/ui/skeleton';
import {useDockerDiskUsage, useDockerPrune} from '#/api/live';
import {usePermissions} from '#/hooks/permissions/use-permissions';

const PRUNE_TARGETS = [
	['containers', 'Containers'],
	['images', 'Images'],
	['networks', 'Networks'],
	['volumes', 'Volumes'],
] as const;

function formatBytes(bytes: number): string {
	if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
	if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`;
	return `${Math.round(bytes / 1e3)} kB`;
}

/** "Docker Disk Usage" card: `docker system df` table + manual prune. */
export function DockerDiskCard() {
	const {can} = usePermissions();
	const canSeeHost = can('server:read');
	const diskQuery = useDockerDiskUsage({enabled: canSeeHost});
	const pruneMutation = useDockerPrune();
	const [prune, setPrune] = useState({
		containers: true,
		images: true,
		networks: false,
		volumes: false,
	});

	if (!canSeeHost) return null;

	const reclaimable = (diskQuery.data ?? []).reduce((sum, row) => {
		const match = /^([\d.]+)([kMG]?)B?$/.exec(
			row.Reclaimable?.trim() ?? '',
		);
		if (!match) return sum;
		const mult = {k: 1e3, M: 1e6, G: 1e9}[match[2]] ?? 1;
		return sum + Number(match[1]) * mult;
	}, 0);

	const handlePrune = () => {
		pruneMutation.mutate(
			{...prune},
			{
				onSuccess: result => {
					toast.success(
						`Docker prune finished (${result.operation || 'prune'})`,
					);
				},
				onError: error => toast.error(error.message),
			},
		);
	};

	return (
		<div className="overflow-hidden rounded-xl border border-border bg-card">
			<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
					<Container className="size-[18px] text-muted-foreground" />
				</div>
				<div className="min-w-0 flex-1">
					<h3 className="text-[15px] font-semibold text-foreground">
						Docker Disk Usage
					</h3>
					<p className="text-xs text-muted-foreground">
						{'`docker system df`'} on this host
						{reclaimable > 0 && (
							<span className="text-amber-500">
								{' · '}
								{formatBytes(reclaimable)} reclaimable
							</span>
						)}
					</p>
				</div>
				<Button
					variant="outline"
					size="sm"
					onClick={() => void diskQuery.refetch()}
					className="h-8 shrink-0 cursor-pointer gap-1.5 text-xs">
					<RotateCw
						className={`size-3.5 ${diskQuery.isFetching ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
			</div>
			{diskQuery.isLoading ? (
				<div className="space-y-2 p-5">
					<Skeleton className="h-8 w-full" />
					<Skeleton className="h-8 w-full" />
					<Skeleton className="h-8 w-full" />
				</div>
			) : (
				<table className="w-full text-xs">
					<thead>
						<tr className="border-b border-border/50 text-left text-muted-foreground">
							<th className="px-5 py-2.5 font-medium">Type</th>
							<th className="px-2 py-2.5 font-medium">Count</th>
							<th className="px-2 py-2.5 font-medium">Active</th>
							<th className="px-2 py-2.5 font-medium">Size</th>
							<th className="px-5 py-2.5 text-right font-medium">
								Reclaimable
							</th>
						</tr>
					</thead>
					<tbody>
						{(diskQuery.data ?? []).map(row => (
							<tr
								key={row.Type}
								className="border-b border-border/40 last:border-0">
								<td className="px-5 py-2.5 font-semibold">{row.Type}</td>
								<td className="px-2 py-2.5 tabular-nums">
									{row.TotalCount}
								</td>
								<td className="px-2 py-2.5 tabular-nums">{row.Active}</td>
								<td className="px-2 py-2.5 tabular-nums">{row.Size}</td>
								<td className="px-5 py-2.5 text-right text-amber-600 tabular-nums dark:text-amber-500">
									{row.Reclaimable}
								</td>
							</tr>
						))}
					</tbody>
				</table>
			)}
			{can('server:create') && (
				<div className="flex flex-wrap items-center gap-4 border-t border-border/50 bg-muted/10 px-5 py-3.5">
					<span className="inline-flex items-center gap-1.5 text-xs font-semibold text-muted-foreground">
						<Trash2 className="size-3.5" />
						Prune:
					</span>
					{PRUNE_TARGETS.map(([key, label]) => (
						<label
							key={key}
							className="flex cursor-pointer items-center gap-2 text-xs text-foreground/80">
							<Checkbox
								checked={prune[key]}
								onCheckedChange={checked =>
									setPrune(prev => ({...prev, [key]: checked === true}))
								}
							/>
							{label}
						</label>
					))}
					<Button
						size="sm"
						variant="destructive"
						onClick={handlePrune}
						disabled={
							pruneMutation.isPending ||
							!Object.values(prune).some(Boolean)
						}
						className="ml-auto h-8 cursor-pointer text-xs">
						{pruneMutation.isPending ? 'Pruning…' : 'Run Prune'}
					</Button>
				</div>
			)}
		</div>
	);
}
