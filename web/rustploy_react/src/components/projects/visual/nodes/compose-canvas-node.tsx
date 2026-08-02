import {memo} from 'react';
import {Handle, Position} from '@xyflow/react';
import {Layers, Cpu, HardDrive} from 'lucide-react';

export type ComposeNodeData = {
	id: number;
	name: string;
	status?: string;
	servicesCount?: number;
	cpuUsage?: string;
	memoryUsage?: string;
	onInspect?: (id: number) => void;
};

export const ComposeCanvasNode = memo(({data}: {data: ComposeNodeData}) => {
	const {name, status, servicesCount = 1, cpuUsage, memoryUsage, onInspect} = data;
	const isRunning = status?.toLowerCase() === 'running' || status?.toLowerCase() === 'active';

	return (
		<div
			onClick={() => onInspect?.(data.id)}
			className={`group relative min-w-[240px] bg-card/95 backdrop-blur-md border rounded-xl p-3.5 shadow-md hover:shadow-lg transition-all cursor-pointer select-none ${
				isRunning
					? 'border-violet-500/40 hover:border-violet-500/70'
					: 'border-border hover:border-border/80'
			}`}
		>
			<Handle
				type="target"
				position={Position.Top}
				id="top-target"
				className="w-3 h-3 !bg-violet-500 border-2 border-background hover:scale-125 transition-transform"
			/>
			<Handle
				type="target"
				position={Position.Left}
				id="left-target"
				className="w-3 h-3 !bg-violet-500 border-2 border-background hover:scale-125 transition-transform"
			/>
			<Handle
				type="source"
				position={Position.Right}
				id="right-source"
				className="w-3 h-3 !bg-violet-500 border-2 border-background hover:scale-125 transition-transform"
			/>
			<Handle
				type="source"
				position={Position.Bottom}
				id="bottom-source"
				className="w-3 h-3 !bg-violet-500 border-2 border-background hover:scale-125 transition-transform"
			/>

			{/* Header */}
			<div className="flex items-center justify-between gap-2 mb-2">
				<div className="flex items-center gap-2.5 min-w-0">
					<div className="w-8 h-8 rounded-lg bg-violet-500/10 border border-violet-500/20 text-violet-500 flex items-center justify-center shrink-0">
						<Layers className="w-4 h-4" />
					</div>
					<div className="min-w-0 flex-1">
						<h4 className="text-xs font-bold text-foreground truncate" title={name}>
							{name}
						</h4>
						<span className="text-[10px] font-mono text-muted-foreground uppercase">
							COMPOSE STACK ({servicesCount} SVC)
						</span>
					</div>
				</div>

				<div className="flex items-center gap-1.5 shrink-0">
					<span
						className={`w-2.5 h-2.5 rounded-full border-2 border-background ${
							isRunning ? 'bg-emerald-500' : 'bg-zinc-500/60'
						}`}
						title={isRunning ? 'Running' : 'Stopped'}
					/>
				</div>
			</div>

			{/* Telemetry Row */}
			<div className="flex items-center justify-between gap-2 pt-2 border-t border-border/40 text-[11px] font-mono">
				<div className="flex items-center gap-1 text-muted-foreground">
					<Cpu className="w-3 h-3 text-sky-500" />
					<span>{cpuUsage || '0.0%'}</span>
				</div>
				<div className="flex items-center gap-1 text-muted-foreground">
					<HardDrive className="w-3 h-3 text-violet-500" />
					<span>{memoryUsage || '0 MB'}</span>
				</div>
			</div>
		</div>
	);
});

ComposeCanvasNode.displayName = 'ComposeCanvasNode';
