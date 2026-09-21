import {useEffect, useRef} from 'react';
import {
	useNodesState,
	useEdgesState,
	type Edge,
	type Node,
} from '@xyflow/react';
import {buildNodes} from './railway-build-nodes';

/** Nodes/edges state + signature-guarded rebuild preserving dragged positions. */
export function useRailwayNodes(
	applications: any[],
	databases: any[],
	composeStacks: any[],
	destinations: any[],
	expandedGroups: Record<number, boolean>,
) {
	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
	const sigRef = useRef('');

	useEffect(() => {
		const next = buildNodes(
			applications,
			databases,
			composeStacks,
			destinations,
			expandedGroups,
		);
		const sig = next
			.map(n => `${n.id}:${n.hidden}:${JSON.stringify(n.data)}`)
			.join('|');
		if (sig === sigRef.current) return;
		sigRef.current = sig;
		setNodes(prev => {
			const posMap = new Map(prev.map(n => [n.id, n.position]));
			const merged = next.map(n => {
				const existingPos = posMap.get(n.id);
				return existingPos ? {...n, position: existingPos} : n;
			});
			const anchors = prev.filter(n => n.type === 'arrowAnchor');
			return [...merged, ...anchors];
		});
	}, [
		applications,
		databases,
		composeStacks,
		destinations,
		expandedGroups,
	]);

	return {
		nodes,
		edges,
		setNodes,
		setEdges,
		onNodesChange,
		onEdgesChange,
	};
}
