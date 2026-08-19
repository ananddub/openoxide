import {useState, useEffect} from 'react';
import {Zap, RefreshCw, XCircle, Activity, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useAppStore} from '#/stores/app-store';
import {DatabaseDeploymentsModal} from './database-deployments-modal';
import {DatabaseDeploymentsList} from './database-deployments-list';

interface DatabaseDeploymentsTabProps {
	dbId: number;
	kind?: string;
	database?: any;
	deployments?: any[];
	onRefresh?: () => void;
	onAction?: (action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel') => Promise<void>;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = ['BUILDING', 'PREPARING', 'QUEUE', 'QUEUED', 'STARTING', 'DEPLOYING', 'PENDING', 'DOCKER'];
	return activeKeywords.some((kw) => s.includes(kw) || st.includes(kw));
};

export function DatabaseDeploymentsTab({
	dbId,
	kind = 'postgres',
	database: _database,
	deployments: passedDeployments,
	onRefresh,
	onAction,
}: DatabaseDeploymentsTabProps) {
	const queryClient = useQueryClient();
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const [cancelingId, setCancelingId] = useState<number | null>(null);

	const storeDeployments = useAppStore((state) => state.deployments || []);
	const dbDeployments = storeDeployments.filter((d: any) => String(d.database_id) === String(dbId));
	const events = dbDeployments.length > 0 ? dbDeployments : (passedDeployments ?? []);
	const activeDeployment = events.find(isBuildActive);

	const deleteMutation = $api.useMutation('delete', '/deployments/{id}');
	const clearDbMutation = $api.useMutation('delete', '/deployments/database/{id}');

	useEffect(() => {
		if (!activeLogId) return;
		setLiveLogs([]);
		let isMounted = true;
		const controller = new AbortController();

		const startStream = async () => {
			try {
				const sessionRaw = localStorage.getItem('openoxide-auth-session');
				let accessToken = '';
				if (sessionRaw) {
					try {
						const parsed = JSON.parse(sessionRaw);
						accessToken = parsed?.tokens?.access_token || parsed?.state?.accessToken || parsed?.accessToken || '';
					} catch {}
				}

				const headers: Record<string, string> = {};
				if (accessToken) headers['Authorization'] = `Bearer ${accessToken}`;

				const response = await fetch(`/api/deployments/${activeLogId}/logs`, {
					headers,
					signal: controller.signal,
				});

				if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				let buffer = '';

				if (reader) {
					while (isMounted) {
						const {done, value} = await reader.read();
						if (done) break;
						buffer += decoder.decode(value, {stream: true});
						const lines = buffer.split('\n');
						buffer = lines.pop() || '';
						for (const line of lines) {
							if (line.startsWith('data:')) {
								try {
									const parsed = JSON.parse(line.slice(5).trim());
									if (parsed?.line && isMounted) setLiveLogs((prev) => [...prev, parsed.line]);
								} catch {
									if (isMounted) setLiveLogs((prev) => [...prev, line.slice(5)]);
								}
							}
						}
					}
				}
			} catch {}
		};

		startStream();
		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [activeLogId]);

	const getStatusBadge = (e: any) => {
		const s = (e.status || '').toUpperCase();
		if (s.includes('DONE') || s.includes('SUCCESS') || s.includes('DEPLOYED')) return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s.includes('FAIL') || s.includes('ERROR') || s.includes('CRASH')) return 'text-destructive bg-destructive/10 border-destructive/20';
		if (s.includes('BUILD') || s.includes('DEPLOY') || s.includes('QUEUE') || s.includes('START')) return 'text-amber-500 bg-amber-500/10 border-amber-500/20';
		return 'text-muted-foreground bg-muted border-border';
	};

	const handleClearDatabaseDeployments = async () => {
		try {
			const res = await clearDbMutation.mutateAsync({params: {path: {id: dbId}}});
			const data = res as any;
			useAppStore.getState().clearDeployments({databaseId: dbId});
			toast.success(`Cleared ${data?.cleared_count || 0} database deployment logs & history`);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteSingleDbDeployment = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			useAppStore.getState().deleteDeployment(id);
			toast.success(`Deployment #${id} deleted`);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const confirmCancel = async () => {
		if (!cancelingId) return;
		try {
			await onAction?.('cancel');
			toast.success('Database deployment cancellation requested');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setCancelingId(null);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						Database Container Deployments History
						{activeDeployment && (
							<span className="flex items-center gap-1 text-[10px] font-semibold text-amber-500 bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 rounded-full animate-pulse">
								<Activity className="w-3 h-3 animate-spin" /> Provisioning Container...
							</span>
						)}
					</h3>
					<p className="text-xs text-muted-foreground mt-0.5">
						Audit log of provisioning, database container deployment operations and container lifecycle events ({kind.toUpperCase()})
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button variant="outline" size="sm" onClick={handleClearDatabaseDeployments} className="border-destructive/30 text-destructive bg-destructive/10 hover:bg-destructive/20 font-semibold flex items-center gap-1.5 h-8 text-xs">
						<Trash2 className="w-3.5 h-3.5" /> Clear History
					</Button>
					<Button variant="outline" size="sm" onClick={() => onRefresh?.()} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs">
						<RefreshCw className="w-3.5 h-3.5" /> Refresh
					</Button>
					<Button variant="outline" size="sm" onClick={() => onAction?.('redeploy')} disabled={!!activeDeployment} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs disabled:opacity-50">
						<RefreshCw className="w-3.5 h-3.5" /> Redeploy Container
					</Button>
					{activeDeployment ? (
						<Button onClick={() => activeDeployment.id && setCancelingId(activeDeployment.id)} size="sm" variant="destructive" className="font-semibold flex items-center gap-1.5 h-8 text-xs">
							<XCircle className="w-3.5 h-3.5" /> Cancel Build
						</Button>
					) : (
						<Button onClick={() => onAction?.('deploy')} size="sm" className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
							<Zap className="w-3.5 h-3.5" /> Deploy Database
						</Button>
					)}
				</div>
			</section>

			{/* Deployments List Section */}
			<DatabaseDeploymentsList
				events={events}
				isBuildActive={isBuildActive}
				getStatusBadge={getStatusBadge}
				onOpenLogs={(id) => setActiveLogId(id)}
				onCancel={(id) => setCancelingId(id)}
				onDelete={handleDeleteSingleDbDeployment}
			/>

			{/* Modals for logs & cancellation */}
			<DatabaseDeploymentsModal
				activeLogId={activeLogId}
				liveLogs={liveLogs}
				cancelingId={cancelingId}
				onCloseLogs={() => setActiveLogId(null)}
				onCloseCancel={() => setCancelingId(null)}
				onConfirmCancel={confirmCancel}
			/>
		</div>
	);
}
