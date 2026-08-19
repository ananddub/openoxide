import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {Globe} from 'lucide-react';
import {
	siPostgresql,
	siMysql,
	siRedis,
	siMongodb,
	siMariadb,
} from 'simple-icons';
import {useCanvasActions} from '../canvas-context';

// ─── Shared card shell ────────────────────────────────────────────────────────

const HANDLE_STYLE: React.CSSProperties = {
	width: 10,
	height: 10,
	borderRadius: '50%',
	border: '2px solid #30363d',
	background: '#0d1117',
	transition: 'border-color .15s, box-shadow .15s',
};

// ─── App node ─────────────────────────────────────────────────────────────────

export type AppCanvasNodeProps = Node<AppNodeData, 'appNode'>;
export interface AppNodeData extends Record<string, unknown> {
	id: number;
	name: string;
	status: string;
	framework?: string;
	url?: string;
}

export function AppCanvasNode({
	data,
	selected,
}: NodeProps<AppCanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() =>
				inspect({
					id: data.id,
					type: 'app',
					name: data.name,
					status: data.status,
				})
			}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				display: 'flex',
				alignItems: 'center',
				gap: 10,
				width: 185,
				padding: '10px 12px',
				background: '#161b22',
				border: selected
					? '1px solid rgba(88,166,255,0.6)'
					: hov
						? '1px solid rgba(255,255,255,0.16)'
						: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 10,
				cursor: 'pointer',
				userSelect: 'none',
				boxShadow: selected ? '0 0 0 3px rgba(88,166,255,0.15)' : 'none',
				transition: 'border-color .12s, box-shadow .12s',
				position: 'relative',
			}}>
			{/* Status dot */}
			<span
				style={{
					position: 'absolute',
					top: 7,
					right: 8,
					width: 6,
					height: 6,
					borderRadius: '50%',
					background: running ? '#3fb950' : '#6e7681',
					boxShadow: running ? '0 0 6px #3fb950' : 'none',
				}}
			/>

			{/* Icon */}
			<div
				style={{
					width: 28,
					height: 28,
					borderRadius: 7,
					background: 'rgba(88,166,255,0.1)',
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					flexShrink: 0,
				}}>
				<Globe size={14} color="#58a6ff" strokeWidth={1.8} />
			</div>

			{/* Name */}
			<div style={{flex: 1, minWidth: 0}}>
				<p
					style={{
						margin: 0,
						fontSize: 12.5,
						fontWeight: 600,
						color: '#e6edf3',
						lineHeight: 1.2,
						whiteSpace: 'nowrap',
						overflow: 'hidden',
						textOverflow: 'ellipsis',
						paddingRight: 14,
					}}>
					{data.name}
				</p>
				<p
					style={{
						margin: 0,
						fontSize: 10,
						color: '#6e7681',
						marginTop: 2,
					}}>
					{data.framework || 'Service'}
				</p>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				style={{...HANDLE_STYLE, left: -6}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{...HANDLE_STYLE, right: -6}}
			/>
		</div>
	);
}

// ─── DB node ─────────────────────────────────────────────────────────────────

export type DbCanvasNodeProps = Node<DbNodeData, 'dbNode'>;
export interface DbNodeData extends Record<string, unknown> {
	id: number;
	name: string;
	dbType: string;
	status: string;
	port?: number;
}

function getDbIcon(dbType: string) {
	const t = (dbType ?? '').toLowerCase();
	if (t.includes('mysql')) return {icon: siMysql, color: '#4479A1'};
	if (t.includes('redis')) return {icon: siRedis, color: '#FF4438'};
	if (t.includes('mongo')) return {icon: siMongodb, color: '#47A248'};
	if (t.includes('maria')) return {icon: siMariadb, color: '#7b8cde'};
	return {icon: siPostgresql, color: '#336791'};
}

export function DbCanvasNode({
	data,
	selected,
}: NodeProps<DbCanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const {icon, color} = getDbIcon(data.dbType as string);
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() =>
				inspect({
					id: data.id,
					type: 'database',
					name: data.name,
					status: data.status,
				})
			}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				display: 'flex',
				alignItems: 'center',
				gap: 10,
				width: 185,
				padding: '10px 12px',
				background: '#161b22',
				border: selected
					? `1px solid ${color}80`
					: hov
						? '1px solid rgba(255,255,255,0.16)'
						: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 10,
				cursor: 'pointer',
				userSelect: 'none',
				boxShadow: selected ? `0 0 0 3px ${color}20` : 'none',
				transition: 'border-color .12s, box-shadow .12s',
				position: 'relative',
			}}>
			<span
				style={{
					position: 'absolute',
					top: 7,
					right: 8,
					width: 6,
					height: 6,
					borderRadius: '50%',
					background: running ? '#3fb950' : '#6e7681',
					boxShadow: running ? '0 0 6px #3fb950' : 'none',
				}}
			/>

			<div
				style={{
					width: 28,
					height: 28,
					borderRadius: 7,
					background: `${color}18`,
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					flexShrink: 0,
				}}>
				<svg
					role="img"
					viewBox="0 0 24 24"
					style={{width: 14, height: 14, fill: color}}>
					<path d={icon.path} />
				</svg>
			</div>

			<div style={{flex: 1, minWidth: 0}}>
				<p
					style={{
						margin: 0,
						fontSize: 12.5,
						fontWeight: 600,
						color: '#e6edf3',
						lineHeight: 1.2,
						whiteSpace: 'nowrap',
						overflow: 'hidden',
						textOverflow: 'ellipsis',
						paddingRight: 14,
					}}>
					{data.name}
				</p>
				<p
					style={{
						margin: 0,
						fontSize: 10,
						color: '#6e7681',
						marginTop: 2,
						fontFamily: 'monospace',
						textTransform: 'uppercase',
						letterSpacing: '0.04em',
					}}>
					{data.dbType}
				</p>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				style={{...HANDLE_STYLE, left: -6, borderColor: color}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{...HANDLE_STYLE, right: -6, borderColor: color}}
			/>
		</div>
	);
}
