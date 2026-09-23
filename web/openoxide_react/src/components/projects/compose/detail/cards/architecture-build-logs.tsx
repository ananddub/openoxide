import {useEffect, useMemo, useRef, useState} from 'react';
import {
	ChevronDown,
	CircleCheck,
	CircleDashed,
	CircleX,
	Copy,
	Loader,
	Search,
} from 'lucide-react';
import {toast} from 'sonner';
import {useDeploymentItems} from '#/api/live';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import {isBuildActive, isBuildSucceeded} from '#/components/projects/detail/deployments/build-status';
import {
	Collapsible,
	CollapsibleTrigger,
	CollapsibleContent,
} from '#/components/ui/collapsible';
import {
	lineTone,
	splitTimestamp,
	stripAnsi,
	LOG_SURFACE_CLS,
	LOG_TS_CLS,
} from '#/components/shared/log-tone';
import {cn} from '#/api/utils';

const deploymentClient = createServiceClient(DeploymentService);

type LogsKind = 'app' | 'compose' | 'database';

const ID_FIELD: Record<LogsKind, string> = {
	app: 'application_id',
	compose: 'compose_id',
	database: 'database_id',
};

/** Epoch (s or ms) diff as "17s" / "2m 05s". */
function formatDuration(
	created?: number | null,
	finished?: number | null,
): string | null {
	const a = Number(created);
	const b = Number(finished);
	if (!a || !b || b <= a) return null;
	const secA = a < 1e11 ? a : a / 1000;
	const secB = b < 1e11 ? b : b / 1000;
	const secs = Math.max(0, Math.round(secB - secA));
	if (secs < 60) return `${secs}s`;
	return `${Math.floor(secs / 60)}m ${String(secs % 60).padStart(2, '0')}s`;
}

function StatusIcon({status}: {status: string}) {
	const s = (status || '').toUpperCase();
	if (
		s.includes('DONE') ||
		s.includes('SUCCESS') ||
		s.includes('DEPLOYED') ||
		s.includes('HEALTHY')
	)
		return (
			<CircleCheck className="size-[18px] fill-blue-500 text-white" />
		);
	if (s.includes('ERROR') || s.includes('FAIL') || s.includes('CRASH'))
		return (
			<CircleX className="size-[18px] fill-red-500 text-white" />
		);
	if (isBuildActive({status: s}))
		return <Loader className="size-[18px] animate-spin text-amber-500" />;
	return <CircleDashed className="size-[18px] text-neutral-500" />;
}

interface BuildLogsSectionProps {
	entityId?: number | null;
	kind: LogsKind;
}

/**
 * Latest build/deploy log for this entity, embedded in the architecture
 * card — a faithful port of the reference build-logs panel: chevron +
 * plain-case header with duration and status circle, copy/line-count +
 * find-in-logs toolbar, and a full-bleed black mono log surface.
 */
