import {useState, useEffect, useRef} from 'react';
import {Zap, RefreshCw, Clock, XCircle, Terminal, X, Activity, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useAppStore} from '#/stores/app-store';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';

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
	database,
	deployments: passedDeployments,
	onRefresh,
	onAction,
}: DatabaseDeploymentsTabProps) {
	const queryClient = useQueryClient();
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const [cancelingId, setCancelingId] = useState<number | null>(null);

	// RAM Store Sync
	const storeDeployments = useAppStore((state) => state.deployments || []);
	const dbDeployments = storeDeployments.filter((d: any) => String(d.database_id) === String(dbId));
	const events = dbDeployments.length > 0 ? dbDeployments : (passedDeployments ?? []);

	const activeDeployment = events.find(isBuildActive);

	// Mutations
	const deleteMutation = $api.useMutation('delete', '/deployments/{id}');
	const clearDbMutation = $api.useMutation('delete', '/deployments/database/{id}');

	// Realtime SSE log stream listener
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
				if (accessToken) {
					headers['Authorization'] = `Bearer ${accessToken}`;
				}

				const response = await fetch(`/api/deployments/${activeLogId}/logs`, {
					headers,
					signal: controller.signal,
				});

				if (!response.ok) {
					throw new Error(`HTTP error! status: ${response.status}`);
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				let buffer = '';

				if (!reader) return;

				while (isMounted) {
					const {value, done} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						if (line.startsWith('data: ')) {
							const dataStr = line.slice(6).trim();
							if (dataStr) {
								try {
									const dataObj = JSON.parse(dataStr);
									const logText = typeof dataObj === 'string' ? dataObj : dataObj.message || dataObj.log || JSON.stringify(dataObj);
									setLiveLogs((prev) => [...prev, logText]);
								} catch {
									setLiveLogs((prev) => [...prev, dataStr]);
								}
							}
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setLiveLogs((prev) => [...prev, `[System] Stream error: ${err.message || 'Disconnected'}`]);
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [activeLogId]);

	const getStatusBadge = (e: any) => {
		const s = (e.status || e.state || '').toUpperCase();
		if (s === 'DONE' || s === 'HEALTHY' || s === 'SUCCESS' || s === 'DEPLOYED')
			return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s === 'ERROR' || s === 'FAILED' || s === 'CRASHED')
			return 'text-rose-500 bg-rose-500/10 border-rose-500/20';
		if (s === 'CANCELLED') return 'text-zinc-400 bg-zinc-500/10 border-zinc-500/20';
		if (isBuildActive(e)) return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		return 'text-muted-foreground bg-muted border-border';
	};

	const handleClearDatabaseDeployments = async () => {
		try {
			const res = await clearDbMutation.mutateAsync({
				params: {
					path: {
						id: dbId,
					},
				},
			});
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
			await deleteMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
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
					<Button
						variant="outline"
						size="sm"
						onClick={handleClearDatabaseDeployments}
						className="border-destructive/30 text-destructive bg-destructive/10 hover:bg-destructive/20 font-semibold flex items-center gap-1.5 h-8 text-xs"
					>
						<Trash2 className="w-3.5 h-3.5" /> Clear History
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onRefresh?.()}
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs"
					>
						<RefreshCw className="w-3.5 h-3.5" /> Refresh
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onAction?.('redeploy')}
						disabled={!!activeDeployment}
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs disabled:opacity-50"
					>
						<RefreshCw className="w-3.5 h-3.5" /> Redeploy Container
					</Button>
					{activeDeployment ? (
						<Button
							onClick={() => activeDeployment.id && setCancelingId(activeDeployment.id)}
							size="sm"
							variant="destructive"
							className="font-semibold flex items-center gap-1.5 h-8 text-xs"
						>
							<XCircle className="w-3.5 h-3.5" /> Cancel Build
						</Button>
					) : (
						<Button
							onClick={() => onAction?.('deploy')}
							size="sm"
							className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs"
						>
							<Zap className="w-3.5 h-3.5" /> Deploy Database
						</Button>
					)}
				</div>
			</section>

			{/* Deployments List Section */}
			<section className="bg-card border border-border rounded-xl overflow-hidden">
				{events.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
						<Zap className="w-10 h-10 opacity-30 mb-3" />
						<p className="text-xs font-semibold">No database deployment history recorded yet</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{events.map((e: any) => {
							const isActive = isBuildActive(e);
							return (
								<div key={e.id} className="flex items-center justify-between p-4 hover:bg-muted/10 transition-colors">
									<div className="min-w-0 flex flex-col gap-0.5">
										<span className="text-xs font-semibold text-foreground truncate">{e.title || `Database Deployment #${e.id}`}</span>
										{e.description && <span className="text-[11px] text-muted-foreground truncate">{e.description}</span>}
									</div>

									<div className="flex items-center gap-3 shrink-0">
										<span className={`text-[10px] font-bold px-2 py-0.5 rounded border uppercase tracking-wider ${getStatusBadge(e)}`}>
											{e.status || 'DEPLOYED'}
										</span>
										<span className="text-[10px] text-muted-foreground flex items-center gap-1 font-medium">
											<Clock className="w-3 h-3" />
											{e.created_at ? new Date(e.created_at * 1000).toLocaleDateString() : 'Just now'}
										</span>

										<Button
											size="sm"
											variant="outline"
											onClick={() => setActiveLogId(e.id)}
											className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1"
										>
											<Terminal className="w-3 h-3" /> Stream Logs
										</Button>

										{isActive && (
											<Button
												size="sm"
												variant="ghost"
												onClick={() => setCancelingId(e.id)}
												className="h-7 text-xs text-destructive hover:text-destructive hover:bg-destructive/10 px-2 rounded-lg font-semibold flex items-center gap-1"
											>
												<XCircle className="w-3 h-3" /> Cancel
											</Button>
										)}

										{!isActive && e.id !== undefined && (
											<Button
												size="sm"
												variant="ghost"
												onClick={() => handleDeleteSingleDbDeployment(e.id)}
												className="h-7 text-xs text-muted-foreground hover:text-destructive hover:bg-destructive/10 px-1.5 rounded-lg font-semibold flex items-center gap-1"
											>
												<Trash2 className="w-3.5 h-3.5" />
											</Button>
										)}
									</div>
								</div>
							);
						})}
					</div>
				)}
			</section>

			{/* Cancel Confirmation Dialog */}
			<AlertDialog open={cancelingId !== null} onOpenChange={(open) => !open && setCancelingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Database Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this database container deployment? This action will stop the running container setup.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setCancelingId(null)}>Keep Running</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={confirmCancel}
						>
							Yes, Cancel Deployment
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>

			{/* Realtime Stream Logs Modal */}
			{activeLogId && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-4xl flex flex-col overflow-hidden max-h-[85vh]">
						<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
							<div className="flex items-center gap-2">
								<Terminal className="w-4 h-4 text-foreground" />
								<h3 className="text-xs font-bold text-foreground">Live Database Container Deployment Stream #{activeLogId}</h3>
							</div>
							<Button variant="ghost" size="sm" onClick={() => setActiveLogId(null)} className="h-7 w-7 p-0 rounded-lg hover:bg-muted text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<div className="p-4 overflow-y-auto">
							<DeploymentViewer
								logs={liveLogs}
								isLoading={liveLogs.length === 0}
								isLive={true}
								loadingText="Connecting to realtime SSE database deployment stream..."
								emptyText="No container deployment build log entries available."
							/>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
