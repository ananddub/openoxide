import {useEffect} from 'react';
import {type NodeProps, type Node} from '@xyflow/react';
import {
	SlidersHorizontal,
	HardDrive,
	Globe,
	Zap,
	Cpu,
	MemoryStick,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {useContainerMonitoring} from '#/hooks/monitoring/use-container-monitoring';
import {deriveServiceStatus, type ServiceStatus} from './compose-parse';
import {
	ServiceIcon,
	NodeHandles,
	type ComposeServiceNodeData,
} from './compose-node-parts';

export type {ComposeServiceNodeData} from './compose-node-parts';

/** Live CPU/RAM for the node — same mono badge grammar as the project service cards, refreshed every 15s. Hidden until stats arrive. */
function NodeUsageBadge({
	id,
	type,
}: {
	id: number;
	type: 'application' | 'database' | 'compose';
}) {
	const {metrics, triggerRefresh} = useContainerMonitoring(
		id,
		type,
		true,
		false,
	);
	useEffect(() => {
		const timer = setInterval(triggerRefresh, 15_000);
		return () => clearInterval(timer);
	}, [triggerRefresh]);
	if (!metrics) return null;
	return (
		<div className="mt-2 flex min-w-0 items-center gap-2.5 border-t border-border/60 pt-2 font-mono text-[10px] text-muted-foreground">
			<span
				className="flex shrink-0 items-center gap-1"
				title="Live CPU usage">
				<Cpu className="size-3 shrink-0" />
				{metrics.cpuPercent.toFixed(1)}%
			</span>
			<span
				className="flex min-w-0 items-center gap-1"
				title={`Memory ${metrics.memUsage} of ${metrics.memLimit}`}>
				<MemoryStick className="size-3 shrink-0" />
				<span className="truncate">
					{metrics.memUsage} / {metrics.memLimit}
				</span>
			</span>
		</div>
	);
}

const STATUS_DOT: Record<ServiceStatus, {cls: string; label: string}> = {
	running: {cls: 'bg-emerald-500', label: 'Running'},
	busy: {cls: 'animate-pulse bg-amber-500', label: 'Starting / deploying'},
	stopped: {cls: 'bg-muted-foreground/50', label: 'Stopped'},
	error: {cls: 'bg-red-500', label: 'Error'},
};

export function ComposeServiceNode({
	data,
	selected,
}: NodeProps<Node<ComposeServiceNodeData>>) {
	const envCount = Object.keys(data.envVars ?? {}).length;
	const shortImg =
		(data.image ?? '').length > 22
			? (data.image ?? '').slice(0, 22) + '…'
			: (data.image ?? 'no image');
	const status = deriveServiceStatus(
		typeof data.status === 'string' ? data.status : undefined,
	);

	return (
		<div
			className={cn(
				'relative w-[260px] overflow-hidden rounded-xl border border-border bg-card shadow-[0_1px_2px_rgb(0_0_0/0.05),0_4px_12px_rgb(0_0_0/0.08)] transition-[border-color,box-shadow] duration-100 dark:shadow-[0_2px_12px_rgb(0_0_0/0.4)]',
				selected ? 'border-primary/50 shadow-[0_0_0_3px_var(--ring)]' : '',
			)}>
			{/* Main body */}
			<div className="px-3.5 pt-3 pb-2.5">
				{/* Icon + name + action trigger */}
				<div className="mb-2.5 flex items-center gap-2.5">
					<span className="relative shrink-0">
						<ServiceIcon image={data.image} />
						{status && (
							<span
								title={STATUS_DOT[status].label}
								className={cn(
									'absolute -top-0.5 -right-0.5 size-2.5 rounded-full ring-2 ring-card',
									STATUS_DOT[status].cls,
								)}
							/>
						)}
					</span>
					<div className="min-w-0 flex-1">
						<p className="truncate text-[13.5px] font-bold text-foreground">
							{data.name}
						</p>
						<p className="mt-0.5 truncate font-mono text-[10px] text-muted-foreground">
							{shortImg}
						</p>
					</div>
					{!data.readOnly && (
						<div className="flex shrink-0 items-center gap-[3px] rounded-md border border-sky-500/25 bg-sky-500/10 px-[7px] py-[3px] text-[10px] font-bold text-sky-600 dark:text-sky-400">
							<Zap size={10} /> Actions
						</div>
					)}
				</div>

				{/* depends_on badge row */}
				{data.dependsOn.length > 0 && (
					<div className="mb-2 flex flex-wrap gap-1">
						{data.dependsOn.map(d => (
							<span
								key={d}
								className="rounded-full border border-border bg-muted px-[7px] py-[2px] text-[9.5px] font-semibold text-muted-foreground">
								↳ {d}
							</span>
						))}
					</div>
				)}

				{/* Config row */}
				<div className="flex items-center gap-[7px]">
					<SlidersHorizontal size={11} className="text-muted-foreground" />
					<span className="text-[11px] text-muted-foreground">
						{envCount > 0
							? `${envCount} variable${envCount > 1 ? 's' : ''}`
							: 'No config required'}
					</span>
				</div>

				{/* Volume row */}
				{data.volumes.slice(0, 1).map((vol, i) => (
					<div key={i} className="mt-1.5 flex items-center gap-[7px]">
						<HardDrive size={11} className="text-muted-foreground" />
						<span className="truncate font-mono text-[10px] text-muted-foreground">
							{vol}
						</span>
					</div>
				))}

				{/* Ports row */}
				{data.ports.slice(0, 1).map((port, i) => (
					<div key={i} className="mt-1 flex items-center gap-[7px]">
						<Globe size={11} className="text-muted-foreground" />
						<span className="font-mono text-[10px] text-muted-foreground">
							{port}
						</span>
					</div>
				))}

				{/* Live CPU / RAM */}
				{!!data.monitorId && !!data.monitorType && (
					<NodeUsageBadge
						id={data.monitorId}
						type={data.monitorType}
					/>
				)}
			</div>

			<NodeHandles />
		</div>
	);
}
