import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {HardDrive} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

export type S3CanvasNodeProps = Node<S3NodeData, 's3Node'>;
export interface S3NodeData extends Record<string, unknown> {
	id: number; name: string; provider: string; bucket: string;
}

export function S3CanvasNode({data}: NodeProps<S3CanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() => inspect({id: data.id, type: 'destination', name: data.name, status: 'active'})}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: 220,
				background: '#1a1a2e',
				border: hov ? '1px solid rgba(251,191,36,0.45)' : '1px solid rgba(255,255,255,0.08)',
				borderRadius: 12,
				padding: '10px 14px',
				cursor: 'pointer',
				userSelect: 'none',
				transition: 'border-color .15s, box-shadow .15s',
				boxShadow: hov ? '0 0 0 3px rgba(251,191,36,0.08)' : 'none',
			}}
		>
			<div style={{display: 'flex', alignItems: 'center', gap: 10}}>
				<div style={{
					width: 32, height: 32, borderRadius: 8,
					background: 'rgba(251,191,36,0.1)',
					border: '1px solid rgba(251,191,36,0.25)',
					display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
				}}>
					<HardDrive size={15} color="#fbbf24" strokeWidth={1.6} />
				</div>

				<div style={{flex: 1, minWidth: 0}}>
					<p style={{margin: 0, fontSize: 13, fontWeight: 600, color: '#f1f5f9', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
						{data.name}
					</p>
					<p style={{margin: 0, fontSize: 10, color: 'rgba(255,255,255,0.35)', marginTop: 2}}>
						{data.provider} · {data.bucket}
					</p>
				</div>

				<span style={{width: 7, height: 7, borderRadius: '50%', background: '#fbbf24', boxShadow: '0 0 8px #fbbf24', flexShrink: 0}} />
			</div>

			<Handle type="target" position={Position.Left}
				style={{left: -7, width: 12, height: 12, background: '#1a1a2e', border: '2px solid #fbbf24', borderRadius: '50%'}} />
		</div>
	);
}
