import {useState, useEffect, useMemo} from 'react';
import {Zap, RefreshCw, Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {$api} from '#/api/query';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import {ComposeDeploymentsList} from './deployments/compose-deployments-list';

interface ComposeDeploymentsTabProps {
	composeId: number;
}

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

export function ComposeDeploymentsTab({composeId}: ComposeDeploymentsTabProps) {
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [liveLogs, setLiveLogs] = useState<string[]>([]);

	// Fetch deployments query with polling
	const {data: rawDeployments = [], isLoading, refetch} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {
				query: {
					compose_id: composeId,
					limit: 50,
				} as any,
			},
		},
		{
			enabled: !!composeId,
			refetchInterval: (query) => {
				const data = query.state.data as any[] | undefined;
				const hasActive = data?.some(e => {
					if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
					const s = (e.status || '').toUpperCase();
					const st = (e.state || '').toUpperCase();
					return !FINAL_STATES.includes(s) && !FINAL_STATES.includes(st);
				});
				return hasActive ? 3000 : false;
			},
		}
	);

	const deployments = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

	const selectedEvent = deployments.find(d => d.id === activeLogId);

	// Real-time log stream reader for active modal
	useEffect(() => {
		if (!activeLogId) return;

		let isMounted = true;
		let controller = new AbortController();

		const initialFallback = selectedEvent?.log_content || selectedEvent?.message || selectedEvent?.description;
		if (initialFallback) {
			setLiveLogs(initialFallback.split('\n'));
		} else {
			setLiveLogs([]);
		}

		const startStream = async () => {
			try {
				const sessionRaw = localStorage.getItem('rustploy-auth-session');
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
	}, [activeLogId, selectedEvent]);

	return (
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Zap className="w-4 h-4 text-primary" /> Deployment History
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Audit log records of all compose build executions and stack deployments</p>
				</div>
				<Button
					variant="outline"
					size="sm"
					onClick={() => refetch()}
					className="h-8 text-xs font-semibold flex items-center gap-1.5"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isLoading ? 'animate-spin' : ''}`} /> Refresh
				</Button>
			</section>

			{/* Deployments List Component (< 200 lines) */}
			<ComposeDeploymentsList
				deployments={deployments}
				isLoading={isLoading}
				onOpenStream={setActiveLogId}
			/>

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
								isDeployment={true}
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
