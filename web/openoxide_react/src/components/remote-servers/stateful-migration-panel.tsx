import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {formatApiError, serverHostLabel} from '#/api/utils';
import {ArrowLeftRight, Ban, Database, RefreshCw} from 'lucide-react';
import {
	useMigrationObjects,
	useOverviewServices,
	useRemoteServerCancelMigration,
	useRemoteServerMigrateService,
	useServiceMigrations,
	type StatefulMigrationPayload,
} from '#/api/live';

import type {RemoteServerResponse} from '#/types/api-helpers';
import {isActive} from './stateful-migration-parts';
import {StatefulMigrationCard} from './stateful-migration-card';
import {MigrationSelect} from './migration-select';

interface StatefulMigrationPanelProps {
	/** Source node — only services currently assigned to it are offered. */
	server: RemoteServerResponse;
	/** All servers in the org — used to pick the migration target. */
	servers: RemoteServerResponse[];
}

/** Single-service stateful migration: rsyncs the chosen service's data (source dir, volumes, binds) into the target's staging area, verifies, promotes, repoints only that service, redeploys and health-gates it — the source keeps everything until REPOINT, and cancel is offered while the row is still RUNNING. */
export function StatefulMigrationPanel({
	server,
	servers,
}: StatefulMigrationPanelProps) {
	const [serviceId, setServiceId] = useState<string>('');
	const [targetId, setTargetId] = useState<string>('');
	const [starting, setStarting] = useState(false);

	const {data: services} = useOverviewServices();
	const migrateMutation = useRemoteServerMigrateService();
	const cancelMutation = useRemoteServerCancelMigration();

	const migrationsQuery = useServiceMigrations(
		serviceId ? BigInt(serviceId) : 0n,
		{
			pollMs: 2000,
		},
	);
	const latest: StatefulMigrationPayload | undefined =
		migrationsQuery.data?.[0];
	const objectsQuery = useMigrationObjects(latest?.id ?? '', {
		pollMs: 2000,
	});

	const movable = (services ?? []).filter(
		s =>
			s.server_id !== null &&
			s.server_id === BigInt(server.id) &&
			s.service_type !== 'DATABASE',
	);
	const candidates = servers.filter(s => s.id !== server.id);
	const polling = isActive(latest);

	const handleStart = async () => {
		if (!serviceId || !targetId) return;
		setStarting(true);
		try {
			const row = await migrateMutation.mutateAsync({
				serviceId: BigInt(serviceId),
				targetServerId: BigInt(targetId),
			});
			toast.success(
				`Migration ${row.id.slice(0, 8)}… started — syncing to target`,
			);
			await migrationsQuery.refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setStarting(false);
		}
	};

	const handleCancel = async () => {
		if (!latest) return;
		try {
			await cancelMutation.mutateAsync(latest.id);
			toast.info(
				'Cancellation requested — the runner stops at the next boundary',
			);
			await migrationsQuery.refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-4 py-3">
			<div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
				<MigrationSelect
					label="Service"
					value={serviceId}
					placeholder="Pick a service on this node…"
					options={movable.map(s => ({
						value: String(s.id),
						label: `${s.name} · ${s.service_type}`,
					}))}
					emptyHint="No migratable services on this node (databases are not supported yet)."
					onChange={setServiceId}
				/>
				<MigrationSelect
					label="Target server"
					value={targetId}
					placeholder="Select a destination node…"
					options={candidates.map(s => ({
						value: String(s.id),
						label: `${s.name} · ${serverHostLabel(s.ip_address)}`,
					}))}
					onChange={setTargetId}
				/>
			</div>

			<div className="flex items-center gap-2.5 rounded-xl bg-muted/20 p-3.5">
				<Database className="size-5 shrink-0 text-primary" />
				<p className="text-[11px] text-muted-foreground">
					The service&apos;s source directory, docker volumes and bind
					mounts are rsynced to the target and verified before anything is
					repointed. The source workload stays stopped-but-intact until the
					target passes its health gate; cancel is available up to the
					repoint step.
				</p>
			</div>

			{latest && (
				<StatefulMigrationCard
					latest={latest}
					polling={polling}
					objects={objectsQuery.data}
				/>
			)}

			<div className="flex justify-end gap-2">
				{latest && latest.status === 'RUNNING' && (
					<Button
						variant="outline"
						size="sm"
						onClick={handleCancel}
						disabled={cancelMutation.isPending}
						className="h-8 gap-1.5 text-xs font-semibold text-rose-500">
						<Ban className="h-3.5 w-3.5" />
						Cancel
					</Button>
				)}
				<Button
					size="sm"
					onClick={handleStart}
					disabled={
						!serviceId || !targetId || starting || isActive(latest)
					}
					className="h-8 gap-1.5 text-xs font-semibold">
					{starting ? (
						<RefreshCw className="h-3.5 w-3.5 animate-spin" />
					) : (
						<ArrowLeftRight className="h-3.5 w-3.5" />
					)}
					{starting ? 'Starting…' : 'Start Stateful Migration'}
				</Button>
			</div>
		</div>
	);
}
