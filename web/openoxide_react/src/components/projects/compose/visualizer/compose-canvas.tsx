import {
	useEffect,
	useState,
	useCallback,
	useRef,
	useMemo,
	type CSSProperties,
} from 'react';
import {
	ReactFlow,
	Controls,
	Background,
	BackgroundVariant,
	Panel,
	useNodesState,
	useEdgesState,
	useReactFlow,
	useNodesInitialized,
	type Node,
	type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {Grid3x3} from 'lucide-react';
import {cn} from '#/api/utils';
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
	readOnly = false,
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
	monitorId,
	monitorType,
}: CanvasInnerProps) {
	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
	const {fitView} = useReactFlow();
	const containerRef = useRef<HTMLDivElement>(null);
	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
		targetType: MenuTarget;
		targetData: any;
	} | null>(null);
	const [showGrid, setShowGrid] = useState(() => {
		if (typeof window === 'undefined') return true;
		return localStorage.getItem('compose-canvas-grid') !== 'off';
	});

	const toggleGrid = useCallback(() => {
		setShowGrid(prev => {
			const next = !prev;
			localStorage.setItem('compose-canvas-grid', next ? 'on' : 'off');
			return next;
		});
	}, []);

	const fitOpts = useMemo(
		() =>
			readOnly
				? {padding: 0.12, minZoom: 0.05, maxZoom: 1}
				: {padding: 0.4, minZoom: 0.3, maxZoom: 1.2},
		[readOnly],
	);

	// Rebuild only when the graph *content* changes. Parents routinely pass freshly-allocated arrays (query fallbacks, inline `?? []`); rebuilding on those unmounts every node's measurement, briefly blanking the graph down to stray edge arrows. Payloads from ConnectRPC carry bigints, which JSON.stringify rejects.
	const graphKey = useMemo(
		() =>
			JSON.stringify(
				{services, backups, schedules, domains, readOnly},
				(_k, v) => (typeof v === 'bigint' ? v.toString() : v),
			),
		[services, backups, schedules, domains, readOnly],
	);

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
				readOnly,
				// Stack-level docker stats are one container's line — only
				// honest on a single-service canvas.
				monitorId: services.length === 1 ? monitorId : undefined,
				monitorType: services.length === 1 ? monitorType : undefined,
			},
		);
		setNodes(n);
		setEdges(e);
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [
		graphKey,
		onAddDomain,
		onAddSchedule,
		onAddBackup,
		onOpenTerminal,
		onViewLogs,
		monitorId,
		monitorType,
		setNodes,
		setEdges,
	]);

	// A node is "initialized" only once it has been measured with non-zero size. Until then edges collapse into stray arrows, so they wait here.
	const nodesReady = useNodesInitialized();

	// Re-fit every time the graph becomes measurable (initial mount and each data rebuild). A one-shot fit was racing container/node measurements on fast reloads and leaving the viewport parked on empty space.
	useEffect(() => {
		if (!nodesReady || nodes.length === 0) return;
		const t = setTimeout(() => {
			try {
				fitView(fitOpts);
			} catch {}
		}, 80);
		return () => clearTimeout(t);
	}, [nodesReady, nodes.length, fitView, fitOpts]);

	// Containers resize after mount (tab activation, card animations) and when the window shrinks; keep the graph re-fitted to the current size.
	useEffect(() => {
		const el = containerRef.current;
		if (!el) return;
		let t: ReturnType<typeof setTimeout>;
		const ro = new ResizeObserver(() => {
			clearTimeout(t);
			t = setTimeout(() => {
				try {
					fitView(fitOpts);
				} catch {}
			}, 120);
		});
		ro.observe(el);
		return () => {
			clearTimeout(t);
			ro.disconnect();
		};
	}, [fitView, fitOpts]);

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
		<div
			ref={containerRef}
			style={{width: '100%', height: '100%', position: 'relative'}}>
			<ReactFlow
				nodes={nodes}
				edges={nodesReady ? edges : []}
				nodeTypes={composeNodeTypes}
				edgeTypes={composeEdgeTypes}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onNodeContextMenu={readOnly ? undefined : (handleOpenMenu as any)}
				onNodeClick={readOnly ? undefined : (handleOpenMenu as any)}
				nodesConnectable={false}
				nodesDraggable={!readOnly}
				elementsSelectable={!readOnly}
				panOnDrag={!readOnly}
				zoomOnScroll={!readOnly}
				zoomOnDoubleClick={!readOnly}
				fitView
				fitViewOptions={fitOpts}
				minZoom={readOnly ? 0.05 : 0.2}
				maxZoom={2}
				proOptions={{hideAttribution: true}}
				className={cn(
					'bg-[color-mix(in_srgb,var(--foreground)_3.5%,var(--background))] [&_.react-flow__controls]:overflow-hidden [&_.react-flow__controls]:rounded-lg',
					readOnly && 'bg-transparent',
				)}
				style={
					{
						'--xy-controls-button-background-color': 'var(--card)',
						'--xy-controls-button-background-color-hover': 'var(--accent)',
						'--xy-controls-button-color': 'var(--muted-foreground)',
						'--xy-controls-button-color-hover': 'var(--foreground)',
						'--xy-controls-button-border-color': 'var(--border)',
						'--xy-background-pattern-color':
							'color-mix(in srgb, var(--foreground) 18%, transparent)',
					} as CSSProperties
				}>
				{!readOnly && showGrid && (
					<Background
						variant={BackgroundVariant.Lines}
						gap={40}
						lineWidth={0.6}
					/>
				)}
				{!readOnly && (
					<>
						<Controls position="bottom-right" showInteractive={false} />
						<Panel position="top-right">
							<button
								type="button"
								onClick={toggleGrid}
								aria-pressed={showGrid}
								title={showGrid ? 'Hide grid' : 'Show grid'}
								className={cn(
									'flex size-8 cursor-pointer items-center justify-center rounded-lg border bg-card shadow-sm transition-colors',
									showGrid
										? 'border-primary/40 text-foreground'
										: 'border-border text-muted-foreground hover:bg-accent hover:text-foreground',
								)}>
								<Grid3x3 className="size-4" />
							</button>
						</Panel>
					</>
				)}
				{!readOnly && services.length > 0 && <CanvasLegend />}
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
