import {type NodeProps, type Node} from '@xyflow/react';
import {Server} from 'lucide-react';

export type ComposeMiniServiceNodeProps = Node<
	ComposeMiniServiceData,
	'composeMiniServiceNode'
>;
export interface ComposeMiniServiceData extends Record<string, unknown> {
	serviceName: string;
	image?: string;
}

export function ComposeServiceMiniNode({
	data,
}: NodeProps<ComposeMiniServiceNodeProps>) {
	return (
		<div
			className="flex w-36 items-center gap-2 rounded-xl p-2 select-none"
			style={{
				background: 'rgba(255,255,255,0.06)',
				border: '1px solid rgba(139,92,246,0.2)',
			}}>
			<Server
				className="h-3 w-3 shrink-0"
				style={{color: '#a78bfa'}}
				strokeWidth={1.5}
			/>
			<div className="min-w-0">
				<p className="truncate text-[11px] leading-tight font-semibold text-white">
					{data.serviceName}
				</p>
				<p
					className="truncate font-mono text-[9px]"
					style={{color: 'rgba(255,255,255,0.35)'}}>
					{data.image}
				</p>
			</div>
		</div>
	);
}
