import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {formatApiError, serverHostLabel} from '#/api/utils';
import {ArrowLeftRight} from 'lucide-react';
import {
	useRemoteServerMigrateDependencies,
	useRemoteServerMigration,
	useRemoteServerRollbackMigration,
	type ServerMigrationPayload,
} from '#/api/live';

import type {RemoteServerResponse} from '#/types/api-helpers';
import {StatefulMigrationPanel} from './stateful-migration-panel';
import {
	MigrationStartStep,
	MigrationProgressStep,
} from './migration-steps';

interface MigrateServerModalProps {
	isOpen: boolean;
	/** Server whose hosted services will be repointed (the source node). */
	server: RemoteServerResponse | null;
	/** All servers in the org — used to pick the migration target. */
	servers: RemoteServerResponse[];
	onClose: () => void;
}

/** Wizard modes: fast metadata-only move vs resumable per-service rsync. */
type WizardMode = 'metadata' | 'stateful';

export function MigrateServerModal({
	isOpen,
	server,
	servers,
	onClose,
}: MigrateServerModalProps) {
	const [targetId, setTargetId] = useState<string>('');
	const [migrationId, setMigrationId] = useState<string>('');
	const [starting, setStarting] = useState(false);
	const [mode, setMode] = useState<WizardMode>('metadata');

	const migrateMutation = useRemoteServerMigrateDependencies();
	const rollbackMutation = useRemoteServerRollbackMigration();
	// Poll while the migration is non-terminal; table-watch also refreshes it.
	const migrationQuery = useRemoteServerMigration(migrationId, {
		pollMs: 2000,
	});
	const migration = migrationQuery.data;

	const candidates = servers.filter(s => s.id !== server?.id);

	const reset = () => {
		setTargetId('');
		setMigrationId('');
		setStarting(false);
		setMode('metadata');
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
			const dto = JSON.parse(
				result.payload || '{}',
			) as ServerMigrationPayload;
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

	const modeButton = (value: WizardMode, label: string) => (
		<button
			type="button"
			onClick={() => setMode(value)}
			className={`flex-1 cursor-pointer rounded-md px-3 py-1.5 font-semibold transition-colors ${
				mode === value
					? 'bg-card text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'
			}`}>
			{label}
		</button>
	);

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
			<DialogContent className="border- w-full rounded-xl bg-card p-6 shadow-md sm:max-w-xl md:max-w-2xl">
				<DialogHeader className="border-b pb-3">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
						<ArrowLeftRight className="h-5 w-5 text-primary" />
						Migrate Services: {server.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Move workloads off {serverHostLabel(server.ip_address)} to
						another managed node. The source server stays intact until you
						confirm.
					</DialogDescription>
				</DialogHeader>

				{/* ── Mode switch: whole-server metadata vs single-service stateful ── */}
				<div className="mt-3 flex gap-1 rounded-lg bg-muted/30 p-1 text-xs">
					{modeButton('metadata', 'Whole server (metadata)')}
					{modeButton('stateful', 'Single service (with data)')}
				</div>

				{mode === 'stateful' ? (
					<StatefulMigrationPanel server={server} servers={servers} />
				) : !migrationId ? (
					/* ── Step 1: pick a target and start ── */
					<MigrationStartStep
						server={server}
						candidates={candidates}
						targetId={targetId}
						onTarget={setTargetId}
						starting={starting}
						onStart={() => void handleStart()}
						onCancel={handleClose}
					/>
				) : (
					/* ── Step 2: live progress + rollback ── */
					<MigrationProgressStep
						migration={migration}
						isLoading={migrationQuery.isLoading}
						onNew={() => setMigrationId('')}
						rollbackPending={rollbackMutation.isPending}
						onRollback={() => void handleRollback()}
						onClose={handleClose}
					/>
				)}
			</DialogContent>
		</Dialog>
	);
}
