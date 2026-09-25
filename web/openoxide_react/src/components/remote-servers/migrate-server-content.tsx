import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerMigrateDependencies,
	useRemoteServerMigration,
	useRemoteServerRollbackMigration,
} from '#/api/live';

import type {RemoteServerResponse} from '#/types/api-helpers';
import {StatefulMigrationPanel} from './stateful-migration-panel';
import {
	MigrationStartStep,
	MigrationProgressStep,
} from './migration-steps';

/** Wizard modes: fast metadata-only move vs resumable per-service rsync. */
type WizardMode = 'metadata' | 'stateful';

/**
 * Migration flow as a plain content block (mode switch + steps) so it can
 * live inside the server settings panel. `onClose` fires when a step asks
 * to leave the flow (e.g. progress "Done").
 */
export function MigrateServerContent({
	server,
	servers,
	onClose,
}: {
	server: RemoteServerResponse;
	servers: RemoteServerResponse[];
	onClose: () => void;
}) {
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

	const candidates = servers.filter(s => s.id !== server.id);

	const handleStart = async () => {
		if (!targetId) return;
		setStarting(true);
		try {
			const dto = await migrateMutation.mutateAsync({
				id: server.id,
				target_server_id: BigInt(targetId),
			});
			if (!dto?.migration_id) {
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
		<div className="flex min-w-0 flex-col gap-4">
			{/* ── Mode switch: whole-server metadata vs single-service stateful ── */}
			<div className="flex gap-1 rounded-lg bg-muted/30 p-1 text-xs">
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
					onCancel={onClose}
				/>
			) : (
				/* ── Step 2: live progress + rollback ── */
				<MigrationProgressStep
					migration={migration}
					isLoading={migrationQuery.isLoading}
					onNew={() => setMigrationId('')}
					rollbackPending={rollbackMutation.isPending}
					onRollback={() => void handleRollback()}
					onClose={onClose}
				/>
			)}
		</div>
	);
}
