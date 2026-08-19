import {useRef, useCallback, useId, useState} from 'react';
import {
	useReactFlow,
	type EdgeProps,
	EdgeLabelRenderer,
} from '@xyflow/react';

export type CustomEdgeData = {
	cp?: {x: number; y: number};
	color?: string;
};

const DEFAULT = '#6e7681'; // Railway's edge color — muted gray

export function CustomEdge({
	id,
	sourceX,
	sourceY,
	targetX,
	targetY,
	data,
	selected,
}: EdgeProps) {
	const {setEdges, screenToFlowPosition} = useReactFlow();
	const uid = useId().replace(/:/g, '');

	const color = (data as CustomEdgeData)?.color ?? DEFAULT;
	const cp = (data as CustomEdgeData)?.cp;
	const midX = (sourceX + targetX) / 2;
	const midY = (sourceY + targetY) / 2;
	const cpX = cp ? midX + cp.x : midX;
	const cpY = cp ? midY + cp.y : midY;

	const path = `M${sourceX} ${sourceY} Q${cpX} ${cpY} ${targetX} ${targetY}`;

	const [hov, setHov] = useState(false);
	const dragging = useRef(false);

	const onHandleDown = useCallback(
		(e: React.MouseEvent) => {
			e.stopPropagation();
			e.preventDefault();
			dragging.current = true;

			const move = (ev: MouseEvent) => {
				if (!dragging.current) return;
				const fp = screenToFlowPosition({x: ev.clientX, y: ev.clientY});
				setEdges(eds =>
					eds.map(edge =>
						edge.id === id
							? {
									...edge,
									data: {
										...(edge.data ?? {}),
										cp: {x: fp.x - midX, y: fp.y - midY},
									},
								}
							: edge,
					),
				);
			};
			const up = () => {
				dragging.current = false;
				window.removeEventListener('mousemove', move);
				window.removeEventListener('mouseup', up);
			};
			window.addEventListener('mousemove', move);
			window.addEventListener('mouseup', up);
		},
		[id, midX, midY, setEdges, screenToFlowPosition],
	);

	const markerId = `mk-${uid}`;
	const activeColor = selected ? '#58a6ff' : hov ? '#8b949e' : color;

	return (
		<>
			<defs>
				{/* Clean small arrowhead — Railway style */}
				<marker
					id={markerId}
					markerWidth="8"
					markerHeight="8"
					refX="6"
					refY="4"
					orient="auto"
					markerUnits="userSpaceOnUse">
					<path
						d="M0,0.5 L6.5,4 L0,7.5 L1.5,4 Z"
						fill={activeColor}
						opacity="0.9"
					/>
				</marker>
			</defs>

			{/* Wide invisible hit area */}
			<path
				d={path}
				fill="none"
				stroke="transparent"
				strokeWidth={18}
				className="react-flow__edge-interaction"
				style={{cursor: 'pointer'}}
				onMouseEnter={() => setHov(true)}
				onMouseLeave={() => setHov(false)}
			/>

			{/* Main edge — thin, Railway style */}
			<path
				d={path}
				fill="none"
				stroke={activeColor}
				strokeWidth={selected ? 1.8 : 1.4}
				strokeLinecap="round"
				markerEnd={`url(#${markerId})`}
				opacity={selected ? 1 : hov ? 0.85 : 0.55}
				style={{transition: 'opacity .15s, stroke .15s'}}
				onMouseEnter={() => setHov(true)}
				onMouseLeave={() => setHov(false)}
			/>

			{/* Bend handle — visible when selected or hovering edge */}
			{(selected || hov) && (
				<EdgeLabelRenderer>
					<div
						className="nodrag nopan"
						style={{
							position: 'absolute',
							transform: `translate(-50%,-50%) translate(${cpX}px,${cpY}px)`,
							pointerEvents: 'all',
							zIndex: 10,
						}}>
						<div
							onMouseDown={onHandleDown as any}
							title="Drag to bend"
							style={{
								width: 10,
								height: 10,
								borderRadius: '50%',
								background: '#0d1117',
								border: `2px solid ${selected ? '#58a6ff' : '#8b949e'}`,
								cursor: 'crosshair',
								transition: 'transform .1s, border-color .1s',
							}}
							onMouseEnter={e => {
								e.currentTarget.style.transform = 'scale(1.5)';
							}}
							onMouseLeave={e => {
								e.currentTarget.style.transform = 'scale(1)';
							}}
						/>
					</div>
				</EdgeLabelRenderer>
			)}
		</>
	);
}
