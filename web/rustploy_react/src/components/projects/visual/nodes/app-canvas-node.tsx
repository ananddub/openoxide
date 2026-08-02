import {useState} from 'react';
import {Handle, Position, type NodeProps, type Node} from '@xyflow/react';
import {Globe, ExternalLink} from 'lucide-react';
import {useCanvasActions} from '../canvas-context';

export type AppCanvasNodeProps = Node<AppNodeData, 'appNode'>;
export interface AppNodeData extends Record<string, unknown> {
	id: number; name: string; status: string; framework?: string; url?: string;
}

const HANDLE = '!w-3 !h-3 !rounded-full !border-2 !border-blue-500 !bg-[#1a1a2e] hover:!bg-blue-500 transition-colors';

export function AppCanvasNode({data}: NodeProps<AppCanvasNodeProps>) {
	const {inspect} = useCanvasActions();
	const running = data.status === 'RUNNING' || data.status === 'running';
	const [hov, setHov] = useState(false);

	return (
		<div
			onClick={() => inspect({id: data.id, type: 'app', name: data.name, status: data.status})}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: 220,
				background: '#1a1a2e',
				border: hov ? '1px solid rgba(99,102,241,0.5)' : '1px solid rgba(255,255,255,0.08)',
				borderRadius: 12,
				padding: '10px 14px',
				cursor: 'pointer',
				userSelect: 'none',
				transition: 'border-color .15s, box-shadow .15s',
				boxShadow: hov ? '0 0 0 3px rgba(99,102,241,0.1)' : 'none',
			}}
		>
			<div style={{display: 'flex', alignItems: 'center', gap: 10}}>
				{/* Icon badge */}
				<div style={{
					width: 32, height: 32, borderRadius: 8,
					background: 'rgba(96,165,250,0.12)',
					border: '1px solid rgba(96,165,250,0.2)',
					display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
				}}>
					<Globe size={15} color="#60a5fa" strokeWidth={1.6} />
				</div>

				{/* Text */}
				<div style={{flex: 1, minWidth: 0}}>
					<p style={{margin: 0, fontSize: 13, fontWeight: 600, color: '#f1f5f9', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
						{data.name}
					</p>
					<p style={{margin: 0, fontSize: 10, color: 'rgba(255,255,255,0.35)', marginTop: 2}}>
						{data.framework || 'Application'}
					</p>
				</div>

				{/* Status */}
				<span style={{
					width: 7, height: 7, borderRadius: '50%', flexShrink: 0,
					background: running ? '#34d399' : '#52525b',
					boxShadow: running ? '0 0 8px #34d399' : 'none',
				}} />
			</div>

			{data.url && (
				<a
					href={data.url as string} target="_blank" rel="noreferrer"
					onClick={e => e.stopPropagation()}
					style={{display: 'flex', alignItems: 'center', gap: 4, marginTop: 8, fontSize: 10, color: 'rgba(96,165,250,0.7)', textDecoration: 'none'}}
				>
					Open <ExternalLink size={9} />
				</a>
			)}

			<Handle type="target" position={Position.Left} className={HANDLE} style={{left: -7}} />
			<Handle type="source" position={Position.Right} className={HANDLE} style={{right: -7}} />
		</div>
	);
}
