import {useState} from 'react';
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

	const patchResources = $api.useMutation('patch', '/applications/{id}/resources');
	const deleteApplication = $api.useMutation('delete', '/applications/{id}');

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
			<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Resource Limits</h3>
					<p className="text-xs text-muted-foreground mt-1">
						Define memory limits (e.g. <code className="font-mono bg-muted/30 px-1 py-0.5 rounded text-[11px]">512m</code>, <code className="font-mono bg-muted/30 px-1 py-0.5 rounded text-[11px]">1g</code>) and CPU bounds (e.g. <code className="font-mono bg-muted/30 px-1 py-0.5 rounded text-[11px]">0.5</code>)
					</p>
				</div>

				<div className="grid grid-cols-2 gap-4">
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">Memory Reservation</span>
						<Input placeholder="e.g. 256m" value={memRes} onChange={e => setMemRes(e.target.value)} className="bg-card border-border text-xs" />
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">Memory Limit</span>
						<Input placeholder="e.g. 512m" value={memLimit} onChange={e => setMemLimit(e.target.value)} className="bg-card border-border text-xs" />
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">CPU Reservation</span>
						<Input placeholder="e.g. 0.1" value={cpuRes} onChange={e => setCpuRes(e.target.value)} className="bg-card border-border text-xs" />
					</div>
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">CPU Limit</span>
						<Input placeholder="e.g. 0.5" value={cpuLimit} onChange={e => setCpuLimit(e.target.value)} className="bg-card border-border text-xs" />
					</div>
				</div>

				<div className="flex flex-col gap-1.5 w-32 mt-1">
					<span className="text-xs font-bold text-muted-foreground">Replicas</span>
					<Input type="number" min="1" max="10" value={replicas} onChange={e => setReplicas(e.target.value)} className="bg-card border-border text-xs" />
				</div>

				<div className="flex justify-end mt-2">
					<Button onClick={handleSaveResources} disabled={saving} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
						<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Resource Limits'}
					</Button>
				</div>
			</section>

			{/* Danger zone */}
			<section className="bg-card border border-destructive/30 rounded-xl p-5 flex flex-col gap-4">
				<div className="flex items-center gap-2 text-destructive">
					<AlertTriangle className="w-4 h-4" />
					<h3 className="text-sm font-bold">Danger Zone</h3>
				</div>
				<p className="text-xs text-muted-foreground">Permanently remove this application and all associated data, settings, domains, and routes.</p>

				{confirmDelete ? (
					<div className="bg-destructive/5 border border-destructive/20 rounded-lg p-4 flex flex-col gap-3">
						<p className="text-xs text-foreground font-semibold">Are you sure? This action is absolutely irreversible.</p>
						<div className="flex items-center gap-2">
							<Button onClick={handleDeleteApp} disabled={deleting} className="bg-destructive hover:bg-destructive/90 text-destructive-foreground font-semibold text-xs h-8">
								{deleting ? 'Deleting...' : 'Yes, Delete application'}
							</Button>
							<Button variant="outline" onClick={() => setConfirmDelete(false)} className="border-border text-foreground hover:bg-muted font-semibold text-xs h-8">
								Cancel
							</Button>
						</div>
					</div>
				) : (
					<div>
						<Button variant="outline" onClick={() => setConfirmDelete(true)} className="border-destructive/40 hover:bg-destructive/5 hover:text-destructive text-destructive/80 font-semibold text-xs h-9 flex items-center gap-1.5">
							<Trash2 className="w-4 h-4" /> Delete Application
						</Button>
					</div>
				)}
			</section>
		</div>
	);
}
