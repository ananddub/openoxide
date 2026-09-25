import {useEffect, useMemo, useRef, useState} from 'react';
import {Download, Loader, Radio, RefreshCw, Search} from 'lucide-react';
import {cn} from '#/api/utils';
import {AnalyzeLogsButton} from '#/components/ai/analyze-logs-button';
import type {AiLogAnalysisKind} from '#/api/live/ai';
import {
	lineTone,
	splitTimestamp,
	stripAnsi,
	LOG_SURFACE_CLS,
	LOG_TS_CLS,
} from './log-tone';

export interface DeploymentViewerProps {
	logs?: string[] | null;
	isLoading?: boolean;
	loadingText?: string;
	emptyText?: string;
	/** Log surface height; the default fills the viewport under the page chrome instead of clipping to a fixed strip. */
	heightClass?: string;
	onDownload?: () => void;
	onReload?: () => void;
	onToggleLive?: () => void;
	isLive?: boolean;
	/** Kept for call-site compatibility; the minimal viewer ignores it. */
	isDeployment?: boolean;
	borderless?: boolean;
	/** When set, shows an AI analyze button in the toolbar for this log corpus. */
	aiContext?: AiLogAnalysisKind;
}

const ICON_BTN =
	'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted/60 hover:text-foreground disabled:cursor-default disabled:opacity-40';

/** Minimalist log surface — same black terminal look as the architecture card's Build Logs strip: plain white mono lines, dim timestamps, colour only on error/warning/success, plus line count and find-in-logs. */
export function DeploymentViewer({
	logs,
	isLoading = false,
	loadingText = 'Connecting to deployment log stream...',
	emptyText = 'No deployment logs received.',
	heightClass = 'h-[calc(100dvh_-_260px)] min-h-[360px]',
	onDownload,
	onReload,
	onToggleLive,
	isLive = true,
	borderless = false,
	aiContext,
}: DeploymentViewerProps) {
	const [query, setQuery] = useState('');
	const searchRef = useRef<HTMLInputElement>(null);
	const scrollRef = useRef<HTMLDivElement>(null);

	// Callers may pass raw stream chunks containing embedded newlines; colour and count per real line so one FATAL doesn't redden the whole blob.
	const lines = useMemo(
		() =>
			(logs ?? [])
				.flatMap(l => stripAnsi(l).split('\n'))
				.filter(l => l.trim() !== ''),
		[logs],
	);

	const visibleLines = useMemo(() => {
		const q = query.trim().toLowerCase();
		if (!q) return lines;
		return lines.filter(l => l.toLowerCase().includes(q));
	}, [lines, query]);

	// Ctrl+F focuses the in-panel search.
	useEffect(() => {
		const onKey = (e: KeyboardEvent) => {
			if (e.defaultPrevented) return;
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
				e.preventDefault();
				searchRef.current?.focus();
			}
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, []);

	// Follow the tail while live.
	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [visibleLines.length, isLive]);

	return (
		<div
			className={cn(
				'flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-xl border border-border bg-card',
				borderless && 'border-0 bg-transparent',
			)}>
			{/* Toolbar: count | find-in-logs + controls */}
			<div className="flex shrink-0 items-center gap-2.5 border-b border-border px-4 py-2.5">
				<span className="text-[13px] text-muted-foreground">
					{lines.length} line{lines.length === 1 ? '' : 's'}
					{query.trim() && ` · ${visibleLines.length} match`}
				</span>
				<div className="relative ml-auto w-72 max-w-[50%]">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground" />
					<input
						ref={searchRef}
						value={query}
						onChange={e => setQuery(e.target.value)}
						placeholder="Find in logs"
						className="h-8 w-full rounded-lg border border-border bg-muted/30 pr-14 pl-8 text-[13px] text-foreground outline-none placeholder:text-muted-foreground focus:border-neutral-500"
					/>
					<kbd className="pointer-events-none absolute top-1/2 right-2 -translate-y-1/2 rounded border border-border bg-background px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
						Ctrl+F
					</kbd>
				</div>

				{aiContext && (
					<AnalyzeLogsButton lines={lines} context={aiContext} />
				)}

				{onToggleLive && (
					<button
						type="button"
						onClick={onToggleLive}
						className={cn(
							'flex h-7 cursor-pointer items-center gap-1.5 rounded-md border px-2 text-[11px] font-medium transition-colors',
							isLive
								? 'border-emerald-500/30 text-emerald-500'
								: 'border-border text-muted-foreground hover:text-foreground',
						)}>
						<Radio className={cn('size-3', isLive && 'animate-pulse')} />
						{isLive ? 'Live' : 'Paused'}
					</button>
				)}
				{onReload && (
					<button
						type="button"
						onClick={onReload}
						aria-label="Reload logs"
						className={ICON_BTN}>
						<RefreshCw className="size-3.5" />
					</button>
				)}
				{onDownload && (
					<button
						type="button"
						onClick={onDownload}
						aria-label="Download logs"
						className={ICON_BTN}>
						<Download className="size-3.5" />
					</button>
				)}
			</div>

			{/* Theme-aware log surface — flush inside the card */}
			<div ref={scrollRef} className={cn(LOG_SURFACE_CLS, heightClass)}>
				{isLoading && lines.length === 0 ? (
					<p className="flex items-center gap-2 py-2 font-mono text-[12.5px] text-muted-foreground">
						<Loader className="size-3.5 animate-spin" />
						{loadingText}
					</p>
				) : visibleLines.length === 0 ? (
					<p className="py-2 font-mono text-[12.5px] text-muted-foreground">
						{query.trim()
							? `No log line matches "${query.trim()}".`
							: emptyText}
					</p>
				) : (
					<div className="flex flex-col">
						{visibleLines.map((line, idx) => {
							const [ts, rest] = splitTimestamp(line);
							return (
								<span
									key={idx}
									className={cn(
										'font-mono text-[12.5px] leading-[25px] break-all whitespace-pre-wrap',
										lineTone(line),
									)}>
									{ts && <span className={LOG_TS_CLS}>{ts}</span>}
									{rest || ' '}
								</span>
							);
						})}
					</div>
				)}
			</div>
		</div>
	);
}
