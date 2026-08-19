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

export const ComposeCanvasNode = memo(
	({data}: {data: ComposeNodeData}) => {
		const {
			name,
			status,
			servicesCount = 1,
			cpuUsage,
			memoryUsage,
			onInspect,
		} = data;
		const isRunning =
			status?.toLowerCase() === 'running' ||
			status?.toLowerCase() === 'active';

		return (
			<div
				onClick={() => onInspect?.(data.id)}
				className={`group relative min-w-[240px] cursor-pointer rounded-xl border bg-card/95 p-3.5 shadow-md backdrop-blur-md transition-all select-none hover:shadow-lg ${
					isRunning
						? 'border-violet-500/40 hover:border-violet-500/70'
						: 'border-border hover:border-border/80'
				}`}>
				<Handle
					type="target"
					position={Position.Top}
					id="top-target"
					className="h-3 w-3 border-2 border-background !bg-violet-500 transition-transform hover:scale-125"
				/>
				<Handle
					type="target"
					position={Position.Left}
					id="left-target"
					className="h-3 w-3 border-2 border-background !bg-violet-500 transition-transform hover:scale-125"
				/>
				<Handle
					type="source"
					position={Position.Right}
					id="right-source"
					className="h-3 w-3 border-2 border-background !bg-violet-500 transition-transform hover:scale-125"
				/>
				<Handle
					type="source"
					position={Position.Bottom}
					id="bottom-source"
					className="h-3 w-3 border-2 border-background !bg-violet-500 transition-transform hover:scale-125"
				/>

				{/* Header */}
				<div className="mb-2 flex items-center justify-between gap-2">
					<div className="flex min-w-0 items-center gap-2.5">
						<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-violet-500/20 bg-violet-500/10 text-violet-500">
							<Layers className="h-4 w-4" />
						</div>
						<div className="min-w-0 flex-1">
							<h4
								className="truncate text-xs font-bold text-foreground"
								title={name}>
								{name}
							</h4>
							<span className="font-mono text-[10px] text-muted-foreground uppercase">
								COMPOSE STACK ({servicesCount} SVC)
							</span>
						</div>
					</div>

					<div className="flex shrink-0 items-center gap-1.5">
						<span
							className={`h-2.5 w-2.5 rounded-full border-2 border-background ${
								isRunning ? 'bg-emerald-500' : 'bg-zinc-500/60'
							}`}
							title={isRunning ? 'Running' : 'Stopped'}
						/>
					</div>
				</div>

				{/* Telemetry Row */}
				<div className="flex items-center justify-between gap-2 border-t border-border/40 pt-2 font-mono text-[11px]">
					<div className="flex items-center gap-1 text-muted-foreground">
						<Cpu className="h-3 w-3 text-sky-500" />
						<span>{cpuUsage || '0.0%'}</span>
					</div>
					<div className="flex items-center gap-1 text-muted-foreground">
						<HardDrive className="h-3 w-3 text-violet-500" />
						<span>{memoryUsage || '0 MB'}</span>
					</div>
				</div>
			</div>
		);
	},
);

ComposeCanvasNode.displayName = 'ComposeCanvasNode';