export function BuildLogsSection({
	entityId,
	kind,
}: BuildLogsSectionProps) {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const [logs, setLogs] = useState('');
	const [isLoading, setIsLoading] = useState(false);
	const searchRef = useRef<HTMLInputElement>(null);
	const {data: deployments = []} = useDeploymentItems();

	const latest = useMemo(() => {
		if (!entityId) return null;
		const field = ID_FIELD[kind];
		const matches = (deployments as any[])
			.filter(d => String(d?.[field]) === String(entityId))
			.sort((a, b) => Number(b?.id ?? 0) - Number(a?.id ?? 0));
		return matches[0] ?? null;
	}, [deployments, entityId, kind]);

	const deploymentId = latest ? Number(latest.id ?? 0) : 0;

	// Auto-follow the build: open the strip when a build starts, close it on
	// success, and leave it open when the build failed so the error reads.
	const watched = useRef<number | null>(null);
	useEffect(() => {
		if (!deploymentId) return;
		if (isBuildActive(latest)) {
			if (watched.current !== deploymentId) {
				watched.current = deploymentId;
				setOpen(true);
			}
		} else if (watched.current === deploymentId) {
			watched.current = null;
			if (isBuildSucceeded(latest)) setOpen(false);
		}
	}, [latest, deploymentId]);

	// The stored `logs` column only carries the file path on the panel;
	// the agent tails /etc/openoxide/logs/deployments/<id>.log over the
	// deployment-log stream, so fetch lazily when the strip is opened.
	useEffect(() => {
		if (!open || !deploymentId) return;
		let mounted = true;
		const controller = new AbortController();
		setLogs('');
		setIsLoading(true);
		(async () => {
			let acc = '';
			try {
				const call = deploymentClient.streamDeploymentLogs(
					{id: BigInt(deploymentId)},
					{signal: controller.signal},
				);
				for await (const event of call) {
					if (!mounted) return;
					if (event.error) break;
					const ls = extractLogLines(event.payload);
					if (ls.length > 0) {
						acc = acc ? `${acc}\n${ls.join('\n')}` : ls.join('\n');
						setLogs(acc);
					}
				}
			} catch {
				// Stream failed (network drop, abort) — fall back below.
			}
			if (mounted) {
				setIsLoading(false);
				if (!acc) {
					const fallback = String(
						latest?.error_message || latest?.error || '',
					);
					if (fallback) setLogs(fallback);
				}
			}
		})();
		return () => {
			mounted = false;
			controller.abort();
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [open, deploymentId]);

	const cleanLogs = stripAnsi(logs).trim();
	const lines = useMemo(
		() => (cleanLogs ? cleanLogs.split('\n') : []),
		[cleanLogs],
	);

	const visibleLines = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return lines;
		return lines.filter(l => l.toLowerCase().includes(q));
	}, [lines, query]);

	const duration = latest
		? formatDuration(latest.created_at, latest.finished_at)
		: null;

	// Ctrl+F focuses the in-panel search while the strip is open.
	useEffect(() => {
		if (!open) return;
		const onKey = (e: KeyboardEvent) => {
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
				e.preventDefault();
				searchRef.current?.focus();
			}
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, [open]);

	const handleCopy = async () => {
		try {
			await navigator.clipboard.writeText(cleanLogs);
			toast.success('Logs copied to clipboard');
		} catch {
			toast.error('Could not copy');
		}
	};

	return (
		<Collapsible
			open={open}
			onOpenChange={setOpen}
			className="border-t border-border">
			<CollapsibleTrigger
				className={cn(
					'group flex w-full cursor-pointer items-center gap-3 px-5 py-3.5 text-left transition-colors hover:bg-muted/30',
					open && 'bg-muted/30',
				)}>
				<ChevronDown
					className={cn(
						'size-4 shrink-0 text-muted-foreground transition-transform duration-200',
						!open && '-rotate-90',
					)}
				/>
				<span className="text-sm font-medium text-foreground">
					Build Logs
				</span>
				{latest && (
					<span className="ml-auto flex items-center gap-2.5">
						{duration && (
							<span className="font-mono text-[13px] text-muted-foreground">
								{duration}
							</span>
						)}
						<StatusIcon status={String(latest.status || '')} />
					</span>
				)}
			</CollapsibleTrigger>
			<CollapsibleContent>
				{lines.length > 0 || isLoading ? (
					<div>
						{/* Toolbar: copy + line count | find in logs */}
						<div className="flex items-center gap-2.5 border-y border-border px-5 py-3">
							<button
								type="button"
								onClick={handleCopy}
								disabled={!lines.length}
								aria-label="Copy logs"
								className="flex cursor-pointer items-center justify-center text-muted-foreground transition-colors hover:text-foreground disabled:cursor-default disabled:opacity-40">
								<Copy className="size-4" />
							</button>
							<span className="text-[13px] text-muted-foreground">
								{lines.length} line
								{lines.length === 1 ? '' : 's'}
							</span>
							<div className="relative ml-auto w-72">
								<Search className="pointer-events-none absolute left-2.5 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground" />
								<input
									ref={searchRef}
									value={query}
									onChange={e => setQuery(e.target.value)}
									placeholder="Find in logs"
									className="h-8 w-full rounded-lg border border-border bg-muted/30 pl-8 pr-14 text-[13px] text-foreground outline-none placeholder:text-muted-foreground focus:border-neutral-500"
								/>
								<kbd className="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
									Ctrl+F
								</kbd>
							</div>
						</div>

						{/* Theme-aware log surface — bordered, no inset */}
						<div className={cn('max-h-[420px] overflow-auto border-x border-b border-border', LOG_SURFACE_CLS)}>
							{lines.length === 0 && isLoading ? (
								<p className="flex items-center gap-2 py-2 font-mono text-[12.5px] text-muted-foreground">
									<Loader className="size-3.5 animate-spin" />
									Loading build logs...
								</p>
								) : visibleLines.length > 0 ? (
									<div className="flex flex-col">
										{visibleLines.map((line, i) => {
											const [ts, rest] =
												splitTimestamp(line);
											return (
												<span
													key={i}
													className={cn(
														'break-all whitespace-pre-wrap font-mono text-[12.5px] leading-[25px]',
														lineTone(line),
													)}>
													{ts && (
														<span className={LOG_TS_CLS}>
															{ts}
														</span>
													)}
													{rest || ' '}
												</span>
											);
										})}
									</div>
							) : (
								<p className="py-2 font-mono text-[12.5px] text-muted-foreground">
									No log line matches "{query.trim()}".
								</p>
							)}
						</div>
					</div>
				) : (
					<p className="px-5 pb-4 text-[13px] text-muted-foreground">
						Build logs will appear here after the first
						deployment.
					</p>
				)}
			</CollapsibleContent>
		</Collapsible>
	);
}
