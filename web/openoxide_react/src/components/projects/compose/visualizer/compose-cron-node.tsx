import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Clock} from 'lucide-react';
import {cn} from '#/api/utils';
import {describeCron} from '#/utils/cron';

export interface CronJobData extends Record<string, unknown> {
	name: string;
	cron?: string;
	cron_expression?: string;
	target?: string;
	service_name?: string;
}

export function ComposeCronNode({
	data,
	selected,
}: NodeProps<Node<CronJobData>>) {
	const rawCron = data.cron || data.cron_expression || '0 * * * *';
	const taskName = data.name || 'Cron Task';
	const intervalText = describeCron(rawCron) ?? rawCron;

	return (
		<div
			className={cn(
				'flex h-[44px] w-[200px] cursor-pointer items-center gap-[9px] rounded-xl border border-purple-500/60 bg-card px-2.5 shadow-[0_1px_2px_rgb(0_0_0/0.05),0_4px_12px_rgb(0_0_0/0.08)] transition-[border-color,box-shadow] duration-100 dark:shadow-[0_2px_12px_rgb(0_0_0/0.4)]',
				selected
					? 'border-purple-500 shadow-[0_0_0_3px_rgb(192_132_252/0.2)]'
					: '',
			)}>
			<div className="flex size-[26px] shrink-0 items-center justify-center rounded-[7px] bg-purple-500/15 ring-1 ring-purple-500/30">
				<Clock
					size={14}
					className="text-purple-600 dark:text-purple-400"
				/>
			</div>

			<div className="flex min-w-0 flex-1 flex-col gap-px">
				<span className="truncate text-[11.5px] font-bold text-foreground">
					{taskName}
				</span>
				<span className="truncate text-[9.5px] font-semibold text-purple-600 dark:text-purple-400">
					{intervalText}
				</span>
			</div>

			{/* Only Source Handle to Service (Right side) */}
			<Handle
				type="source"
				position={Position.Right}
				style={{
					width: 7,
					height: 7,
					background: '#c084fc',
					border: '2px solid var(--card)',
					right: -4,
				}}
			/>
		</div>
	);
}
