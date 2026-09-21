import {type NodeProps, type Node} from '@xyflow/react';
import {Globe} from 'lucide-react';
import {
	siPostgresql,
	siMysql,
	siRedis,
	siMongodb,
	siMariadb,
} from 'simple-icons';
import {useCanvasActions} from '../canvas-context';
import {NodeShell} from './railway-node-shell';

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

	return (
		<NodeShell
			selected={selected}
			running={running}
			iconBg="rgba(88,166,255,0.1)"
			selectedBorder="1px solid rgba(88,166,255,0.6)"
			selectedShadow="0 0 0 3px rgba(88,166,255,0.15)"
			icon={<Globe size={14} color="#58a6ff" strokeWidth={1.8} />}
			name={data.name}
			subtitle={data.framework || 'Service'}
			onClick={() =>
				inspect({
					id: data.id,
					type: 'app',
					name: data.name,
					status: data.status,
				})
			}
		/>
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

	return (
		<NodeShell
			selected={selected}
			running={running}
			iconBg={`${color}18`}
			selectedBorder={`1px solid ${color}80`}
			selectedShadow={`0 0 0 3px ${color}20`}
			handleAccent={color}
			icon={
				<svg
					role="img"
					viewBox="0 0 24 24"
					style={{width: 14, height: 14, fill: color}}>
					<path d={icon.path} />
				</svg>
			}
			name={data.name}
			subtitle={data.dbType}
			subtitleStyle={{
				fontFamily: 'monospace',
				textTransform: 'uppercase',
				letterSpacing: '0.04em',
			}}
			onClick={() =>
				inspect({
					id: data.id,
					type: 'database',
					name: data.name,
					status: data.status,
				})
			}
		/>
	);
}
