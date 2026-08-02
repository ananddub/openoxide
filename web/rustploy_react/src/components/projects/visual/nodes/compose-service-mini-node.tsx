import {type NodeProps, type Node} from '@xyflow/react';
import {Server} from 'lucide-react';

export type ComposeMiniServiceNodeProps = Node<ComposeMiniServiceData, 'composeMiniServiceNode'>;
export interface ComposeMiniServiceData extends Record<string, unknown> {
	serviceName: string; image?: string;
}

export function ComposeServiceMiniNode({data}: NodeProps<ComposeMiniServiceNodeProps>) {
	return (
		<div
			className="w-36 select-none rounded-xl p-2 flex items-center gap-2"
			style={{
				background: 'rgba(255,255,255,0.06)',
				border: '1px solid rgba(139,92,246,0.2)',
			}}
		>
			<Server className="w-3 h-3 shrink-0" style={{color: '#a78bfa'}} strokeWidth={1.5} />
			<div className="min-w-0">
				<p className="text-[11px] font-semibold text-white truncate leading-tight">{data.serviceName}</p>
				<p className="text-[9px] font-mono truncate" style={{color: 'rgba(255,255,255,0.35)'}}>{data.image}</p>
			</div>
		</div>
	);
}
