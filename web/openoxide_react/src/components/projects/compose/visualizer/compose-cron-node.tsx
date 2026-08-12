import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Clock} from 'lucide-react';

export interface CronJobData extends Record<string, unknown> {
	name:            string;
	cron?:           string;
	cron_expression?: string;
	target?:         string;
	service_name?:   string;
}

function formatCronInterval(cron: string): string {
	const c = cron.trim();
	if (!c) return 'Scheduled Task';
	if (c === '*/5 * * * *') return 'Every 5 mins';
	if (c === '*/10 * * * *') return 'Every 10 mins';
	if (c === '*/15 * * * *') return 'Every 15 mins';
	if (c === '*/30 * * * *') return 'Every 30 mins';
	if (c === '0 * * * *') return 'Every hour';
	if (c === '0 0 * * *') return 'Daily at 00:00';
	if (c === '0 2 * * *') return 'Daily at 02:00';
	if (c === '0 0 * * 0') return 'Weekly on Sunday';
	if (c === '0 0 * * 1') return 'Weekly on Monday';

	const parts = c.split(/\s+/);
	if (parts.length === 5 && parts[2] === '*' && parts[3] === '*' && parts[4] === '*') {
		const minute = parts[0].padStart(2, '0');
		const hour = parts[1].padStart(2, '0');
		if (!isNaN(Number(minute)) && !isNaN(Number(hour))) {
			return `Daily at ${hour}:${minute}`;
		}
	}

	return 'Scheduled Task';
}

export function ComposeCronNode({data, selected}: NodeProps<Node<CronJobData>>) {
	const rawCron = data.cron || data.cron_expression || '0 * * * *';
	const taskName = data.name || 'Cron Task';
	const intervalText = formatCronInterval(rawCron);

	return (
		<div style={{
			width: 200,
			height: 44,
			borderRadius: 10,
			background: '#150c22',
			border: selected
				? '1px solid #c084fc'
				: '1px solid rgba(192,132,252,0.3)',
			boxShadow: selected
				? '0 0 0 3px rgba(192,132,252,0.2)'
				: '0 2px 10px rgba(192,132,252,0.08)',
			display: 'flex',
			alignItems: 'center',
			padding: '0 10px',
			gap: 9,
			cursor: 'pointer',
		}}>
			<div style={{
				width: 26, height: 26, borderRadius: 7,
				background: 'rgba(192,132,252,0.15)',
				display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
			}}>
				<Clock size={14} color="#c084fc" />
			</div>

			<div style={{
				flex: 1, minWidth: 0,
				display: 'flex', flexDirection: 'column', gap: 1,
			}}>
				<span style={{
					fontSize: 11.5, fontWeight: 700, color: '#faf5ff',
					whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
				}}>
					{taskName}
				</span>

				<span style={{
					fontSize: 9.5, fontWeight: 600, fontFamily: 'monospace', color: '#c084fc',
					whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
				}}>
					{intervalText}
				</span>
			</div>

			{/* Only Source Handle to Service (Right side) */}
			<Handle type="source" position={Position.Right}
				style={{width: 7, height: 7, background: '#c084fc', border: '2px solid #150c22', right: -4}} />
		</div>
	);
}
