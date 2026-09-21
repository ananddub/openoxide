import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {HardDrive} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

const HANDLE_STYLE: React.CSSProperties = {
	width: 10,
	height: 10,
	borderRadius: '50%',
	border: '2px solid #30363d',
	background: '#0d1117',
	transition: 'border-color .15s',
};

export type S3CanvasNodeProps = Node<S3NodeData, 's3Node'>;
export interface S3NodeData extends Record<string, unknown> {
	id: number;
	name: string;
	provider: string;
	bucket: string;
}

export function S3CanvasNode({
	data,
	selected,
}: NodeProps<S3CanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() =>
				inspect({
					id: data.id,
					type: 'destination',
					name: data.name,
					status: 'active',
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
					? '1px solid rgba(251,191,36,0.55)'
					: hov
						? '1px solid rgba(255,255,255,0.16)'
						: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 10,
				cursor: 'pointer',
				userSelect: 'none',
				transition: 'border-color .12s',
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
					background: '#fbbf24',
					boxShadow: '0 0 5px #fbbf24',
				}}
			/>
			<div
				style={{
					width: 28,
					height: 28,
					borderRadius: 7,
					background: 'rgba(251,191,36,0.1)',
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					flexShrink: 0,
				}}>
				<HardDrive size={14} color="#fbbf24" strokeWidth={1.8} />
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
					{data.provider}
				</p>
			</div>
			<Handle
				type="target"
				position={Position.Left}
				style={{...HANDLE_STYLE, left: -6}}
			/>
		</div>
	);
}
