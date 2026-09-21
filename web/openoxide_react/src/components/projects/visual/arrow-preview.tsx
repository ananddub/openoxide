import type {ArrowDragState} from './use-arrow-drag';

/** Dashed preview line while dragging a free arrow. */
export function ArrowPreview({drag}: {drag: ArrowDragState}) {
	return (
		<svg
			className="pointer-events-none absolute inset-0 z-20"
			style={{width: '100%', height: '100%'}}>
			<defs>
				<marker
					id="prev-mk"
					markerWidth="8"
					markerHeight="8"
					refX="6"
					refY="4"
					orient="auto"
					markerUnits="userSpaceOnUse">
					<path
						d="M0,0.5 L6.5,4 L0,7.5 L1.5,4 Z"
						fill="#58a6ff"
						opacity="0.85"
					/>
				</marker>
			</defs>
			<circle
				cx={drag.ss.x}
				cy={drag.ss.y}
				r={3.5}
				fill="#58a6ff"
				opacity="0.8"
			/>
			<line
				x1={drag.ss.x}
				y1={drag.ss.y}
				x2={drag.cs.x}
				y2={drag.cs.y}
				stroke="#58a6ff"
				strokeWidth={1.5}
				strokeDasharray="5 4"
				opacity="0.7"
				markerEnd="url(#prev-mk)"
			/>
		</svg>
	);
}
