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
	onAction?: (
		action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel',
	) => Promise<void>;
}

const FINAL_STATES = [
	'DONE',
	'DEPLOYED',
	'SUCCESS',
	'FAILED',
	'ERROR',
	'CANCELLED',
	'STOPPEDBYUSER',
	'CRASHED',
];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = [
		'BUILDING',
		'PREPARING',
		'QUEUE',
		'QUEUED',
		'STARTING',
		'DEPLOYING',
		'PENDING',
		'DOCKER',
	];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
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

	const storeDeployments = useAppStore(state => state.deployments || []);
	const dbDeployments = storeDeployments.filter(
		(d: any) => String(d.database_id) === String(dbId),
	);
	const events =
		dbDeployments.length > 0 ? dbDeployments : (passedDeployments ?? []);
	const activeDeployment = events.find(isBuildActive);

	const deleteMutation = $api.useMutation('delete', '/deployments/{id}');
	const clearDbMutation = $api.useMutation(
		'delete',
		'/deployments/database/{id}',
	);

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
						accessToken =
							parsed?.tokens?.access_token ||
							parsed?.state?.accessToken ||
							parsed?.accessToken ||
							'';
					} catch {}
				}

				const headers: Record<string, string> = {};
				if (accessToken)
					headers['Authorization'] = `Bearer ${accessToken}`;

				const response = await fetch(
					`/api/deployments/${activeLogId}/logs`,
					{
						headers,
						signal: controller.signal,
					},
				);

				if (!response.ok)
					throw new Error(`HTTP error! status: ${response.status}`);

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
									if (parsed?.line && isMounted)
										setLiveLogs(prev => [...prev, parsed.line]);
								} catch {
									if (isMounted)
										setLiveLogs(prev => [...prev, line.slice(5)]);
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
		if (
			s.includes('DONE') ||
			s.includes('SUCCESS') ||
			s.includes('DEPLOYED')
		)
			return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s.includes('FAIL') || s.includes('ERROR') || s.includes('CRASH'))
			return 'text-destructive bg-destructive/10 border-destructive/20';
		if (
			s.includes('BUILD') ||
			s.includes('DEPLOY') ||
			s.includes('QUEUE') ||
			s.includes('START')
		)
			return 'text-amber-500 bg-amber-500/10 border-amber-500/20';
		return 'text-muted-foreground bg-muted border-border';
	};

	const handleClearDatabaseDeployments = async () => {
		try {
			const res = await clearDbMutation.mutateAsync({
				params: {path: {id: dbId}},
			});
			const data = res as any;
			useAppStore.getState().clearDeployments({databaseId: dbId});
			toast.success(
				`Cleared ${data?.cleared_count || 0} database deployment logs & history`,
			);
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
			<section className="flex items-center justify-between rounded-xl border border-border bg-card p-5">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						Database Container Deployments History
						{activeDeployment && (
							<span className="flex animate-pulse items-center gap-1 rounded-full border border-amber-500/20 bg-amber-500/10 px-2 py-0.5 text-[10px] font-semibold text-amber-500">
								<Activity className="h-3 w-3 animate-spin" /> Provisioning
								Container...
							</span>
						)}
					</h3>
					<p className="mt-0.5 text-xs text-muted-foreground">
						Audit log of provisioning, database container deployment
						operations and container lifecycle events ({kind.toUpperCase()}
						)
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={handleClearDatabaseDeployments}
						className="flex h-8 items-center gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
						<Trash2 className="h-3.5 w-3.5" /> Clear History
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onRefresh?.()}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw className="h-3.5 w-3.5" /> Refresh
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onAction?.('redeploy')}
						disabled={!!activeDeployment}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted disabled:opacity-50">
						<RefreshCw className="h-3.5 w-3.5" /> Redeploy Container
					</Button>
					{activeDeployment ? (
						<Button
							onClick={() =>
								activeDeployment.id && setCancelingId(activeDeployment.id)
							}
							size="sm"
							variant="destructive"
							className="flex h-8 items-center gap-1.5 text-xs font-semibold">
							<XCircle className="h-3.5 w-3.5" /> Cancel Build
						</Button>
					) : (
						<Button
							onClick={() => onAction?.('deploy')}
							size="sm"
							className="flex h-8 items-center gap-1.5 bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/90">
							<Zap className="h-3.5 w-3.5" /> Deploy Database
						</Button>
					)}
				</div>
			</section>

			{/* Deployments List Section */}
			<DatabaseDeploymentsList
				events={events}
				isBuildActive={isBuildActive}
				getStatusBadge={getStatusBadge}
				onOpenLogs={id => setActiveLogId(id)}
				onCancel={id => setCancelingId(id)}
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
