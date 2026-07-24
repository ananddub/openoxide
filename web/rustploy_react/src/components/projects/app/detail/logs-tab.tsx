import {useState, useEffect, useRef} from 'react';
import {FileText, RefreshCw, Download, Play, Square} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';

interface LogsTabProps {
	app: any;
}

export function LogsTab({app}: LogsTabProps) {
	const [lines, setLines] = useState('100');
	const [timestamps, setTimestamps] = useState(false);
	const [isLive, setIsLive] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const scrollRef = useRef<HTMLDivElement>(null);

	const {data: logsRaw = '', isLoading, refetch} = $api.useQuery(
		'get',
		'/deployments/docker/service/{target}/logs',
		{
			params: {
				path: {target: app.app_name},
				query: {
					tail: parseInt(lines),
					timestamps,
				} as any,
			},
		},
		{
			enabled: !!app.app_name,
			refetchInterval: isLive ? 3000 : false,
		}
	);

	useEffect(() => {
		const raw = logsRaw as unknown as string;
		if (typeof raw === 'string' && raw) {
			const parsed = raw.split('\n').filter(Boolean);
			setStreamedLogs(parsed);
		}
	}, [logsRaw]);

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
		a.download = `${app.app_name || 'logs'}.txt`;
		a.click();
		URL.revokeObjectURL(url);
	};

	const renderLogLine = (line: string, i: number) => {
		const l = line.toLowerCase();
		let colorClass = 'text-zinc-300';
		if (l.includes('error') || l.includes('failed') || l.includes('err!')) {
			colorClass = 'text-rose-400 font-medium';
		} else if (l.includes('warn') || l.includes('warning')) {
			colorClass = 'text-amber-400';
		} else if (l.includes('success') || l.includes('done') || l.includes('started')) {
			colorClass = 'text-emerald-400';
		}
		return (
			<div key={i} className={`whitespace-pre-wrap break-all leading-relaxed ${colorClass}`}>
				{line}
			</div>
		);
	};

	return (
		<div className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
			<div className="flex items-center justify-between gap-4 flex-wrap">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Logs</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time terminal output stream from the running container</p>
				</div>

				<div className="flex items-center gap-3">
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						{isLive ? <Square className="w-3 h-3 fill-current" /> : <Play className="w-3 h-3 fill-current" />}
						{isLive ? 'Live Stream' : 'Paused'}
					</Button>

					<label className="flex items-center gap-2 cursor-pointer text-xs font-semibold text-muted-foreground">
						<input type="checkbox" checked={timestamps} onChange={e => setTimestamps(e.target.checked)} className="accent-primary w-4 h-4 rounded" />
						Timestamps
					</label>

					<select
						value={lines}
						onChange={e => setLines(e.target.value)}
						className="h-8 rounded-lg border border-border bg-card px-2 text-xs font-semibold text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
					>
						<option value="50">50 lines</option>
						<option value="100">100 lines</option>
						<option value="200">200 lines</option>
						<option value="500">500 lines</option>
					</select>

					<Button variant="outline" size="sm" onClick={() => refetch()} className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5">
						<RefreshCw className="w-3.5 h-3.5" /> Refresh
					</Button>

					{streamedLogs.length > 0 && (
						<Button variant="outline" size="sm" onClick={handleDownload} className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5">
							<Download className="w-3.5 h-3.5" /> Download
						</Button>
					)}
				</div>
			</div>

			{isLoading && streamedLogs.length === 0 ? (
				<div className="rounded-lg bg-zinc-950 border border-border p-4 font-mono text-xs h-96 flex items-center justify-center text-zinc-500">
					<RefreshCw className="w-4 h-4 animate-spin mr-2" /> Connecting to container log stream...
				</div>
			) : streamedLogs.length === 0 ? (
				<div className="rounded-lg bg-zinc-950 border border-border p-4 font-mono text-xs h-96 flex flex-col items-center justify-center text-zinc-600">
					<FileText className="w-8 h-8 mb-2 opacity-50" />
					<p>No container log entries found. The application may be stopped.</p>
				</div>
			) : (
				<div ref={scrollRef} className="rounded-lg bg-zinc-950 border border-border p-4 font-mono text-[11px] h-96 overflow-y-auto flex flex-col gap-1">
					{streamedLogs.map((line, idx) => renderLogLine(line, idx))}
				</div>
			)}
		</div>
	);
}
