import {
	useCallback,
	useState,
	type Dispatch,
	type SetStateAction,
} from 'react';
import {useReactFlow, type Edge, type Node} from '@xyflow/react';

export type ArrowDragState = {
	ss: {x: number; y: number}; // screen start
	sf: {x: number; y: number}; // flow start
	cs: {x: number; y: number}; // screen current
};

/** Free-drawn arrow connections: pointer drag creates two anchors + an edge. */
export function useArrowDrag(
	mode: 'select' | 'arrow',
	canvasRef: React.RefObject<HTMLDivElement | null>,
	setNodes: Dispatch<SetStateAction<Node[]>>,
	setEdges: Dispatch<SetStateAction<Edge[]>>,
) {
	const {screenToFlowPosition} = useReactFlow();
	const [arrowDrag, setArrowDrag] = useState<ArrowDragState | null>(null);

	const localPoint = (e: React.MouseEvent) => {
		const rect = canvasRef.current?.getBoundingClientRect() ?? {
			left: 0,
			top: 0,
		};
		return {x: e.clientX - rect.left, y: e.clientY - rect.top};
	};

	const onMouseDown = useCallback(
		(e: React.MouseEvent) => {
			if (mode !== 'arrow') return;
			if ((e.target as HTMLElement).closest('.react-flow__node')) return;
			e.preventDefault();
			const ss = localPoint(e);
			const sf = screenToFlowPosition({x: e.clientX, y: e.clientY});
			setArrowDrag({ss, sf, cs: ss});
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[mode, screenToFlowPosition],
	);

	const onMouseMove = useCallback(
		(e: React.MouseEvent) => {
			setArrowDrag(prev => (prev ? {...prev, cs: localPoint(e)} : null));
		},
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[],
	);

	const onMouseUp = useCallback(
		(e: React.MouseEvent) => {
			setArrowDrag(prev => {
				if (!prev) return null;
				const endFlow = screenToFlowPosition({x: e.clientX, y: e.clientY});
				const dx = endFlow.x - prev.sf.x;
				const dy = endFlow.y - prev.sf.y;
				if (Math.sqrt(dx * dx + dy * dy) < 10) return null;
				const ts = Date.now();
				const sid = `anc-${ts}-s`;
				const eid = `anc-${ts}-e`;
				setNodes(p => [
					...p,
					{
						id: sid,
						type: 'arrowAnchor' as any,
						position: prev.sf,
						data: {},
					},
					{
						id: eid,
						type: 'arrowAnchor' as any,
						position: endFlow,
						data: {},
					},
				]);
				setEdges(p => [
					...p,
					{
						id: `arrow-${ts}`,
						source: sid,
						target: eid,
						type: 'customEdge',
						animated: false,
						data: {},
					},
				]);
				return null;
			});
		},
		[screenToFlowPosition, setNodes, setEdges],
	);

	return {arrowDrag, setArrowDrag, onMouseDown, onMouseMove, onMouseUp};
}
