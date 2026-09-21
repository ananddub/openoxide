import {useEffect, useState, useCallback, useRef} from 'react';
import {
	ReactFlow,
	Controls,
	Background,
	BackgroundVariant,
	useNodesState,
	useEdgesState,
	useReactFlow,
	type Node,
	type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {
	buildGraph,
	composeNodeTypes,
	composeEdgeTypes,
} from './compose-graph';
import {CanvasContextMenu} from './canvas-context-menu';
import {CanvasLegend, CanvasEmptyState} from './compose-canvas-overlay';
import type {CanvasInnerProps} from './compose-canvas-types';

type MenuTarget = 'service' | 'domain' | 'schedule' | 'backup';

const NODE_TYPE_TO_TARGET: Record<string, MenuTarget> = {
	composeDomain: 'domain',
	composeCron: 'schedule',
	composeBackup: 'backup',
	composeService: 'service',
};

// ─── Inner Canvas Component ───────────────────────────────────────────────────

export function CanvasInner({
	services,
	backups,
	schedules,
	domains,
	gitBuildPath,
	isGitSource,
	onAddDomain,
	onAddSchedule,
	onAddBackup,
	onOpenTerminal,
	onViewLogs,
	onViewDeployLogs,
	onEditDomain,
	onDeleteDomain,
	onEditSchedule,
	onDeleteSchedule,
	onEditBackup,
	onDeleteBackup,
}: CanvasInnerProps) {
	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
	const {fitView} = useReactFlow();
	const hasFittedViewRef = useRef(false);
	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
		targetType: MenuTarget;
		targetData: any;
	} | null>(null);

	useEffect(() => {
		const {nodes: n, edges: e} = buildGraph(
			services,
			backups,
			schedules,
			domains,
			{
				onAddDomain,
				onAddSchedule,
				onAddBackup,
				onOpenTerminal,
				onViewLogs,
			},
		);
		setNodes(n);
		setEdges(e);

		if (!hasFittedViewRef.current && n.length > 0) {
			hasFittedViewRef.current = true;
			const t = setTimeout(() => {
				try {
					fitView({padding: 0.4, minZoom: 0.3, maxZoom: 1.2});
				} catch {}
			}, 100);
			return () => clearTimeout(t);
		}
	}, [
		services,
		backups,
		schedules,
		domains,
		onAddDomain,
		onAddSchedule,
		onAddBackup,
		onOpenTerminal,
		onViewLogs,
		setNodes,
		setEdges,
		fitView,
	]);

	const handleOpenMenu = useCallback(
		(event: React.MouseEvent, node: Node) => {
			event.preventDefault();
			setContextMenu({
				x: event.clientX,
				y: event.clientY,
				targetType: NODE_TYPE_TO_TARGET[node.type ?? ''] ?? 'service',
				targetData: node.data,
			});
		},
		[],
	);

	return (
		<div style={{width: '100%', height: '100%', position: 'relative'}}>
			<ReactFlow
				nodes={nodes}
				edges={edges}
				nodeTypes={composeNodeTypes}
				edgeTypes={composeEdgeTypes}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onNodeContextMenu={handleOpenMenu as any}
				onNodeClick={handleOpenMenu as any}
				nodesConnectable={false}
				nodesDraggable={true}
				fitView
				fitViewOptions={{padding: 0.4, minZoom: 0.3, maxZoom: 1.2}}
				minZoom={0.2}
				maxZoom={2}
				proOptions={{hideAttribution: true}}
				style={{background: '#0d1117'}}>
				<Background
					variant={BackgroundVariant.Lines}
					gap={40}
					lineWidth={0.4}
					color="#21262d"
					style={{background: '#0d1117'}}
				/>
				<Controls
					position="bottom-right"
					style={{
						background: '#161b22',
						border: '1px solid rgba(255,255,255,0.08)',
						borderRadius: 8,
						color: '#8b949e',
					}}
				/>
				{services.length > 0 && <CanvasLegend />}
				{services.length === 0 && (
					<CanvasEmptyState
						isGitSource={isGitSource}
						gitBuildPath={gitBuildPath}
					/>
				)}
			</ReactFlow>

			{contextMenu && (
				<CanvasContextMenu
					x={contextMenu.x}
					y={contextMenu.y}
					targetType={contextMenu.targetType}
					targetData={contextMenu.targetData}
					onClose={() => setContextMenu(null)}
					onAddDomain={onAddDomain}
					onAddSchedule={onAddSchedule}
					onAddBackup={onAddBackup}
					onOpenTerminal={onOpenTerminal}
					onViewLogs={onViewLogs}
					onViewDeployLogs={onViewDeployLogs}
					onEditDomain={onEditDomain}
					onDeleteDomain={onDeleteDomain}
					onEditSchedule={onEditSchedule}
					onDeleteSchedule={onDeleteSchedule}
					onEditBackup={onEditBackup}
					onDeleteBackup={onDeleteBackup}
				/>
			)}
		</div>
	);
}
