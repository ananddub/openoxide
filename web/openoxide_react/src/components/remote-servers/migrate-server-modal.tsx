import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';
import {formatApiError, serverHostLabel} from '#/api/utils';
import {
	ArrowRight,
	ArrowLeftRight,
	ChevronLeft,
	RefreshCw,
	RotateCcw,
	ShieldCheck,
	CheckCircle2,
	XCircle,
	Clock,
} from 'lucide-react';
import {
	useRemoteServerMigrateDependencies,
	useRemoteServerMigration,
	useRemoteServerRollbackMigration,
	type ServerMigrationPayload,
} from '#/api/live';

import type {RemoteServerResponse} from '#/types/api-helpers';

interface MigrateServerModalProps {
	isOpen: boolean;
	/** Server whose hosted services will be repointed (the source node). */
	server: RemoteServerResponse | null;
	/** All servers in the org — used to pick the migration target. */
	servers: RemoteServerResponse[];
	onClose: () => void;
}

/** Terminal states — polling stops and the rollback affordance is offered. */
const TERMINAL = new Set(['SUCCEEDED', 'FAILED', 'ROLLED_BACK']);

function statusMeta(status: string) {
	switch (status) {
		case 'SUCCEEDED':
			return {
				icon: <CheckCircle2 className="h-4 w-4 text-emerald-500" />,
				cls: 'text-emerald-500',
			};
		case 'FAILED':
			return {icon: <XCircle className="h-4 w-4 text-rose-500" />, cls: 'text-rose-500'};
		case 'RUNNING':
			return {
				icon: <RefreshCw className="h-4 w-4 animate-spin text-primary" />,
				cls: 'text-primary',
			};
		default:
			return {icon: <Clock className="h-4 w-4 text-amber-500" />, cls: 'text-amber-500'};
	}
}

/** Resource counts surfaced once a migration row exists. */
function ProgressGrid({migration}: {migration: ServerMigrationPayload}) {
	const rows: Array<[string, number]> = [
		['Applications', migration.applications],
		['Compose projects', migration.compose_projects],
		['Databases', migration.databases],
		['Build assignments', migration.build_assignments],
		['Certificates', migration.certificates],
		['Schedules', migration.schedules],
	];
	return (
		<div className="grid grid-cols-2 gap-2 rounded-xl bg-muted/20 p-3.5 text-xs">
			{rows.map(([label, value]) => (
				<div key={label} className="flex items-center justify-between">
					<span className="text-muted-foreground">{label}</span>
					<span className="font-mono font-semibold text-foreground">
						{value}
					</span>
				</div>
			))}
			{(migration.queued_applications > 0 ||
				migration.queued_compose_projects > 0) && (
				<div className="col-span-2 flex items-center justify-between border-t pt-2">
					<span className="text-muted-foreground">
						Redeploys queued on target
					</span>
					<span className="font-mono font-semibold text-foreground">
						{migration.queued_applications + migration.queued_compose_projects}
					</span>
				</div>
			)}
		</div>
	);
}

