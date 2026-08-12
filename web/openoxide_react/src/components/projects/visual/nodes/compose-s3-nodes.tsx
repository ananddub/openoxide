import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {Package2, HardDrive, ChevronDown, ChevronUp} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

const HANDLE_STYLE: React.CSSProperties = {
	width: 10, height: 10, borderRadius: '50%',
	border: '2px solid #30363d', background: '#0d1117',
	transition: 'border-color .15s',
};

// ─── Compose group ─────────────────────────────────────────────────────────

export type ComposeGroupNodeProps = Node<ComposeGroupData, 'composeGroupNode'>;
export interface ComposeGroupData extends Record<string, unknown> {
	id: number; name: string; status: string; servicesCount: number; expanded: boolean;
}

export function ComposeGroupNode({data, selected}: NodeProps<ComposeGroupNodeProps>) {
	const {inspect, toggleExpand} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const [hov, setHov] = useState(false);
	const count = data.servicesCount as number;

	return (
		<div
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: data.expanded ? Math.max(count * 160 + 20, 360) : 185,
				minHeight: data.expanded ? 140 : 'auto',
				background: '#161b22',
				border: selected
					? '1px solid rgba(167,139,250,0.55)'
					: data.expanded
					? '1px dashed rgba(167,139,250,0.35)'
					: hov
					? '1px solid rgba(255,255,255,0.16)'
					: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 10,
				userSelect: 'none',
				transition: 'border-color .12s, width .18s',
				position: 'relative',
			}}
		>
			{/* Status dot */}
			<span style={{
				position: 'absolute', top: 9, right: 30,
				width: 6, height: 6, borderRadius: '50%',
				background: running ? '#3fb950' : '#6e7681',
				boxShadow: running ? '0 0 6px #3fb950' : 'none',
			}} />

			{/* Header */}
			<div style={{display: 'flex', alignItems: 'center', gap: 10, padding: '10px 12px', cursor: 'pointer'}}
				onClick={() => inspect({id: data.id, type: 'compose', name: data.name, status: data.status})}>
				<div style={{
					width: 28, height: 28, borderRadius: 7,
					background: 'rgba(167,139,250,0.1)',
					display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
				}}>
					<Package2 size={14} color="#a78bfa" strokeWidth={1.8} />
				</div>
				<div style={{flex: 1, minWidth: 0}}>
					<p style={{margin: 0, fontSize: 12.5, fontWeight: 600, color: '#e6edf3',
						whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', paddingRight: 8}}>
						{data.name}
					</p>
					<p style={{margin: 0, fontSize: 10, color: '#6e7681', marginTop: 2}}>
						{count} service{count !== 1 ? 's' : ''}
					</p>
				</div>
				<button type="button"
					onClick={e => { e.stopPropagation(); toggleExpand(data.id as number); }}
					style={{padding: 2, border: 'none', background: 'transparent', cursor: 'pointer',
						color: '#6e7681', display: 'flex', alignItems: 'center'}}
					onMouseEnter={e => (e.currentTarget.style.color = '#a78bfa')}
					onMouseLeave={e => (e.currentTarget.style.color = '#6e7681')}>
					{data.expanded ? <ChevronUp size={13} /> : <ChevronDown size={13} />}
				</button>
			</div>

			<Handle type="target" position={Position.Left}
				style={{...HANDLE_STYLE, left: -6, borderColor: '#a78bfa'}} />
			<Handle type="source" position={Position.Right}
				style={{...HANDLE_STYLE, right: -6, borderColor: '#a78bfa'}} />
		</div>
	);
}

// ─── Compose mini service ─────────────────────────────────────────────────

export type ComposeServiceMiniNodeProps = Node<ComposeMiniData, 'composeMiniServiceNode'>;
export interface ComposeMiniData extends Record<string, unknown> {
	serviceName: string; image: string;
}

export function ComposeServiceMiniNode({data}: NodeProps<ComposeServiceMiniNodeProps>) {
	return (
		<div style={{
			width: 148, padding: '7px 10px',
			background: '#0d1117', border: '1px solid rgba(255,255,255,0.08)',
			borderRadius: 8, userSelect: 'none',
		}}>
			<p style={{margin: 0, fontSize: 11.5, fontWeight: 600, color: '#e6edf3',
				whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
				{data.serviceName}
			</p>
			<p style={{margin: 0, fontSize: 10, color: '#6e7681', marginTop: 1,
				whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
				{data.image}
			</p>
		</div>
	);
}

// ─── S3 / Storage node ─────────────────────────────────────────────────────

export type S3CanvasNodeProps = Node<S3NodeData, 's3Node'>;
export interface S3NodeData extends Record<string, unknown> {
	id: number; name: string; provider: string; bucket: string;
}

export function S3CanvasNode({data, selected}: NodeProps<S3CanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() => inspect({id: data.id, type: 'destination', name: data.name, status: 'active'})}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				display: 'flex', alignItems: 'center', gap: 10,
				width: 185, padding: '10px 12px',
				background: '#161b22',
				border: selected
					? '1px solid rgba(251,191,36,0.55)'
					: hov
					? '1px solid rgba(255,255,255,0.16)'
					: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 10,
				cursor: 'pointer', userSelect: 'none',
				transition: 'border-color .12s',
				position: 'relative',
			}}
		>
			<span style={{
				position: 'absolute', top: 7, right: 8,
				width: 6, height: 6, borderRadius: '50%',
				background: '#fbbf24', boxShadow: '0 0 5px #fbbf24',
			}} />
			<div style={{
				width: 28, height: 28, borderRadius: 7,
				background: 'rgba(251,191,36,0.1)',
				display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
			}}>
				<HardDrive size={14} color="#fbbf24" strokeWidth={1.8} />
			</div>
			<div style={{flex: 1, minWidth: 0}}>
				<p style={{margin: 0, fontSize: 12.5, fontWeight: 600, color: '#e6edf3',
					whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', paddingRight: 14}}>
					{data.name}
				</p>
				<p style={{margin: 0, fontSize: 10, color: '#6e7681', marginTop: 2}}>
					{data.provider}
				</p>
			</div>
			<Handle type="target" position={Position.Left}
				style={{...HANDLE_STYLE, left: -6}} />
		</div>
	);
}
