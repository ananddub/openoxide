import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {
	siPostgresql, siMysql, siRedis, siMongodb,
	siMariadb, siSqlite, siClickhouse, siApachecassandra,
} from 'simple-icons';
import {useCanvasActions} from '../canvas-context';

export type DbCanvasNodeProps = Node<DbNodeData, 'dbNode'>;
export interface DbNodeData extends Record<string, unknown> {
	id: number; name: string; dbType: string; status: string; port?: number;
}

function getDbMeta(dbType: string) {
	const t = (dbType ?? '').toLowerCase();
	if (t.includes('postgres') || t === 'pg') return {icon: siPostgresql, color: '#4169E1'};
	if (t.includes('mysql'))                   return {icon: siMysql,      color: '#4479A1'};
	if (t.includes('redis'))                   return {icon: siRedis,      color: '#FF4438'};
	if (t.includes('mongo'))                   return {icon: siMongodb,    color: '#47A248'};
	if (t.includes('maria'))                   return {icon: siMariadb,    color: '#7b8cde'};
	if (t.includes('sqlite'))                  return {icon: siSqlite,     color: '#5b9bd5'};
	if (t.includes('click'))                   return {icon: siClickhouse, color: '#FFCC01'};
	if (t.includes('cassandra'))               return {icon: siApachecassandra, color: '#1287B1'};
	return                                            {icon: siPostgresql, color: '#4169E1'};
}

export function DbCanvasNode({data}: NodeProps<DbCanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const {icon, color} = getDbMeta(data.dbType as string);
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() => inspect({id: data.id, type: 'database', name: data.name, status: data.status, dbType: data.dbType})}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: 220,
				background: '#1a1a2e',
				border: hov ? `1px solid ${color}60` : '1px solid rgba(255,255,255,0.08)',
				borderRadius: 12,
				padding: '10px 14px',
				cursor: 'pointer',
				userSelect: 'none',
				transition: 'border-color .15s, box-shadow .15s',
				boxShadow: hov ? `0 0 0 3px ${color}15` : 'none',
			}}
		>
			<div style={{display: 'flex', alignItems: 'center', gap: 10}}>
				{/* Real brand icon badge */}
				<div style={{
					width: 32, height: 32, borderRadius: 8,
					background: `${color}18`,
					border: `1px solid ${color}30`,
					display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
				}}>
					<svg role="img" viewBox="0 0 24 24" style={{width: 15, height: 15, fill: color}}>
						<path d={icon.path} />
					</svg>
				</div>

				<div style={{flex: 1, minWidth: 0}}>
					<p style={{margin: 0, fontSize: 13, fontWeight: 600, color: '#f1f5f9', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
						{data.name}
					</p>
					<p style={{margin: 0, fontSize: 10, color: 'rgba(255,255,255,0.35)', marginTop: 2, fontFamily: 'monospace', textTransform: 'uppercase', letterSpacing: 1}}>
						{data.dbType}{data.port ? ` · ${data.port}` : ''}
					</p>
				</div>

				<span style={{
					width: 7, height: 7, borderRadius: '50%', flexShrink: 0,
					background: running ? '#34d399' : '#52525b',
					boxShadow: running ? '0 0 8px #34d399' : 'none',
				}} />
			</div>

			<Handle type="target" position={Position.Left}
				style={{left: -7, width: 12, height: 12, background: '#1a1a2e', border: `2px solid ${color}`, borderRadius: '50%'}} />
			<Handle type="source" position={Position.Right}
				style={{right: -7, width: 12, height: 12, background: '#1a1a2e', border: `2px solid ${color}`, borderRadius: '50%'}} />
		</div>
	);
}
