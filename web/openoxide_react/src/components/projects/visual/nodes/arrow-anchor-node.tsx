import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';

export type ArrowAnchorNodeProps = Node<
	Record<string, unknown>,
	'arrowAnchor'
>;

export function ArrowAnchorNode({
	selected,
}: NodeProps<ArrowAnchorNodeProps>) {
	return (
		<div
			style={{
				width: 14,
				height: 14,
				position: 'relative',
				display: 'flex',
				alignItems: 'center',
				justifyContent: 'center',
			}}>
			<div
				style={{
					width: 10,
					height: 10,
					borderRadius: '50%',
					background: selected ? '#818cf8' : 'rgba(99,102,241,0.5)',
					border: '1.5px solid rgba(0,0,0,0.5)',
					boxShadow: selected
						? '0 0 10px #818cf8'
						: '0 0 4px rgba(99,102,241,0.4)',
					cursor: 'grab',
					transition: 'all 0.15s',
				}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{
					opacity: 0,
					width: 0,
					height: 0,
					minWidth: 0,
					minHeight: 0,
				}}
			/>
			<Handle
				type="target"
				position={Position.Left}
				style={{
					opacity: 0,
					width: 0,
					height: 0,
					minWidth: 0,
					minHeight: 0,
				}}
			/>
		</div>
	);
}
