import {useEffect, useRef, useState} from 'react';
import {Loader} from 'lucide-react';
import {
	lineTone,
	splitTimestamp,
	stripAnsi,
	LOG_SURFACE_CLS,
	LOG_TS_CLS,
} from '#/components/shared/log-tone';
import {isBuildActive} from './build-status';
import {useDeploymentLogStream} from './use-deployment-log-stream';

interface DeploymentInlineLogsProps {
	deployment: any;
}

/** Inline streaming build-log panel — opens under the clicked deployment row (no modal). The bottom grip is draggable to resize the surface. */
export function DeploymentInlineLogs({
	deployment,
}: DeploymentInlineLogsProps) {
	const id = Number(deployment?.id ?? 0);
	const fallback =
		deployment?.log_content ||
		deployment?.message ||
		deployment?.description;
	const logs = useDeploymentLogStream(id, fallback);
	const isLive = isBuildActive(deployment);

	const [height, setHeight] = useState(300);
	const scrollRef = useRef<HTMLDivElement>(null);
	const dragRef = useRef<{y: number; h: number} | null>(null);

	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [logs.length, isLive]);

	const lines = (logs ?? []).map(stripAnsi).filter(l => l.trim() !== '');

	return (
		<div className="flex flex-col border-t border-border">
			<div ref={scrollRef} style={{height}} className={LOG_SURFACE_CLS}>
				{lines.length === 0 ? (
					<p className="flex items-center gap-2 py-2 font-mono text-[12.5px] text-muted-foreground">
						<Loader className="size-3.5 animate-spin" />
						Connecting to realtime deployment stream...
					</p>
				) : (
					<div className="flex flex-col">
						{lines.map((line, idx) => {
							const [ts, rest] = splitTimestamp(line);
							return (
								<span
									key={idx}
									className={`font-mono text-[12.5px] leading-[25px] break-all whitespace-pre-wrap ${lineTone(line)}`}>
									{ts && <span className={LOG_TS_CLS}>{ts}</span>}
									{rest || ' '}
								</span>
							);
						})}
					</div>
				)}
			</div>
			<div
				role="separator"
				aria-orientation="horizontal"
				title="Drag to resize"
				onPointerDown={e => {
					e.currentTarget.setPointerCapture(e.pointerId);
					dragRef.current = {y: e.clientY, h: height};
				}}
				onPointerMove={e => {
					if (!dragRef.current) return;
					const dy = e.clientY - dragRef.current.y;
					setHeight(Math.min(720, Math.max(140, dragRef.current.h + dy)));
				}}
				onPointerUp={() => (dragRef.current = null)}
				onPointerCancel={() => (dragRef.current = null)}
				className="flex h-4 shrink-0 cursor-row-resize touch-none items-center justify-center border-t border-border bg-muted/20 transition-colors hover:bg-muted/50">
				<div className="h-0.5 w-8 rounded-full bg-border" />
			</div>
		</div>
	);
}
