// ─── Stack dashed dependency edge ─────────────────────────────────────────────

export function StackEdge({
	sourceX,
	sourceY,
	targetX,
	targetY,
	selected,
	id,
}: any) {
	const mx = (sourceX + targetX) / 2;
	const path = `M${sourceX} ${sourceY} C${mx} ${sourceY} ${mx} ${targetY} ${targetX} ${targetY}`;
	const col = selected ? '#58a6ff' : '#30363d';
	return (
		<>
			<path
				d={path}
				fill="none"
				stroke="transparent"
				strokeWidth={14}
				className="react-flow__edge-interaction"
			/>
			<defs>
				<marker
					id={`sk-mk-${id}`}
					markerWidth="7"
					markerHeight="7"
					refX="5"
					refY="3.5"
					orient="auto"
					markerUnits="userSpaceOnUse">
					<path
						d="M0,0.5 L5.5,3.5 L0,6.5 L1,3.5 Z"
						fill={col}
						opacity="0.7"
					/>
				</marker>
			</defs>
			<path
				d={path}
				fill="none"
				stroke={col}
				strokeWidth={1.5}
				strokeDasharray="6 4"
				opacity={selected ? 0.9 : 0.45}
				markerEnd={`url(#sk-mk-${id})`}
				style={{transition: 'stroke .15s, opacity .15s'}}
			/>
		</>
	);
}
