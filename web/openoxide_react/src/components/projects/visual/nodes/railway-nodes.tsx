import {type NodeProps, type Node} from '@xyflow/react';
import {Globe} from 'lucide-react';
import {siRedis, siMongodb} from 'simple-icons';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
} from '#/components/icons/db/postgres-mysql';
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
	if (t.includes('mysql') && !t.includes('maria'))
		return {brand: MysqlIcon, color: '#4479A1'};
	if (t.includes('redis')) return {icon: siRedis, color: '#FF4438'};
	if (t.includes('mongo')) return {icon: siMongodb, color: '#47A248'};
	if (t.includes('maria')) return {brand: MariadbIcon, color: '#7b8cde'};
	return {brand: PostgresqlIcon, color: '#336791'};
}

export function DbCanvasNode({
	data,
	selected,
}: NodeProps<DbCanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const {
		icon,
		brand: BrandIcon,
		color,
	} = getDbIcon(data.dbType as string) as {
		icon?: {path: string};
		brand?: (p: {className?: string}) => React.ReactElement;
		color: string;
	};

	return (
		<NodeShell
			selected={selected}
			running={running}
			iconBg={`${color}18`}
			selectedBorder={`1px solid ${color}80`}
			selectedShadow={`0 0 0 3px ${color}20`}
			handleAccent={color}
			icon={
				BrandIcon ? (
					<BrandIcon className="size-[14px]" />
				) : (
					<svg
						role="img"
						viewBox="0 0 24 24"
						style={{width: 14, height: 14, fill: color}}>
						<path d={icon!.path} />
					</svg>
				)
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
