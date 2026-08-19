import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {Package2, ChevronDown, ChevronUp} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

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
}: NodeProps<ComposeGroupNodeProps>) {
	const {inspect, toggleExpand} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const count = data.servicesCount as number;
	const [hov, setHov] = useState(false);

	return (
		<div
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: data.expanded ? Math.max(count * 158 + 24, 380) : 220,
				minHeight: data.expanded ? 160 : 'auto',
				background: '#1a1a2e',
				border: data.expanded
					? '1px dashed rgba(167,139,250,0.4)'
					: hov
						? '1px solid rgba(167,139,250,0.45)'
						: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 12,
				transition: 'border-color .15s, box-shadow .15s, width .2s',
				boxShadow:
					hov && !data.expanded
						? '0 0 0 3px rgba(167,139,250,0.08)'
						: 'none',
				userSelect: 'none',
			}}>
			{/* Header */}
			<div
				style={{
					display: 'flex',
					alignItems: 'center',
					gap: 10,
					padding: '10px 14px',
				}}>
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 10,
						flex: 1,
						minWidth: 0,
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
							width: 32,
							height: 32,
							borderRadius: 8,
							background: 'rgba(167,139,250,0.12)',
							border: '1px solid rgba(167,139,250,0.25)',
							display: 'flex',
							alignItems: 'center',
							justifyContent: 'center',
							flexShrink: 0,
						}}>
						<Package2 size={15} color="#a78bfa" strokeWidth={1.6} />
					</div>
					<div style={{flex: 1, minWidth: 0}}>
						<div style={{display: 'flex', alignItems: 'center', gap: 6}}>
							<p
								style={{
									margin: 0,
									fontSize: 13,
									fontWeight: 600,
									color: '#f1f5f9',
									whiteSpace: 'nowrap',
									overflow: 'hidden',
									textOverflow: 'ellipsis',
								}}>
								{data.name}
							</p>
							<span
								style={{
									width: 6,
									height: 6,
									borderRadius: '50%',
									flexShrink: 0,
									background: running ? '#34d399' : '#52525b',
									boxShadow: running ? '0 0 7px #34d399' : 'none',
								}}
							/>
						</div>
						<p
							style={{
								margin: 0,
								fontSize: 10,
								color: 'rgba(255,255,255,0.35)',
								marginTop: 2,
							}}>
							{count} service{count !== 1 ? 's' : ''}
						</p>
					</div>
				</div>
				<button
					type="button"
					onClick={e => {
						e.stopPropagation();
						toggleExpand(data.id as number);
					}}
					style={{
						padding: 4,
						borderRadius: 6,
						border: 'none',
						background: 'transparent',
						cursor: 'pointer',
						color: 'rgba(255,255,255,0.4)',
						display: 'flex',
						alignItems: 'center',
					}}
					onMouseEnter={e => (e.currentTarget.style.color = '#a78bfa')}
					onMouseLeave={e =>
						(e.currentTarget.style.color = 'rgba(255,255,255,0.4)')
					}>
					{data.expanded ? (
						<ChevronUp size={14} />
					) : (
						<ChevronDown size={14} />
					)}
				</button>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				style={{
					left: -7,
					width: 12,
					height: 12,
					background: '#1a1a2e',
					border: '2px solid #a78bfa',
					borderRadius: '50%',
				}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{
					right: -7,
					width: 12,
					height: 12,
					background: '#1a1a2e',
					border: '2px solid #a78bfa',
					borderRadius: '50%',
				}}
			/>
		</div>
	);
}
