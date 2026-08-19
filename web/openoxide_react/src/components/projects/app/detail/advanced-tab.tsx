import {useState, useEffect} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {Trash2, AlertTriangle, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface AdvancedTabProps {
	app: any;
	onUpdated: () => void;
}

export function AdvancedTab({app, onUpdated}: AdvancedTabProps) {
	const navigate = useNavigate();
	const [memRes, setMemRes] = useState(app.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(app.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(app.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(app.cpu_limit || '');
	const [replicas, setReplicas] = useState(String(app.replicas || '1'));
	const [saving, setSaving] = useState(false);
	const [confirmDelete, setConfirmDelete] = useState(false);
	const [deleting, setDeleting] = useState(false);

	// Sync local state when app updates from parent refetch
	useEffect(() => {
		setMemRes(app.memory_reservation || '');
		setMemLimit(app.memory_limit || '');
		setCpuRes(app.cpu_reservation || '');
		setCpuLimit(app.cpu_limit || '');
		setReplicas(String(app.replicas || '1'));
	}, [
		app.memory_reservation,
		app.memory_limit,
		app.cpu_reservation,
		app.cpu_limit,
		app.replicas,
	]);

	const patchResources = $api.useMutation(
		'patch',
		'/applications/{id}/resources',
	);
	const deleteApplication = $api.useMutation(
		'delete',
		'/applications/{id}',
	);

	const handleSaveResources = async () => {
		setSaving(true);
		try {
			await patchResources.mutateAsync({
				params: {path: {id: app.id}},
				body: {
					memory_reservation: memRes || undefined,
					memory_limit: memLimit || undefined,
					cpu_reservation: cpuRes || undefined,
					cpu_limit: cpuLimit || undefined,
					replicas: replicas ? parseInt(replicas) : 1,
				},
			});
			toast.success('Resource limits saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const handleDeleteApp = async () => {
		setDeleting(true);
		try {
			await deleteApplication.mutateAsync({
				params: {path: {id: app.id}},
			});
			toast.success('Application deleted successfully');
			navigate({to: `/projects/${app.project_id}` as any});
		} catch (err: any) {
			toast.error(formatApiError(err));
			setDeleting(false);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Resource limits */}
			<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5">
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Container Resource Limits
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Define memory limits (e.g.{' '}
						<code className="rounded bg-muted/30 px-1 py-0.5 font-mono text-[11px]">
							512m
						</code>
						,{' '}
						<code className="rounded bg-muted/30 px-1 py-0.5 font-mono text-[11px]">
							1g
						</code>
						) and CPU bounds (e.g.{' '}
						<code className="rounded bg-muted/30 px-1 py-0.5 font-mono text-[11px]">
							0.5
						</code>
						)
					</p>
				</div>

				<div className="grid grid-cols-2 gap-4">
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							Memory Reservation
						</span>
						<Input
							placeholder="e.g. 256m"
							value={memRes}
							onChange={e => setMemRes(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							Memory Limit
						</span>
						<Input
							placeholder="e.g. 512m"
							value={memLimit}
							onChange={e => setMemLimit(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							CPU Reservation
						</span>
						<Input
							placeholder="e.g. 0.1"
							value={cpuRes}
							onChange={e => setCpuRes(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							CPU Limit
						</span>
						<Input
							placeholder="e.g. 0.5"
							value={cpuLimit}
							onChange={e => setCpuLimit(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
				</div>

				<div className="mt-1 flex w-32 flex-col gap-1.5">
					<span className="text-xs font-bold text-muted-foreground">
						Replicas
					</span>
					<Input
						type="number"
						min="1"
						max="10"
						value={replicas}
						onChange={e => setReplicas(e.target.value)}
						className="border-border bg-card text-xs"
					/>
				</div>

				<div className="mt-2 flex justify-end">
					<Button
						onClick={handleSaveResources}
						disabled={saving}
						className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
						<Save className="h-3.5 w-3.5" />{' '}
						{saving ? 'Saving...' : 'Save Resource Limits'}
					</Button>
				</div>
			</section>

			{/* Danger zone */}
			<section className="flex flex-col gap-4 rounded-xl border border-destructive/30 bg-card p-5">
				<div className="flex items-center gap-2 text-destructive">
					<AlertTriangle className="h-4 w-4" />
					<h3 className="text-sm font-bold">Danger Zone</h3>
				</div>
				<p className="text-xs text-muted-foreground">
					Permanently remove this application and all associated data,
					settings, domains, and routes.
				</p>

				{confirmDelete ? (
					<div className="flex flex-col gap-3 rounded-lg border border-destructive/20 bg-destructive/5 p-4">
						<p className="text-xs font-semibold text-foreground">
							Are you sure? This action is absolutely irreversible.
						</p>
						<div className="flex items-center gap-2">
							<Button
								onClick={handleDeleteApp}
								disabled={deleting}
								className="text-destructive-foreground h-9 bg-destructive px-6 text-xs font-semibold shadow-md hover:bg-destructive/90">
								{deleting ? 'Deleting...' : 'Yes, Delete application'}
							</Button>
						</div>
					</div>
				) : (
					<div>
						<Button
							variant="outline"
							onClick={() => setConfirmDelete(true)}
							className="flex h-9 items-center gap-1.5 border-destructive/40 text-xs font-semibold text-destructive/80 hover:bg-destructive/5 hover:text-destructive">
							<Trash2 className="h-4 w-4" /> Delete Application
						</Button>
					</div>
				)}
			</section>
		</div>
	);
}
