import dagre from '@dagrejs/dagre';
import type {Node, Edge} from '@xyflow/react';

/** Run a dagre auto-layout over raw ReactFlow nodes/edges. */
export function getDagreLayout(
	rawNodes: Node[],
	rawEdges: Edge[],
	direction = 'LR',
): {nodes: Node[]; edges: Edge[]} {
	if (rawNodes.length === 0) return {nodes: [], edges: []};

	const g = new dagre.graphlib.Graph();
	g.setGraph({
		rankdir: direction,
		nodesep: 65,
		ranksep: 160,
		marginx: 50,
		marginy: 50,
	});
	g.setDefaultEdgeLabel(() => ({}));

	const nodeW = 260;
	const nodeH = 150;

	rawNodes.forEach(n => g.setNode(n.id, {width: nodeW, height: nodeH}));
	rawEdges.forEach(e => g.setEdge(e.source, e.target));
	dagre.layout(g);

	const layoutedNodes = rawNodes.map(n => {
		const pos = g.node(n.id);
		return {
			...n,
			position: {
				x: (pos?.x ?? 0) - nodeW / 2,
				y: (pos?.y ?? 0) - nodeH / 2,
			},
		};
	});

	return {nodes: layoutedNodes, edges: rawEdges};
}
