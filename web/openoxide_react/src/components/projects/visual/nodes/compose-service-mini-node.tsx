import {type NodeProps, type Node} from '@xyflow/react';

export type ComposeServiceMiniNodeProps = Node<
	ComposeMiniData,
	'composeMiniServiceNode'
>;
export interface ComposeMiniData extends Record<string, unknown> {
	serviceName: string;
	image: string;
}

export function ComposeServiceMiniNode({
	data,
}: NodeProps<ComposeServiceMiniNodeProps>) {
	return (
		<div
			style={{
				width: 148,
				padding: '7px 10px',
				background: '#0d1117',
				border: '1px solid rgba(255,255,255,0.08)',
				borderRadius: 8,
				userSelect: 'none',
			}}>
			<p
				style={{
					margin: 0,
					fontSize: 11.5,
					fontWeight: 600,
					color: '#e6edf3',
					whiteSpace: 'nowrap',
					overflow: 'hidden',
					textOverflow: 'ellipsis',
				}}>
				{data.serviceName}
			</p>
			<p
				style={{
					margin: 0,
					fontSize: 10,
					color: '#6e7681',
					marginTop: 1,
					whiteSpace: 'nowrap',
					overflow: 'hidden',
					textOverflow: 'ellipsis',
				}}>
				{data.image}
			</p>
		</div>
	);
}
