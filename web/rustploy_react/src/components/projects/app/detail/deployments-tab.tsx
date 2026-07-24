import {useState, useEffect, useRef} from 'react';
import {Zap, RefreshCw, Clock, XCircle, Terminal, X, Activity} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface DeploymentsTabProps {
	appId: number;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

const isBuildActive = (e: any) => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	const activeKeywords = ['BUILDING', 'PREPARING', 'QUEUE', 'QUEUED', 'STARTING', 'DEPLOYING', 'PENDING', 'GIT', 'DOCKER'];
	return activeKeywords.some(kw => s.includes(kw) || st.includes(kw));
};

export function DeploymentsTab({appId}: DeploymentsTabProps) {
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);
	const logContainerRef = useRef<HTMLDivElement>(null);

	// Fetch Application deployment events with auto refetch when active
	const {data: events = [], isLoading, refetch} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {
				query: {
					application_id: appId,
					limit: 100,
				} as any,
			},
		},
		{
			refetchInterval: (query) => {
				const data = query.state.data as any[] | undefined;
				const hasActive = data?.some(isBuildActive);
				return hasActive ? 1000 : 3000;
			},
		}
	);

	const activeDeployment = events.find(isBuildActive);

	// Mutations
	const deployMutation = $api.useMutation('post', '/applications/{id}/deploy');
	const redeployMutation = $api.useMutation('post', '/applications/{id}/redeploy');
	const cancelMutation = $api.useMutation('post', '/deployments/{id}/cancel');

	// Realtime SSE log stream listener with JWT Authorization header
	useEffect(() => {
		if (!activeLogId) return;
		setLiveLogs([]);
		let isMounted = true;
		const controller = new AbortController();

		const startStream = async () => {
			try {
				let accessToken = '';
				const sessionRaw = localStorage.getItem('rustploy-auth-session');
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
					if (isMounted) setLiveLogs([`Failed to stream logs: ${response.statusText} (${response.status})`]);
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						const trimmed = line.trim();
						if (!trimmed || trimmed.startsWith('event:') || trimmed.startsWith(':')) continue;

						if (line.startsWith('data:')) {
							try {
								const jsonStr = line.slice(5).trim();
								if (jsonStr) {
									const data = JSON.parse(jsonStr);
									if (data.line) {
										if (isMounted) setLiveLogs(prev => [...prev, data.line]);
									} else if (data.message) {
										if (isMounted) setLiveLogs(prev => [...prev, data.message]);
									}
								}
							} catch {
								const rawContent = line.slice(5).trim();
								if (rawContent && isMounted) setLiveLogs(prev => [...prev, rawContent]);
							}
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setLiveLogs(prev => [...prev, `Stream error: ${err.message || 'Connection lost'}`]);
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
			logContainerRef.current.scrollTop = logContainerRef.current.scrollHeight;
		}
	}, [liveLogs]);

	const [isTriggering, setIsTriggering] = useState(false);

	const handleDeploy = async () => {
		setIsTriggering(true);
		try {
			await deployMutation.mutateAsync({params: {path: {id: appId}}});
			toast.success('Deployment triggered successfully');
			await refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleRedeploy = async () => {
		setIsTriggering(true);
		try {
			await redeployMutation.mutateAsync({params: {path: {id: appId}}});
			toast.success('Redeploy triggered successfully');
			await refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleCancel = async (id: number) => {
		if (!confirm('Are you sure you want to cancel this deployment?')) return;
		setIsTriggering(true);
		try {
			await cancelMutation.mutateAsync({params: {path: {id}}});
			toast.success('Deployment cancellation requested');
			await refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const getStatusBadge = (e: any) => {
		const s = (e.status || '').toUpperCase();
		if (s === 'DONE' || s === 'HEALTHY' || s === 'SUCCESS' || s === 'DEPLOYED') 
			return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
		if (s === 'ERROR' || s === 'FAILED' || s === 'CRASHED') 
			return 'text-rose-500 bg-rose-500/10 border-rose-500/20';
		if (isBuildActive(e)) 
			return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
		return 'text-muted-foreground bg-muted border-border';
	};

	const renderLogLine = (line: string, i: number) => {
		const l = line.toLowerCase();
		let colorClass = 'text-zinc-300';
		if (l.includes('error') || l.includes('failed') || l.includes('err!')) {
			colorClass = 'text-rose-400 font-semibold';
		} else if (l.includes('warn') || l.includes('warning')) {
			colorClass = 'text-amber-400';
		} else if (l.includes('success') || l.includes('done') || l.includes('finished')) {
			colorClass = 'text-emerald-400';
		}
		return (
			<div key={i} className={`whitespace-pre-wrap break-all leading-relaxed ${colorClass}`}>
				{line}
			</div>
		);
	};

	const isActionPending = isTriggering || deployMutation.isPending || redeployMutation.isPending || cancelMutation.isPending;

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header */}
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
					<Button variant="outline" size="sm" onClick={() => refetch()} disabled={isActionPending} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs">
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
						<Button onClick={() => activeDeployment.id && handleCancel(activeDeployment.id)} size="sm" variant="destructive" className="font-semibold flex items-center gap-1.5 h-8 text-xs">
							<XCircle className="w-3.5 h-3.5" /> Cancel Build
						</Button>
					) : (
						<Button onClick={handleDeploy} size="sm" className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
							<Zap className="w-3.5 h-3.5" /> Deploy
						</Button>
					)}
				</div>
			</section>

			{/* List Section */}
			<section className="bg-card border border-border rounded-xl overflow-hidden">
				{isLoading ? (
					<div className="flex justify-center py-12"><RefreshCw className="w-6 h-6 animate-spin text-muted-foreground/45" /></div>
				) : events.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
						<Zap className="w-10 h-10 opacity-30 mb-3" />
						<p className="text-xs font-semibold">No deployments registered yet</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{events.map((e: any) => {
							const isActive = isBuildActive(e);
							return (
								<div key={e.id} className="flex items-center justify-between p-4 hover:bg-muted/10 transition-colors">
									<div className="min-w-0 flex flex-col gap-0.5">
										<span className="text-xs font-semibold text-foreground truncate">{e.title}</span>
										{e.description && <span className="text-[11px] text-muted-foreground truncate">{e.description}</span>}
									</div>

									<div className="flex items-center gap-3 shrink-0">
										<span className={`text-[10px] font-bold px-2 py-0.5 rounded border uppercase tracking-wider ${getStatusBadge(e)}`}>
											{e.status}
										</span>
										<span className="text-[10px] text-muted-foreground flex items-center gap-1 font-medium">
											<Clock className="w-3 h-3" />
											{new Date(e.created_at * 1000).toLocaleDateString()}
										</span>

										<Button size="sm" variant="outline" onClick={() => setActiveLogId(e.id)} className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1">
											<Terminal className="w-3 h-3" /> Stream Logs
										</Button>

										{isActive && (
											<Button size="sm" variant="ghost" onClick={() => handleCancel(e.id)} className="h-7 text-xs text-destructive hover:text-destructive hover:bg-destructive/10 px-2 rounded-lg font-semibold flex items-center gap-1">
												<XCircle className="w-3 h-3" /> Cancel
											</Button>
										)}
									</div>
								</div>
							);
						})}
					</div>
				)}
			</section>

			{/* Realtime Stream Logs Modal */}
			{activeLogId && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-3xl flex flex-col overflow-hidden max-h-[80vh]">
						<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
							<div className="flex items-center gap-2">
								<Terminal className="w-4 h-4 text-foreground" />
								<h3 className="text-xs font-bold text-foreground">Live Deployment Build Stream #{activeLogId}</h3>
							</div>
							<Button variant="ghost" size="sm" onClick={() => setActiveLogId(null)} className="h-7 w-7 p-0 rounded-lg hover:bg-muted text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<div ref={logContainerRef} className="p-4 bg-zinc-950 font-mono text-[11px] h-96 overflow-y-auto flex flex-col gap-1">
							{liveLogs.length === 0 ? (
								<div className="flex items-center justify-center h-full text-zinc-500">
									<RefreshCw className="w-4 h-4 animate-spin mr-2" /> Connecting to realtime SSE log stream...
								</div>
							) : (
								liveLogs.map((l, i) => renderLogLine(l, i))
							)}
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