export function MigrateServerModal({
	isOpen,
	server,
	servers,
	onClose,
}: MigrateServerModalProps) {
	const [targetId, setTargetId] = useState<string>('');
	const [migrationId, setMigrationId] = useState<string>('');
	const [starting, setStarting] = useState(false);

	const migrateMutation = useRemoteServerMigrateDependencies();
	const rollbackMutation = useRemoteServerRollbackMigration();
	// Poll while the migration is non-terminal; table-watch also refreshes it.
	const migrationQuery = useRemoteServerMigration(migrationId, {
		pollMs: 2000,
	});
	const migration = migrationQuery.data;
	const isRunning = !!migration && !TERMINAL.has(migration.status);

	const candidates = servers.filter(s => s.id !== server?.id);

	const reset = () => {
		setTargetId('');
		setMigrationId('');
		setStarting(false);
	};

	const handleClose = () => {
		// Allow closing anytime; the migration keeps running server-side and
		// can be resumed/rolled back from the row's live status later.
		reset();
		onClose();
	};

	const handleStart = async () => {
		if (!server?.id || !targetId) return;
		setStarting(true);
		try {
			const result = await migrateMutation.mutateAsync({
				id: server.id,
				target_server_id: BigInt(targetId),
			});
			const dto = JSON.parse(result.payload || '{}') as ServerMigrationPayload;
			if (!dto.migration_id) {
				throw new Error('migration did not return an id');
			}
			setMigrationId(dto.migration_id);
			toast.success('Migration started — repointing services to target');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setStarting(false);
		}
	};

	const handleRollback = async () => {
		if (!migrationId) return;
		try {
			await rollbackMutation.mutateAsync(migrationId);
			toast.success('Rollback requested — source reassignments restored');
			await migrationQuery.refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	if (!server) return null;

	const target = servers.find(s => String(s.id) === targetId);
	const meta = migration ? statusMeta(migration.status) : null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
			<DialogContent className="border- w-full rounded-xl bg-card p-6 shadow-xl sm:max-w-xl md:max-w-2xl">
				<DialogHeader className="border-b pb-3">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
						<ArrowLeftRight className="h-5 w-5 text-primary" />
						Migrate Services: {server.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Repoint everything hosted on{' '}
						{serverHostLabel(server.ip_address)} to another managed node.
						The source server stays intact until you confirm.
					</DialogDescription>
				</DialogHeader>

				{!migrationId ? (
					/* ── Step 1: pick a target and start ── */
					<div className="flex flex-col gap-4 py-3">
						<div className="flex flex-col gap-2">
							<label className="text-[11px] font-semibold text-foreground">
								Target server
							</label>
							<Select value={targetId} onValueChange={v => setTargetId(v ?? '')}>
								<SelectTrigger size="sm" className="h-9 w-full text-xs">
									<SelectValue placeholder="Select a destination node…" />
								</SelectTrigger>
								<SelectContent>
									{candidates.map(s => (
										<SelectItem key={s.id} value={String(s.id)}>
											{s.name} · {serverHostLabel(s.ip_address)}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
							{candidates.length === 0 && (
								<p className="text-[11px] text-amber-500">
									No other servers available — add a destination node first.
								</p>
							)}
						</div>

						<div className="flex items-center justify-center gap-3 rounded-xl bg-primary/10 p-3.5 text-xs">
							<span className="font-mono text-foreground">
								{serverHostLabel(server.ip_address)}
							</span>
							<ArrowRight className="size-4 text-primary" />
							<span className="font-mono text-foreground">
								{target ? serverHostLabel(target.ip_address) : '—'}
							</span>
						</div>

						<div className="flex items-center gap-2.5 rounded-xl bg-muted/20 p-3.5">
							<ShieldCheck className="size-5 shrink-0 text-primary" />
							<p className="text-[11px] text-muted-foreground">
								Applications, compose projects, databases, certificates and
								schedules owned by this node are reassigned, then redeployed
								on the target.
							</p>
						</div>

						<div className="flex justify-end gap-2">
							<Button
								variant="ghost"
								size="sm"
								onClick={handleClose}
								className="h-8 text-xs">
								Cancel
							</Button>
							<Button
								size="sm"
								onClick={handleStart}
								disabled={!targetId || starting}
								className="h-8 gap-1.5 text-xs font-semibold">
								{starting ? (
									<RefreshCw className="h-3.5 w-3.5 animate-spin" />
								) : (
									<ArrowLeftRight className="h-3.5 w-3.5" />
								)}
								{starting ? 'Starting…' : 'Start Migration'}
							</Button>
						</div>
					</div>
				) : (
					/* ── Step 2: live progress + rollback ── */
					<div className="flex flex-col gap-4 py-3">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-2">
								<button
									onClick={() => setMigrationId('')}
									className="flex cursor-pointer items-center gap-1 text-xs text-muted-foreground hover:text-foreground">
									<ChevronLeft className="size-3.5" /> New migration
								</button>
							</div>
							{meta && (
								<span
									className={`flex items-center gap-1.5 text-xs font-semibold ${meta.cls}`}>
									{meta.icon} {migration?.status}
								</span>
							)}
						</div>

						{migrationQuery.isLoading && (
							<p className="text-xs text-muted-foreground">
								Loading migration status…
							</p>
						)}

						{migration && <ProgressGrid migration={migration} />}

						{migration?.error && (
							<div className="rounded-xl bg-rose-500/10 p-3.5 text-[11px] text-rose-500">
								{migration.error}
							</div>
						)}

						{isRunning && (
							<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
								<RefreshCw className="size-3 animate-spin" /> Running — you can
								close this dialog; it keeps going in the background.
							</p>
						)}

						<div className="flex justify-end gap-2">
							{migration && TERMINAL.has(migration.status) && migration.status !== 'ROLLED_BACK' && (
								<Button
									variant="outline"
									size="sm"
									onClick={handleRollback}
									disabled={rollbackMutation.isPending}
									className="h-8 gap-1.5 text-xs font-semibold">
									{rollbackMutation.isPending ? (
										<RefreshCw className="h-3.5 w-3.5 animate-spin" />
									) : (
										<RotateCcw className="h-3.5 w-3.5" />
									)}
									Roll Back
								</Button>
							)}
							<Button
								size="sm"
								onClick={handleClose}
								className="h-8 text-xs font-semibold">
								Done
							</Button>
						</div>
					</div>
				)}
			</DialogContent>
		</Dialog>
	);
}
