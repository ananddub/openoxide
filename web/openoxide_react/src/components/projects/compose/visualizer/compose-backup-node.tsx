import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Database, HardDrive, Clock} from 'lucide-react';
import {cn} from '#/api/utils';

export interface VolumeBackupData extends Record<string, unknown> {
	name: string;
	volumeName: string;
	schedule: string;
	service: string;
	status?: 'success' | 'running' | 'failed' | string;
	lastBackup?: string;
}

function formatBackupSchedule(schedule: string): string {
	const c = schedule.trim();
	if (c === '0 0 * * *') return 'Daily at 00:00';
	if (c === '0 2 * * *') return 'Daily at 02:00';
	if (c === '0 * * * *') return 'Every hour';
	if (c === '0 0 * * 0') return 'Weekly on Sun';
	return c;
}

export function ComposeBackupNode({
	data,
	selected,
}: NodeProps<Node<VolumeBackupData>>) {
	const backupName = data.name || 'Volume Backup';
	const volName = data.volumeName || 'data_vol';
	const rawSched = data.schedule || '0 2 * * *';
	const schedText = formatBackupSchedule(rawSched);

	return (
		<div
			className={cn(
				'flex h-[52px] w-[210px] cursor-pointer items-center gap-[9px] rounded-xl border border-sky-500/45 bg-card px-2.5 shadow-[0_1px_2px_rgb(0_0_0/0.05),0_4px_12px_rgb(0_0_0/0.08)] transition-[border-color,box-shadow] duration-100 dark:shadow-[0_2px_12px_rgb(0_0_0/0.4)]',
				selected
					? 'border-sky-500 shadow-[0_0_0_3px_rgb(56_189_248/0.2)]'
					: 'border-sky-500/30',
			)}>
			{/* Only Target Handle from Service (Left side) */}
			<Handle
				type="target"
				position={Position.Left}
				style={{
					width: 7,
					height: 7,
					background: '#38bdf8',
					border: '2px solid var(--card)',
					left: -4,
				}}
			/>

			{/* Icon */}
			<div className="flex size-7 shrink-0 items-center justify-center rounded-[7px] bg-sky-500/15">
				<Database size={15} className="text-sky-500 dark:text-sky-400" />
			</div>

			{/* 3 Distinct Details: 1) Name, 2) Volume, 3) Schedule Time */}
			<div className="flex min-w-0 flex-1 flex-col gap-px">
				<span className="truncate text-[11px] font-bold text-foreground">
					{backupName}
				</span>
				<div className="flex items-center gap-[3.5px]">
					<HardDrive size={9} className="shrink-0 text-muted-foreground" />
					<span className="truncate font-mono text-[9.5px] font-semibold text-muted-foreground">
						{volName}
					</span>
				</div>
				<div className="flex items-center gap-[3.5px]">
					<Clock
						size={9}
						className="shrink-0 text-sky-500 dark:text-sky-400"
					/>
					<span className="truncate font-mono text-[9.5px] font-semibold text-sky-600 dark:text-sky-400">
						{schedText}
					</span>
				</div>
			</div>
		</div>
	);
}
