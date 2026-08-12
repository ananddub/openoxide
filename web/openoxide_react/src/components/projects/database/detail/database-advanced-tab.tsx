import {useState, useEffect} from 'react';
import {Trash2, AlertTriangle, Save, RefreshCw, Cpu} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DeleteDatabaseDialog} from '#/components/projects/database/delete-database-dialog';

interface DatabaseAdvancedTabProps {
	database: any;
	onUpdated: () => void;
	onAction?: (action: 'deploy' | 'reload' | 'start' | 'stop') => Promise<void>;
}

export function DatabaseAdvancedTab({database, onUpdated, onAction}: DatabaseAdvancedTabProps) {
	const kind = (database?.kind || database?.database_kind || 'postgres').toLowerCase();

	let endpoint: '/postgres/{id}' | '/mysql/{id}' | '/mariadb/{id}' | '/mongo/{id}' | '/redis/{id}' | '/libsql/{id}' = '/postgres/{id}';
	if (kind.includes('mysql')) endpoint = '/mysql/{id}';
	else if (kind.includes('mariadb')) endpoint = '/mariadb/{id}';
	else if (kind.includes('mongo')) endpoint = '/mongo/{id}';
	else if (kind.includes('redis')) endpoint = '/redis/{id}';
	else if (kind.includes('libsql')) endpoint = '/libsql/{id}';

	const patchDatabase = $api.useMutation('patch', endpoint as any);
	const redeployDatabase = $api.useMutation('post', `${endpoint}/redeploy` as any);

	const [replicas, setReplicas] = useState(String(database?.replicas || '1'));
	const [memRes, setMemRes] = useState(database?.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(database?.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(database?.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(database?.cpu_limit || '');
	const [volumeMount, setVolumeMount] = useState(database?.volume_mount || '');

	const [saving, setSaving] = useState(false);
	const [rebuilding, setRebuilding] = useState(false);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	useEffect(() => {
		setReplicas(String(database?.replicas || '1'));
		setMemRes(database?.memory_reservation || '');
		setMemLimit(database?.memory_limit || '');
		setCpuRes(database?.cpu_reservation || '');
		setCpuLimit(database?.cpu_limit || '');
		setVolumeMount(database?.volume_mount || '');
	}, [database]);

	const handleSaveResources = async () => {
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				params: {path: {id: database?.id}},
				body: {
					memory_reservation: memRes || undefined,
					memory_limit: memLimit || undefined,
					cpu_reservation: cpuRes || undefined,
					cpu_limit: cpuLimit || undefined,
					replicas: replicas ? parseInt(replicas) : 1,
				},
			});
			toast.success('Database resources updated successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const handleRebuild = async () => {
		setRebuilding(true);
		try {
			if (onAction) {
				await onAction('deploy');
			} else {
				await redeployDatabase.mutateAsync({params: {path: {id: database?.id}}});
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
		<div className="flex flex-col gap-6 w-full animate-in fade-in duration-200">
			{/* Replicas & Resource Limits Card */}
			<section className="bg-card border border-border/80 rounded-2xl p-6 shadow-sm flex flex-col gap-5">
				<div>
					<h3 className="text-base font-bold text-foreground tracking-tight flex items-center gap-2">
						<Cpu className="size-4 text-primary" /> Container Replicas & Resource Limits
					</h3>
					<p className="text-xs text-muted-foreground mt-0.5">
						Configure instance replicas, memory reservation/limit (e.g. <code className="font-mono bg-muted/40 px-1 py-0.5 rounded text-[11px]">512m</code>, <code className="font-mono bg-muted/40 px-1 py-0.5 rounded text-[11px]">2g</code>), and CPU allocation.
					</p>
				</div>

				<div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-t border-border/40 pt-4">
					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">Replicas</label>
						<Input type="number" min="1" max="10" value={replicas} onChange={e => setReplicas(e.target.value)} className="h-9 text-xs" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">Volume Mount Path</label>
						<Input placeholder="Auto-generated data volume" value={volumeMount} onChange={e => setVolumeMount(e.target.value)} className="h-9 text-xs font-mono" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">Memory Reservation</label>
						<Input placeholder="e.g. 256m" value={memRes} onChange={e => setMemRes(e.target.value)} className="h-9 text-xs" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">Memory Limit</label>
						<Input placeholder="e.g. 1g" value={memLimit} onChange={e => setMemLimit(e.target.value)} className="h-9 text-xs" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">CPU Reservation</label>
						<Input placeholder="e.g. 0.2" value={cpuRes} onChange={e => setCpuRes(e.target.value)} className="h-9 text-xs" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">CPU Limit</label>
						<Input placeholder="e.g. 1.0" value={cpuLimit} onChange={e => setCpuLimit(e.target.value)} className="h-9 text-xs" />
					</div>
				</div>

				<div className="flex justify-end border-t border-border/40 pt-4">
					<Button onClick={handleSaveResources} disabled={saving} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
						<Save className="size-3.5" /> {saving ? 'Saving...' : 'Save Configuration'}
					</Button>
				</div>
			</section>

			{/* Danger Zone Card */}
			<section className="bg-card border border-destructive/30 rounded-2xl p-6 shadow-sm flex flex-col gap-5">
				<div className="flex items-center gap-2 text-destructive">
					<AlertTriangle className="size-5" />
					<div>
						<h3 className="text-base font-bold tracking-tight">Danger Zone</h3>
						<p className="text-xs text-muted-foreground mt-0.5">Rebuild database container or permanently delete this database service.</p>
					</div>
				</div>

				<div className="flex flex-col gap-4 border-t border-border/40 pt-4">
					{/* Rebuild Database */}
					<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-4 rounded-xl border border-border/60 bg-muted/10">
						<div>
							<p className="text-xs font-bold text-foreground flex items-center gap-1.5">
								<RefreshCw className="size-3.5 text-amber-500" /> Rebuild Database
							</p>
							<p className="text-[11px] text-muted-foreground mt-0.5">Forces container re-creation and applies updated volume/image settings.</p>
						</div>
						<Button
							variant="outline"
							onClick={handleRebuild}
							disabled={rebuilding}
							className="h-9 px-4 text-xs font-semibold border-amber-500/30 text-amber-500 hover:bg-amber-500/10 shrink-0">
							{rebuilding ? 'Rebuilding...' : 'Rebuild Database'}
						</Button>
					</div>

					{/* Delete Database (Opens Popup Dialog) */}
					<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-4 rounded-xl border border-destructive/20 bg-destructive/5">
						<div>
							<p className="text-xs font-bold text-destructive flex items-center gap-1.5">
								<Trash2 className="size-3.5 text-destructive" /> Delete Database
							</p>
							<p className="text-[11px] text-muted-foreground mt-0.5">Permanently remove this database container, environment configs, and volume connections.</p>
						</div>

						<Button
							variant="outline"
							onClick={() => setIsDeleteDialogOpen(true)}
							className="h-9 px-4 text-xs font-semibold border-destructive/40 text-destructive hover:bg-destructive/10 shrink-0">
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
