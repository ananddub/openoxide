export function ComposeEdge({
	sourceX, sourceY, targetX, targetY, selected, id,
}: any) {
	const mx   = (sourceX + targetX) / 2;
	const path = `M${sourceX} ${sourceY} C${mx} ${sourceY} ${mx} ${targetY} ${targetX} ${targetY}`;

	const stroke = selected
		? '#58a6ff'
		: '#30363d';
	const opacity = selected ? 0.9 : 0.7;

	return (
		<>
			{/* Wide invisible hit area */}
			<path d={path} fill="none" stroke="transparent" strokeWidth={14}
				className="react-flow__edge-interaction" />
			<defs>
				<marker
					id={`ce-mk-${id}`}
					markerWidth="7" markerHeight="7"
					refX="5" refY="3.5"
					orient="auto" markerUnits="userSpaceOnUse"
				>
					<path d="M0,0.5 L5.5,3.5 L0,6.5 L1,3.5 Z" fill={stroke} opacity={opacity} />
				</marker>
			</defs>
			<path
				d={path}
				fill="none"
				stroke={stroke}
				strokeWidth={1.5}
				strokeDasharray="5 4"
				opacity={opacity}
				markerEnd={`url(#ce-mk-${id})`}
				style={{transition: 'stroke .12s, opacity .12s'}}
			/>
		</>
	);
}
