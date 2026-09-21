import {useState, useCallback, useMemo, useRef} from 'react';
import {
	ReactFlow,
	ReactFlowProvider,
	Controls,
	ConnectionLineType,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import {CanvasActionsContext} from './canvas-context';
import {ServiceInspectorDrawer} from './drawer/service-inspector-drawer';
import {railwayNodeTypes, railwayEdgeTypes} from './railway-build-nodes';
import {useRailwayNodes} from './use-railway-nodes';
import {useCanvasMode} from './use-canvas-mode';
import {useArrowDrag} from './use-arrow-drag';
import {useServiceLinker} from './use-service-linker';
import {ArrowPreview} from './arrow-preview';
import {
	ModeToolbar,
	ArrowHint,
	RailwayAddMenu,
} from './railway-canvas-overlays';
import {RailwayLinkerModal} from './railway-linker-modal';

interface RailwayCanvasProps {
	applications?: any[];
	databases?: any[];
	composeStacks?: any[];
	destinations?: any[];
	onOpenAddApp?: () => void;
	onOpenAddDb?: () => void;
	onOpenAddCompose?: () => void;
}
// ─── Inner canvas ────────────────────────────────────────────────────────────

function CanvasContent({
	applications = [],
	databases = [],
	composeStacks = [],
	destinations = [],
	onOpenAddApp,
	onOpenAddDb,
	onOpenAddCompose,
}: RailwayCanvasProps) {
	const [activeNode, setActiveNode] = useState<any | null>(null);
	const [expandedGroups, setExpandedGroups] = useState<
		Record<number, boolean>
	>({});
	const {mode, pickMode} = useCanvasMode(() => setArrowDrag(null));
	const [ctxMenu, setCtxMenu] = useState<{x: number; y: number} | null>(
		null,
	);

	const canvasRef = useRef<HTMLDivElement>(null);

	const {nodes, edges, setNodes, setEdges, onNodesChange, onEdgesChange} =
		useRailwayNodes(
			applications,
			databases,
			composeStacks,
			destinations,
			expandedGroups,
		);

	const {arrowDrag, setArrowDrag, onMouseDown, onMouseMove, onMouseUp} =
		useArrowDrag(mode, canvasRef, setNodes, setEdges);

	const {pendingConn, onConnect, clearPending} = useServiceLinker(
		applications,
		databases,
		setEdges,
	);

	const onPaneContextMenu = useCallback(
		(e: React.MouseEvent | MouseEvent) => {
			if (mode === 'arrow') return;
			e.preventDefault();
			const x = 'clientX' in e ? e.clientX : 0;
			const y = 'clientY' in e ? e.clientY : 0;
			const b = (
				e.currentTarget as HTMLElement
			)?.getBoundingClientRect() ?? {
				left: 0,
				top: 0,
			};
			setCtxMenu({x: x - b.left, y: y - b.top});
		},
		[mode],
	);

	const ctxActions = useMemo(
		() => ({
			inspect: setActiveNode,
			toggleExpand: (id: number) =>
				setExpandedGroups(p => ({...p, [id]: !p[id]})),
		}),
		[],
	);

	return (
		<CanvasActionsContext.Provider value={ctxActions}>
			<div
				ref={canvasRef}
				className="relative w-full font-sans"
				style={{
					height: 640,
					borderRadius: 12,
					overflow: 'hidden',
					background: '#0d1117',
					border: '1px solid rgba(255,255,255,0.07)',
					cursor: mode === 'arrow' ? 'crosshair' : 'default',
				}}
				onClick={() => setCtxMenu(null)}
				onMouseMove={onMouseMove}
				onMouseDown={mode === 'arrow' ? onMouseDown : undefined}
				onMouseUp={mode === 'arrow' ? onMouseUp : undefined}>
				<ReactFlow
					nodes={nodes}
					edges={edges}
					nodeTypes={railwayNodeTypes}
					edgeTypes={railwayEdgeTypes}
					onNodesChange={onNodesChange}
					onEdgesChange={onEdgesChange}
					onConnect={onConnect}
					onPaneContextMenu={onPaneContextMenu}
					connectionLineType={ConnectionLineType.Bezier}
					connectionLineStyle={{
						stroke: '#58a6ff',
						strokeWidth: 1.5,
						strokeDasharray: '5 4',
						opacity: 0.7,
					}}
					nodesDraggable={mode === 'select'}
					panOnDrag={mode === 'select'}
					edgesReconnectable
					elevateEdgesOnSelect
					fitView
					fitViewOptions={{padding: 0.3}}
					proOptions={{hideAttribution: true}}
					style={{background: '#0d1117'}}>
					{/* No background grid — Railway has none */}
					<Controls
						position="bottom-right"
						style={{
							background: '#161b22',
							border: '1px solid rgba(255,255,255,0.08)',
							borderRadius: 8,
							padding: 2,
						}}
					/>
				</ReactFlow>

				{mode === 'arrow' && arrowDrag && (
					<ArrowPreview drag={arrowDrag} />
				)}

				<ModeToolbar mode={mode} onMode={pickMode} />

				{mode === 'arrow' && <ArrowHint />}

				{ctxMenu && (
					<RailwayAddMenu
						x={ctxMenu.x}
						y={ctxMenu.y}
						onPick={fn => {
							setCtxMenu(null);
							fn?.();
						}}
						items={[
							{label: '🌐  Application', fn: onOpenAddApp},
							{label: '🗄️  Database', fn: onOpenAddDb},
							{label: '📦  Docker Compose', fn: onOpenAddCompose},
						]}
					/>
				)}

				{pendingConn && (
					<RailwayLinkerModal
						pending={pendingConn}
						onClose={clearPending}
					/>
				)}

				<ServiceInspectorDrawer
					isOpen={!!activeNode}
					onClose={() => setActiveNode(null)}
					node={activeNode ?? undefined}
				/>
			</div>
		</CanvasActionsContext.Provider>
	);
}

export function RailwayCanvas(props: RailwayCanvasProps) {
	return (
		<ReactFlowProvider>
			<CanvasContent {...props} />
		</ReactFlowProvider>
	);
}
