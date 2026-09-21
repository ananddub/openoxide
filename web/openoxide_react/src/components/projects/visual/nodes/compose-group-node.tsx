import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {Package2, ChevronDown, ChevronUp} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

const HANDLE_STYLE: React.CSSProperties = {
	width: 10,
	height: 10,
	borderRadius: '50%',
	border: '2px solid #30363d',
	background: '#0d1117',
	transition: 'border-color .15s',
};

export type ComposeGroupNodeProps = Node<
	ComposeGroupData,
	'composeGroupNode'
>;
export interface ComposeGroupData extends Record<string, unknown> {
	id: number;
	name: string;
	status: string;
	servicesCount: number;
	expanded: boolean;
}

export function ComposeGroupNode({
	data,
	selected,
}: NodeProps<ComposeGroupNodeProps>) {
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
			}}>
			{/* Status dot */}
			<span
				style={{
					position: 'absolute',
					top: 9,
					right: 30,
					width: 6,
					height: 6,
					borderRadius: '50%',
					background: running ? '#3fb950' : '#6e7681',
					boxShadow: running ? '0 0 6px #3fb950' : 'none',
				}}
			/>

			{/* Header */}
			<div
				style={{
					display: 'flex',
					alignItems: 'center',
					gap: 10,
					padding: '10px 12px',
					cursor: 'pointer',
				}}
				onClick={() =>
					inspect({
						id: data.id,
						type: 'compose',
						name: data.name,
						status: data.status,
					})
				}>
				<div
					style={{
						width: 28,
						height: 28,
						borderRadius: 7,
						background: 'rgba(167,139,250,0.1)',
						display: 'flex',
						alignItems: 'center',
						justifyContent: 'center',
						flexShrink: 0,
					}}>
					<Package2 size={14} color="#a78bfa" strokeWidth={1.8} />
				</div>
				<div style={{flex: 1, minWidth: 0}}>
					<p
						style={{
							margin: 0,
							fontSize: 12.5,
							fontWeight: 600,
							color: '#e6edf3',
							whiteSpace: 'nowrap',
							overflow: 'hidden',
							textOverflow: 'ellipsis',
							paddingRight: 8,
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
						{count} service{count !== 1 ? 's' : ''}
					</p>
				</div>
				<button
					type="button"
					onClick={e => {
						e.stopPropagation();
						toggleExpand(data.id as number);
					}}
					style={{
						padding: 2,
						border: 'none',
						background: 'transparent',
						cursor: 'pointer',
						color: '#6e7681',
						display: 'flex',
						alignItems: 'center',
					}}
					onMouseEnter={e => (e.currentTarget.style.color = '#a78bfa')}
					onMouseLeave={e => (e.currentTarget.style.color = '#6e7681')}>
					{data.expanded ? (
						<ChevronUp size={13} />
					) : (
						<ChevronDown size={13} />
					)}
				</button>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				style={{...HANDLE_STYLE, left: -6, borderColor: '#a78bfa'}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{...HANDLE_STYLE, right: -6, borderColor: '#a78bfa'}}
			/>
		</div>
	);
}
