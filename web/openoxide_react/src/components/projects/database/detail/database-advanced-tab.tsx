import {useState} from 'react';
import {Trash2, AlertTriangle, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {useDatabaseRedeploy, parseDatabaseKind} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {DeleteDatabaseDialog} from '#/components/projects/database/delete-database-dialog';
import {DatabaseAdvancedCustomCommand} from './database-advanced-custom-command';
import {DatabaseAdvancedResources} from './database-advanced-resources';
import {DatabaseAdvancedMetadata} from './database-advanced-metadata';
import {DatabaseAdvancedNetworks} from './database-advanced-networks';
import {DatabaseAdvancedVolumes} from './database-advanced-volumes';
import {DatabasePostgresConfig} from './database-postgres-config';
import {databaseKind} from './database-endpoint';
import {DatabaseSwarmSettings} from './database-swarm-settings';

interface DatabaseAdvancedTabProps {
	database: any;
	onUpdated: () => void;
	onAction?: (
		action: 'deploy' | 'reload' | 'start' | 'stop',
	) => Promise<void>;
}

export function DatabaseAdvancedTab({
	database,
	onUpdated,
	onAction,
}: DatabaseAdvancedTabProps) {
	const kind = (
		database?.kind ||
		database?.database_kind ||
		'postgres'
	).toLowerCase();
	const dbKind = parseDatabaseKind(kind);
	const redeployDatabase = useDatabaseRedeploy();
	const [rebuilding, setRebuilding] = useState(false);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	const handleRebuild = async () => {
		setRebuilding(true);
		try {
			if (onAction) {
				await onAction('deploy');
			} else {
				await redeployDatabase.mutateAsync({
					kind: dbKind,
					id: database?.id,
				});
				toast.success('Database rebuild triggered');
			}
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setRebuilding(false);
		}
	};

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			<DatabaseAdvancedMetadata database={database} onUpdated={onUpdated} />
			<DatabaseAdvancedCustomCommand
				database={database}
				onUpdated={onUpdated}
			/>
			{databaseKind(database) === 'postgres' && (
				<DatabasePostgresConfig database={database} onUpdated={onUpdated} />
			)}

			{/* 2. Replicas & Resource Limits (Memory / CPU Limits & Reservations) */}
			<DatabaseAdvancedResources
				database={database}
				onUpdated={onUpdated}
			/>
			<DatabaseAdvancedVolumes database={database} onUpdated={onUpdated} />
			<DatabaseAdvancedNetworks database={database} onUpdated={onUpdated} />
			<DatabaseSwarmSettings database={database} onUpdated={onUpdated} />

			{/* 3. Danger Zone Card */}
			<section className="flex flex-col gap-5 rounded-2xl bg-card p-6 shadow-sm">
				<div className="flex items-center gap-2 text-destructive">
					<AlertTriangle className="size-5" />
					<div>
						<h3 className="text-base font-bold tracking-tight">
							Danger Zone
						</h3>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Rebuild database container or permanently delete this
							database service.
						</p>
					</div>
				</div>

				<div className="flex flex-col gap-4 border-t  pt-4">
					{/* Rebuild Database */}
					<div className="flex flex-col justify-between gap-3 rounded-xl bg-muted/10 p-4 sm:flex-row sm:items-center">
						<div>
							<p className="flex items-center gap-1.5 text-xs font-bold text-foreground">
								<RefreshCw className="size-3.5 text-amber-500" /> Rebuild
								Database
							</p>
							<p className="mt-0.5 text-[11px] text-muted-foreground">
								Forces container re-creation and applies updated
								volume/image settings.
							</p>
						</div>
						<Button
							variant="outline"
							onClick={handleRebuild}
							disabled={rebuilding}
							className="h-9 shrink-0 border-amber-500/30 px-4 text-xs font-semibold text-amber-500 hover:bg-amber-500/10">
							{rebuilding ? 'Rebuilding...' : 'Rebuild Database'}
						</Button>
					</div>

					{/* Delete Database (Opens Popup Dialog) */}
					<div className="flex flex-col justify-between gap-3 rounded-xl bg-destructive/5 p-4 sm:flex-row sm:items-center">
						<div>
							<p className="flex items-center gap-1.5 text-xs font-bold text-destructive">
								<Trash2 className="size-3.5 text-destructive" /> Delete
								Database
							</p>
							<p className="mt-0.5 text-[11px] text-muted-foreground">
								Permanently remove this database container, environment
								configs, and volume connections.
							</p>
						</div>

						<Button
							variant="outline"
							onClick={() => setIsDeleteDialogOpen(true)}
							className="h-9 shrink-0 border-destructive/40 px-4 text-xs font-semibold text-destructive hover:bg-destructive/10">
							Delete Database
						</Button>
					</div>
				</div>
			</section>

			{/* Delete Confirmation Popup Dialog */}
			<DeleteDatabaseDialog
				isOpen={isDeleteDialogOpen}
				onClose={() => setIsDeleteDialogOpen(false)}
				database={database}
			/>
		</div>
	);
}
