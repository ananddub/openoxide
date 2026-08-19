import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Database, HardDrive, Clock} from 'lucide-react';

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
			style={{
				width: 210,
				height: 52,
				borderRadius: 10,
				background: '#0a1624',
				border: selected
					? '1px solid #38bdf8'
					: '1px solid rgba(56,189,248,0.35)',
				boxShadow: selected
					? '0 0 0 3px rgba(56,189,248,0.2)'
					: '0 2px 10px rgba(56,189,248,0.08)',
				display: 'flex',
				alignItems: 'center',
				padding: '0 10px',
				gap: 9,
				cursor: 'pointer',
			}}>
			{/* Only Target Handle from Service (Left side) */}
			<Handle
				type="target"
				position={Position.Left}
				style={{
					width: 7,
					height: 7,
					background: '#38bdf8',
					border: '2px solid #0a1624',
					left: -4,
				}}
			/>

			{/* Icon */}
			<div
				style={{
					width: 28,
					height: 28,
					borderRadius: 7,
					background: 'rgba(56,189,248,0.15)',
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					flexShrink: 0,
				}}>
				<Database size={15} color="#38bdf8" />
			</div>

			{/* 3 Distinct Details: 1) Name, 2) Volume, 3) Schedule Time */}
			<div
				style={{
					flex: 1,
					minWidth: 0,
					display: 'flex',
					flexDirection: 'column',
					gap: 1,
				}}>
				<span
					style={{
						fontSize: 11,
						fontWeight: 700,
						color: '#f0f9ff',
						whiteSpace: 'nowrap',
						overflow: 'hidden',
						textOverflow: 'ellipsis',
					}}>
					{backupName}
				</span>

				<div style={{display: 'flex', alignItems: 'center', gap: 3.5}}>
					<HardDrive size={9} color="#94a3b8" />
					<span
						style={{
							fontSize: 9.5,
							fontWeight: 600,
							fontFamily: 'monospace',
							color: '#94a3b8',
							whiteSpace: 'nowrap',
							overflow: 'hidden',
							textOverflow: 'ellipsis',
						}}>
						{volName}
					</span>
				</div>

				<div style={{display: 'flex', alignItems: 'center', gap: 3.5}}>
					<Clock size={9} color="#38bdf8" />
					<span
						style={{
							fontSize: 9.5,
							fontWeight: 600,
							fontFamily: 'monospace',
							color: '#38bdf8',
							whiteSpace: 'nowrap',
							overflow: 'hidden',
							textOverflow: 'ellipsis',
						}}>
						{schedText}
					</span>
				</div>
			</div>
		</div>
	);
}
