import {useState} from 'react';
import {Trash2, AlertTriangle, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DeleteDatabaseDialog} from '#/components/projects/database/delete-database-dialog';
import {DatabaseAdvancedCustomCommand} from './database-advanced-custom-command';
import {DatabaseAdvancedResources} from './database-advanced-resources';

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

	let endpoint:
		| '/postgres/{id}'
		| '/mysql/{id}'
		| '/mariadb/{id}'
		| '/mongo/{id}'
		| '/redis/{id}'
		| '/libsql/{id}' = '/postgres/{id}';
	if (kind.includes('mysql')) endpoint = '/mysql/{id}';
	else if (kind.includes('mariadb')) endpoint = '/mariadb/{id}';
	else if (kind.includes('mongo')) endpoint = '/mongo/{id}';
	else if (kind.includes('redis')) endpoint = '/redis/{id}';
	else if (kind.includes('libsql')) endpoint = '/libsql/{id}';

	const redeployDatabase = $api.useMutation(
		'post',
		`${endpoint}/redeploy` as any,
	);
	const [rebuilding, setRebuilding] = useState(false);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	const handleRebuild = async () => {
		setRebuilding(true);
		try {
			if (onAction) {
				await onAction('deploy');
			} else {
				await redeployDatabase.mutateAsync({
					params: {path: {id: database?.id}},
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
			{/* 1. Custom Docker Image, Command & Engine Args (Dokploy style) */}
			<DatabaseAdvancedCustomCommand
				database={database}
				onUpdated={onUpdated}
			/>

			{/* 2. Replicas & Resource Limits (Memory / CPU Limits & Reservations) */}
			<DatabaseAdvancedResources
				database={database}
				onUpdated={onUpdated}
			/>

			{/* 3. Danger Zone Card */}
			<section className="flex flex-col gap-5 rounded-2xl border border-destructive/30 bg-card p-6 shadow-sm">
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

				<div className="flex flex-col gap-4 border-t border-border/40 pt-4">
					{/* Rebuild Database */}
					<div className="flex flex-col justify-between gap-3 rounded-xl border border-border/60 bg-muted/10 p-4 sm:flex-row sm:items-center">
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
					<div className="flex flex-col justify-between gap-3 rounded-xl border border-destructive/20 bg-destructive/5 p-4 sm:flex-row sm:items-center">
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
