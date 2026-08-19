import {useState, useEffect, useRef, useMemo} from 'react';
import {RefreshCw, Download, Play, Square, Box} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

interface LogsTabProps {
	app: any;
}

export function LogsTab({app}: LogsTabProps) {
	const [lines, setLines] = useState('500');
	const [timestamps, setTimestamps] = useState(false);
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);
	const [selectedTarget, setSelectedTarget] = useState('');
	const scrollRef = useRef<HTMLDivElement>(null);

	const availableTargets = useMemo(() => {
		const list: string[] = [];
		const baseName = app?.app_name || app?.name;
		if (baseName) list.push(baseName);
		if (app?.containers && Array.isArray(app.containers)) {
			app.containers.forEach((c: any) => {
				const name = c?.name || c?.container_name;
				if (name && !list.includes(name)) list.push(name);
			});
		}
		return list.length > 0 ? list : ['app'];
	}, [app]);

	const activeTarget =
		selectedTarget ||
		availableTargets[0] ||
		app?.app_name ||
		app?.name ||
		'app';

	// Connect to backend Server-Sent Events (SSE) docker service log stream
	useEffect(() => {
		if (!activeTarget) return;

		let isMounted = true;
		const controller = new AbortController();
		setIsLoading(true);
		setStreamedLogs([]);

		const startStream = async () => {
			try {
				let accessToken = '';
				const sessionRaw = localStorage.getItem('openoxide-auth-session');
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const params = new URLSearchParams({
					tail: lines,
					timestamps: String(timestamps),
					follow: String(isLive),
				});
				if (application?.server_id) {
					params.append('server_id', String(application.server_id));
				}

				const response = await fetch(
					`/api/deployments/docker/service/${encodeURIComponent(activeTarget)}/logs?${params.toString()}`,
					{
						headers: {
							Authorization: accessToken ? `Bearer ${accessToken}` : '',
						},
						signal: controller.signal,
					},
				);

				if (!response.ok) {
					if (isMounted) {
						setStreamedLogs([
							`Log stream notice: Container service '${activeTarget}' is not active or has no logs.`,
						]);
						setIsLoading(false);
					}
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				if (isMounted) setIsLoading(false);

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';

					for (const rawLine of rawLines) {
						const trimmed = rawLine.trim();
						if (
							!trimmed ||
							trimmed.startsWith('event:') ||
							trimmed.includes('event: log') ||
							trimmed.includes('event: deployment') ||
							trimmed.startsWith('id:') ||
							trimmed.startsWith(':') ||
							trimmed.includes('keep-alive')
						) {
							continue;
						}

						let textToPush = '';
						if (trimmed.startsWith('data:')) {
							const jsonStr = trimmed.slice(5).trim();
							if (!jsonStr || jsonStr.includes('keep-alive')) continue;
							try {
								const data = JSON.parse(jsonStr);
								if (data.type === 'keep-alive') continue;
								textToPush =
									data.line !== undefined
										? data.line
										: data.data || data.message || jsonStr;
							} catch {
								textToPush = jsonStr;
							}
						} else {
							textToPush = trimmed;
						}

						if (textToPush !== undefined) {
							let cleaned = textToPush.replace(/\r?\n$/, '').trim();
							if (
								!cleaned ||
								cleaned.startsWith('event:') ||
								cleaned.includes('event: log') ||
								cleaned.includes('event: deployment') ||
								cleaned.includes('keep-alive')
							) {
								continue;
							}
							if (cleaned.startsWith('logs/')) cleaned = cleaned.slice(5);
							else if (cleaned.startsWith('log/'))
								cleaned = cleaned.slice(4);

							if (isMounted) setStreamedLogs(prev => [...prev, cleaned]);
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setStreamedLogs(prev => [
						...prev,
						`Container log stream ended.`,
					]);
				}
			} finally {
				if (isMounted) setIsLoading(false);
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [activeTarget, lines, timestamps, isLive, refetchTrigger]);

	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [streamedLogs, isLive]);

	const handleDownload = () => {
		const blob = new Blob([streamedLogs.join('\n')], {type: 'text/plain'});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `${activeTarget || 'logs'}.txt`;
		a.click();
		URL.revokeObjectURL(url);
	};

	return (
		<div className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-sm">
			<div className="flex flex-wrap items-center justify-between gap-4">
				<div className="flex flex-wrap items-center gap-3">
					<div>
						<h3 className="text-sm font-bold text-foreground">
							Container Logs
						</h3>
						<p className="mt-1 text-xs text-muted-foreground">
							Real-time terminal output stream from the running container
						</p>
					</div>

					{availableTargets.length > 0 && (
						<Select
							value={activeTarget}
							onValueChange={val => {
								if (val) setSelectedTarget(val);
							}}>
							<SelectTrigger className="ml-2 h-9 min-w-[170px] border-border/80 bg-muted/30 font-mono text-xs font-bold shadow-2xs hover:bg-muted/60">
								<Box className="mr-1 size-3.5 shrink-0 text-primary" />
								<SelectValue placeholder="Select Container" />
							</SelectTrigger>
							<SelectContent className="border-border bg-card">
								<div className="mb-1 border-b border-border/40 px-3 py-1.5 text-[9px] font-bold tracking-wider text-muted-foreground uppercase">
									App Containers
								</div>
								{availableTargets.map(target => (
									<SelectItem
										key={target}
										value={target}
										className="font-mono text-xs font-semibold">
										Target: {target}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}
				</div>

				<div className="flex flex-wrap items-center gap-3">
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="flex h-8 items-center gap-1.5 text-xs font-semibold">
						{isLive ? (
							<Square className="h-3 w-3 fill-current" />
						) : (
							<Play className="h-3 w-3 fill-current" />
						)}
						{isLive ? 'Live Stream' : 'Paused'}
					</Button>

					<label className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-muted-foreground">
						<input
							type="checkbox"
							checked={timestamps}
							onChange={e => setTimestamps(e.target.checked)}
							className="h-4 w-4 rounded accent-primary"
						/>
						Timestamps
					</label>

					<Select
						value={lines}
						onValueChange={val => val && setLines(val)}>
						<SelectTrigger className="h-8 w-[110px] border border-border/60 bg-muted/30 text-xs font-semibold">
							<SelectValue placeholder="Lines" />
						</SelectTrigger>
						<SelectContent className="border-border bg-card">
							<SelectItem value="50" className="text-xs">
								50 lines
							</SelectItem>
							<SelectItem value="100" className="text-xs">
								100 lines
							</SelectItem>
							<SelectItem value="200" className="text-xs">
								200 lines
							</SelectItem>
							<SelectItem value="500" className="text-xs">
								500 lines
							</SelectItem>
							<SelectItem value="1000" className="text-xs">
								1000 lines
							</SelectItem>
						</SelectContent>
					</Select>

					<Button
						variant="outline"
						size="sm"
						onClick={() => setRefetchTrigger(prev => prev + 1)}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw className="h-3.5 w-3.5" /> Refresh
					</Button>

					{streamedLogs.length > 0 && (
						<Button
							variant="outline"
							size="sm"
							onClick={handleDownload}
							className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
							<Download className="h-3.5 w-3.5" /> Download
						</Button>
					)}
				</div>
			</div>

			<DeploymentViewer
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={isLive}
				loadingText={`Connecting to real-time container log stream for '${activeTarget}'...`}
				emptyText={`No container log entries found for '${activeTarget}'. The application container may be stopped or initializing.`}
				onDownload={handleDownload}
				onReload={() => setRefetchTrigger(prev => prev + 1)}
			/>
		</div>
	);
}
