import {useApplicationDeploy, useApplicationRedeploy, useDeploymentCancel, useDeploymentDelete} from '#/api/live';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';

import {useState, useEffect, useRef} from 'react';
import {
	Zap,
	RefreshCw,
	Clock,
	XCircle,
	Terminal,
	X,
	Activity,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {formatApiError} from '#/api/utils';
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

const deploymentClient = createServiceClient(DeploymentService);

interface DeploymentsTabProps {
	appId: number;
	deployments?: any[];
	onRefresh?: () => void;
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
		'GIT',
		'DOCKER',
	];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
};

export function DeploymentsTab({
	appId,
	deployments: passedDeployments,
	onRefresh,
}: DeploymentsTabProps) {
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const logContainerRef = useRef<HTMLDivElement>(null);

	const events = Array.isArray(passedDeployments) ? passedDeployments : [];
	const activeDeployment = events.find(isBuildActive);

	// Mutations
	const deployMutation = useApplicationDeploy();
	const redeployMutation = useApplicationRedeploy();
	const cancelMutation = useDeploymentCancel();
	const deleteMutation = useDeploymentDelete();
	const clearAppMutation = useDeploymentDelete();

	// ConnectRPC deployment log stream
	useEffect(() => {
		if (!activeLogId) return;
		setLiveLogs([]);
		let isMounted = true;
		const controller = new AbortController();

		const startStream = async () => {
			try {
				const call = deploymentClient.streamDeploymentLogs(
					{id: BigInt(activeLogId)},
					{signal: controller.signal},
				);
				for await (const event of call) {
					if (!isMounted) return;
					const lines = extractLogLines(event.payload);
					if (lines.length > 0) setLiveLogs(prev => [...prev, ...lines]);
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setLiveLogs(prev => [
						...prev,
						`Stream error: ${err.message || 'Connection lost'}`,
					]);
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [activeLogId]);

	useEffect(() => {
		if (logContainerRef.current) {
			logContainerRef.current.scrollTop =
				logContainerRef.current.scrollHeight;
		}
	}, [liveLogs]);

	const [isTriggering, setIsTriggering] = useState(false);

	const handleDeploy = async () => {
		setIsTriggering(true);
		try {
			await deployMutation.mutateAsync(appId);
			toast.success('Deployment triggered successfully');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleRedeploy = async () => {
		setIsTriggering(true);
		try {
			await redeployMutation.mutateAsync(appId);
			toast.success('Redeploy triggered successfully');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const [cancelingId, setCancelingId] = useState<number | null>(null);

	const confirmCancel = async () => {
		if (!cancelingId) return;
		setIsTriggering(true);
		try {
			await cancelMutation.mutateAsync(cancelingId);
			toast.success('Deployment cancellation requested');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
			setCancelingId(null);
		}
	};

	const getStatusBadge = (e: any) => {
		const s = (e.status || '').toUpperCase();
		if (
			s === 'DONE' ||
			s === 'HEALTHY' ||
			s === 'SUCCESS' ||
			s === 'DEPLOYED'
		)
			return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s === 'ERROR' || s === 'FAILED' || s === 'CRASHED')
			return 'text-rose-500 bg-rose-500/10 border-rose-500/20';
		if (s === 'CANCELLING')
			return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		if (s === 'CANCELLED')
			return 'text-zinc-400 bg-zinc-500/10 border-zinc-500/20';
		if (isBuildActive(e))
			return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		return 'text-muted-foreground bg-muted ';
	};

	const queryClient = useQueryClient();

	const handleClearAppDeployments = async () => {
		try {
			const res = await clearAppMutation.mutateAsync(appId);
			const data = res as any;
			toast.success(
				`Cleared ${data?.cleared_count || 0} deployment logs & history`,
			);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteSingleAppDeployment = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success(`Deployment #${id} deleted`);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const isActionPending =
		isTriggering ||
		deployMutation.isPending ||
		redeployMutation.isPending ||
		cancelMutation.isPending;

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header */}
			<section className="flex items-center justify-between rounded-xl bg-card p-5">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						Deployments History
						{(activeDeployment || isActionPending) && (
							<span className="flex animate-pulse items-center gap-1 rounded-full bg-amber-500/10 px-2 py-0.5 text-[10px] font-semibold text-amber-500">
								<Activity className="h-3 w-3 animate-spin" />{' '}
								{isActionPending ? 'Triggering...' : 'Active Build'}
							</span>
						)}
					</h3>
					<p className="mt-0.5 text-xs text-muted-foreground">
						Audit log of building, deployment operations and lifecycle
						events
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={handleClearAppDeployments}
						disabled={isActionPending}
						className="flex h-8 items-center gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
						<Trash2 className="h-3.5 w-3.5" /> Clear History
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onRefresh?.()}
						disabled={isActionPending}
						className="flex h-8 items-center gap-1.5  text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw
							className={`h-3.5 w-3.5 ${isActionPending ? 'animate-spin' : ''}`}
						/>{' '}
						Refresh
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={handleRedeploy}
						disabled={!!activeDeployment || isActionPending}
						className="flex h-8 items-center gap-1.5  text-xs font-semibold text-foreground hover:bg-muted disabled:opacity-50">
						<RefreshCw
							className={`h-3.5 w-3.5 ${isActionPending ? 'animate-spin' : ''}`}
						/>{' '}
						Redeploy
					</Button>
					{isActionPending ? (
						<Button
							disabled
							size="sm"
							className="flex h-8 items-center gap-1.5 bg-primary/80 text-xs font-semibold text-primary-foreground">
							<RefreshCw className="h-3.5 w-3.5 animate-spin" />{' '}
							Processing...
						</Button>
					) : activeDeployment ? (
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
							onClick={handleDeploy}
							size="sm"
							className="flex h-8 items-center gap-1.5 bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/90">
							<Zap className="h-3.5 w-3.5" /> Deploy
						</Button>
					)}
				</div>
			</section>

			{/* List Section */}
			<section className="overflow-hidden rounded-xl bg-card">
				{events.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
						<Zap className="mb-3 h-10 w-10 opacity-30" />
						<p className="text-xs font-semibold">
							No deployments registered yet
						</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{events.map((e: any) => {
							const isActive = isBuildActive(e);
							return (
								<div
									key={e.id}
									className="flex items-center justify-between p-4 transition-colors hover:bg-muted/10">
									<div className="flex min-w-0 flex-col gap-0.5">
										<span className="truncate text-xs font-semibold text-foreground">
											{e.title}
										</span>
										{e.description && (
											<span className="truncate text-[11px] text-muted-foreground">
												{e.description}
											</span>
										)}
									</div>

									<div className="flex shrink-0 items-center gap-3">
										<span
											className={`rounded  px-2 py-0.5 text-[10px] font-bold tracking-wider uppercase ${getStatusBadge(e)}`}>
											{e.status}
										</span>
										<span className="flex items-center gap-1 text-[10px] font-medium text-muted-foreground">
											<Clock className="h-3 w-3" />
											{new Date(Number(e.created_at) * 1000).toLocaleDateString()}
										</span>

										<Button
											size="sm"
											variant="outline"
											onClick={() => setActiveLogId(e.id)}
											className="flex h-7 items-center gap-1 rounded-lg  px-2 text-xs font-semibold text-foreground hover:bg-muted">
											<Terminal className="h-3 w-3" /> Stream Logs
										</Button>

										{isActive && (
											<Button
												size="sm"
												variant="ghost"
												onClick={() => setCancelingId(e.id)}
												className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
												<XCircle className="h-3 w-3" /> Cancel
											</Button>
										)}

										{!isActive && e.id !== undefined && (
											<Button
												size="sm"
												variant="ghost"
												onClick={() =>
													handleDeleteSingleAppDeployment(e.id)
												}
												className="flex h-7 items-center gap-1 rounded-lg px-1.5 text-xs font-semibold text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
												<Trash2 className="h-3.5 w-3.5" />
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
			<AlertDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this deployment? This action
							will stop the running build process.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setCancelingId(null)}>
							Keep Running
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
							onClick={confirmCancel}>
							Yes, Cancel Deployment
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>

			{/* Realtime Stream Logs Modal — Rich Dokploy-Grade LogViewer! */}
			{activeLogId && (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
					<div className="flex max-h-[85vh] w-full max-w-4xl flex-col overflow-hidden rounded-xl bg-card shadow-2xl">
						<div className="flex items-center justify-between border-b border- bg-muted/30 p-4">
							<div className="flex items-center gap-2">
								<Terminal className="h-4 w-4 text-foreground" />
								<h3 className="text-xs font-bold text-foreground">
									Live Deployment Build Stream #{activeLogId}
								</h3>
							</div>
							<Button
								variant="ghost"
								size="sm"
								onClick={() => setActiveLogId(null)}
								className="h-7 w-7 rounded-lg p-0 text-muted-foreground hover:bg-muted">
								<X className="h-4 w-4" />
							</Button>
						</div>
						<div className="overflow-y-auto p-4">
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
