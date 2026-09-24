import {useCallback, useEffect, useRef} from 'react';
import {
	useNodesState,
	useEdgesState,
	type Edge,
	type Node,
} from '@xyflow/react';
import type {StackServiceData} from './stack-service-card';
import type {Template} from './stack-templates';

const COL_W = 340;
const ROW_H = 200;

/** Nodes/edges state for the stack canvas + auto-wire + CRUD actions. */
export function useStackCanvasGraph() {
	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
	const counter = useRef(0);

	// Auto-layout: arrange in columns of 3
	const nextPos = (total: number): {x: number; y: number} => {
		const col = Math.floor(total / 3);
		const row = total % 3;
		return {x: 80 + col * COL_W, y: 60 + row * ROW_H};
	};

	const addService = useCallback(
		(t: Template, name: string) => {
			const id = `svc-${++counter.current}`;
			const node: Node = {
				id,
				type: 'stackService',
				position: nextPos(nodes.length),
				data: {
					name,
					image: t.data.image ?? t.label,
					replicas: 1,
					envVars: t.data.envVars ?? [],
					volumes: t.data.volumes ?? [],
					ports: [],
					isNew: true,
				} as StackServiceData,
			};
			setNodes(prev => [...prev, node]);
		},
		[nodes.length, setNodes],
	);

	// ── Dependency auto-wire ────────────────────────────────────────────────── Parses  ${{ServiceName.VARIABLE}}  in env var values. Triggers on blur of any env var input (via onUpdate → setNodes → nodes change).
	useEffect(() => {
		if (nodes.length < 2) {
			setEdges(es => es.filter(e => !e.id.startsWith('dep-')));
			return;
		}

		// name (lowercase) → node id
		const nameToId = new Map<string, string>();
		nodes.forEach(n =>
			nameToId.set((n.data as StackServiceData).name.toLowerCase(), n.id),
		);

		const depEdges: Edge[] = [];

		nodes.forEach(targetNode => {
			const targetData = targetNode.data as StackServiceData;
			(targetData.envVars ?? []).forEach(({value}) => {
				if (!value) return;
				// Fresh regex per value — avoids stale lastIndex bug
				const re = /\$\{\{([A-Za-z][A-Za-z0-9_-]*)\.([^}]*)\}\}/g;
				let m: RegExpExecArray | null;
				while ((m = re.exec(value)) !== null) {
					const sourceId = nameToId.get(m[1].toLowerCase());
					if (!sourceId || sourceId === targetNode.id) continue;
					const eid = `dep-${sourceId}-${targetNode.id}`;
					if (!depEdges.find(e => e.id === eid))
						depEdges.push({
							id: eid,
							source: sourceId,
							target: targetNode.id,
							type: 'stackEdge',
						});
				}
			});
		});

		setEdges(es => [
			...es.filter(e => !e.id.startsWith('dep-')),
			...depEdges,
		]);
	}, [nodes, setEdges]);

	const updateNode = useCallback(
		(nodeId: string, data: Partial<StackServiceData>) => {
			setNodes(ns =>
				ns.map(n =>
					n.id !== nodeId ? n : {...n, data: {...n.data, ...data}},
				),
			);
		},
		[setNodes],
	);

	const deleteNode = useCallback(
		(nodeId: string) => {
			setNodes(ns => ns.filter(n => n.id !== nodeId));
			setEdges(es =>
				es.filter(e => e.source !== nodeId && e.target !== nodeId),
			);
		},
		[setNodes, setEdges],
	);

	return {
		nodes,
		edges,
		onNodesChange,
		onEdgesChange,
		setNodes,
		addService,
		updateNode,
		deleteNode,
	};
}
