import {useEffect, useRef} from 'react';
import {Loader} from 'lucide-react';
import {
	lineTone,
	LOG_SURFACE_CLS,
	LOG_TS_CLS,
	splitTimestamp,
	stripAnsi,
} from '#/components/shared/log-tone';

/** Prefix a batch with its arrival time so the log reads like the deploy log. */
export function stampLogs(lines: string[]): string[] {
	const t = new Date().toTimeString().slice(0, 8);
	return lines.map(l => `${t}  ${l}`);
}

/** Minimal streaming setup log — plain deployment-log surface, nothing else. */
export function SetupLogsSurface({
	logs,
	isLive,
	state,
	className = 'h-56 w-full rounded-lg',
}: {
	logs: string[];
	isLive: boolean;
	state?: 'idle' | 'running' | 'success' | 'failed';
	className?: string;
}) {
	const scrollRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [logs.length, isLive]);

	const lines = logs.map(stripAnsi).filter(l => l.trim() !== '');

	return (
		<div ref={scrollRef} className={`${LOG_SURFACE_CLS} ${className}`}>
			{lines.length === 0 ? (
				<p className="flex items-center gap-2 py-2 font-mono text-[12.5px] text-muted-foreground">
					<Loader className="size-3.5 animate-spin" />
					Connecting to server setup stream...
				</p>
			) : (
				<div className="flex flex-col">
					{lines.map((line, idx) => {
						const [ts, rest] = splitTimestamp(line);
						return (
							<span
								key={idx}
								className={`font-mono text-[12.5px] leading-[25px] break-all whitespace-pre-wrap ${lineTone(rest)}`}>
								{ts && (
									<span className={LOG_TS_CLS}>{ts}</span>
								)}
								{rest || ' '}
							</span>
						);
					})}
					{/* Tail line states the verdict when done. */}
					{state === 'failed' && (
						<span className="py-2 font-mono text-[12.5px] text-red-600 dark:text-red-400">
							✗ Setup stopped — fix the issue above, then run it
							again.
						</span>
					)}
					{state === 'success' && (
						<span className="py-2 font-mono text-[12.5px] text-emerald-500 dark:text-emerald-400">
							✓ Setup completed.
						</span>
					)}
				</div>
			)}
		</div>
	);
}
