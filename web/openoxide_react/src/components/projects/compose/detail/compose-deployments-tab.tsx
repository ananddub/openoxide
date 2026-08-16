import {useState, useEffect} from 'react';
import {Zap, RefreshCw, Terminal, X, XCircle, Activity} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import { useAppStore } from '#/stores/app-store';
import {ComposeDeploymentsList} from './deployments/compose-deployments-list';
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

interface ComposeDeploymentsTabProps {
	composeId: number;
	deployments?: any[];
	isLoading?: boolean;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = ['RUNNING', 'BUILDING', 'PREPARING', 'QUEUE', 'QUEUED', 'STARTING', 'DEPLOYING', 'PENDING', 'GIT', 'DOCKER'];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
};

export function ComposeDeploymentsTab({composeId, deployments: passedDeployments, isLoading: passedIsLoading}: ComposeDeploymentsTabProps) {
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const [isTriggering, setIsTriggering] = useState(false);

	// Read deployments directly from Zustand RAM Store
	const storeDeployments = useAppStore((state) => state.deployments || []);
	const rawDeployments = storeDeployments.filter((d: any) => String(d.compose_id) === String(composeId));
	const deployments = rawDeployments.length > 0 ? rawDeployments : (passedDeployments ?? []);
	const isLoading = false;
	const activeDeployment = deployments.find(isBuildActive);
	const selectedEvent = deployments.find(d => d.id === activeLogId);

	// Initial state for activeLogId is null so modal never opens automatically on tab load

	// Mutations
	const deployMutation = $api.useMutation('post', '/compose/{id}/deploy') as any;
	const redeployMutation = $api.useMutation('post', '/compose/{id}/redeploy') as any;
	const cancelMutation = $api.useMutation('post', '/compose/{id}/cancel') as any;
	const deleteMutation = $api.useMutation('delete', '/deployments/{id}') as any;
	const clearComposeMutation = $api.useMutation('delete', '/deployments/compose/{id}') as any;

	const handleDeploy = async () => {
		setIsTriggering(true);
		try {
			await deployMutation.mutateAsync({params: {path: {id: composeId}}});
			toast.success('Compose deployment triggered successfully');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleRedeploy = async () => {
		setIsTriggering(true);
		try {
			await redeployMutation.mutateAsync({params: {path: {id: composeId}}});
			toast.success('Compose redeploy triggered successfully');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const [isConfirmCancelOpen, setIsConfirmCancelOpen] = useState(false);

	const handleCancel = async () => {
		setIsTriggering(true);
		try {
			await (cancelMutation as any).mutateAsync({params: {path: {id: composeId}}});
			toast.success('Deployment cancellation requested');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
			setIsConfirmCancelOpen(false);
		}
	};

	// Real-time log stream reader for active modal
	useEffect(() => {
		if (!activeLogId) return;

		let isMounted = true;
		let controller = new AbortController();

		const initialFallback = (selectedEvent as any)?.log_content || (selectedEvent as any)?.message || selectedEvent?.description;
		if (initialFallback) {
			setLiveLogs(initialFallback.split('\n'));
		} else {
			setLiveLogs([]);
		}

		const startStream = async () => {
			try {
				const sessionRaw = localStorage.getItem('openoxide-auth-session');
				let accessToken = '';
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const response = await fetch(`/api/deployments/${activeLogId}/logs`, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				if (!response.ok) {
					if (isMounted && initialFallback) {
						setLiveLogs(initialFallback.split('\n'));
					}
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				let buffer = '';

				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) {
						if (buffer.trim()) {
							const finalLines = extractLogLines(buffer);
							if (finalLines.length > 0 && isMounted) {
								setLiveLogs(prev => [...prev, ...finalLines]);
							}
						}
						break;
					}

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						const extracted = extractLogLines(line);
						if (extracted.length > 0 && isMounted) {
							setLiveLogs(prev => [...prev, ...extracted]);
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted && initialFallback) {
					setLiveLogs(initialFallback.split('\n'));
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	const queryClient = useQueryClient();

	const handleClearComposeDeployments = async () => {
		try {
			const res = await clearComposeMutation.mutateAsync({
				params: {
					path: {
						id: composeId,
					},
				},
			});
			const data = res as any;
			useAppStore.getState().clearDeployments({composeId});
			toast.success(`Cleared ${data?.cleared_count || 0} compose deployment logs & history`);
			queryClient.invalidateQueries({ queryKey: ['get', '/deployments'] });
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteSingleComposeDeployment = async (id: number) => {
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
			queryClient.invalidateQueries({ queryKey: ['get', '/deployments'] });
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const isActionPending = isTriggering || deployMutation.isPending || redeployMutation.isPending || cancelMutation.isPending;

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						Deployments History
						{(activeDeployment || isActionPending) && (
							<span className="flex items-center gap-1 text-[10px] font-semibold text-amber-500 bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 rounded-full animate-pulse">
								<Activity className="w-3 h-3 animate-spin" /> {isActionPending ? 'Triggering...' : 'Active Build'}
							</span>
						)}
					</h3>
					<p className="text-xs text-muted-foreground mt-0.5">Audit log of building, deployment operations and lifecycle events</p>
				</div>

				<div className="flex items-center gap-2">
					<Button variant="outline" size="sm" onClick={handleClearComposeDeployments} disabled={isActionPending} className="border-destructive/30 text-destructive bg-destructive/10 hover:bg-destructive/20 font-semibold flex items-center gap-1.5 h-8 text-xs">
						Clear History
					</Button>
					<Button variant="outline" size="sm" disabled={isActionPending} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs">
						<RefreshCw className={`w-3.5 h-3.5 ${isActionPending ? 'animate-spin' : ''}`} /> Refresh
					</Button>
					<Button variant="outline" size="sm" onClick={handleRedeploy} disabled={!!activeDeployment || isActionPending} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs disabled:opacity-50">
						<RefreshCw className={`w-3.5 h-3.5 ${isActionPending ? 'animate-spin' : ''}`} /> Redeploy
					</Button>
					{isActionPending ? (
						<Button disabled size="sm" className="bg-primary/80 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
							<RefreshCw className="w-3.5 h-3.5 animate-spin" /> Processing...
						</Button>
					) : activeDeployment ? (
						<Button onClick={() => setIsConfirmCancelOpen(true)} size="sm" variant="destructive" className="font-semibold flex items-center gap-1.5 h-8 text-xs">
							<XCircle className="w-3.5 h-3.5" /> Cancel Build
						</Button>
					) : (
						<Button onClick={handleDeploy} size="sm" className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
							<Zap className="w-3.5 h-3.5" /> Deploy
						</Button>
					)}
				</div>
			</section>

			{/* Deployments List Component */}
			<ComposeDeploymentsList
				deployments={deployments}
				isLoading={isLoading}
				onOpenStream={setActiveLogId}
				onCancelBuild={() => setIsConfirmCancelOpen(true)}
				onDeleteDeployment={handleDeleteSingleComposeDeployment}
			/>

			{/* Cancel Confirmation Dialog */}
			<AlertDialog open={isConfirmCancelOpen} onOpenChange={setIsConfirmCancelOpen}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Compose Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this compose deployment? This action will stop the running build process.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setIsConfirmCancelOpen(false)}>No, Keep Running</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={handleCancel}
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
								<h3 className="text-xs font-bold text-foreground">Live Deployment Build Stream #{activeLogId}</h3>
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
								loadingText="Connecting to realtime SSE deployment stream..."
								emptyText="No deployment build log entries available."
							/>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
