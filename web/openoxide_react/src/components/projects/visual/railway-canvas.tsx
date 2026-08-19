import {useState, useCallback, useEffect, useMemo, useRef} from 'react';
import {
	ReactFlow,
	ReactFlowProvider,
	Controls,
	useNodesState,
	useEdgesState,
	useReactFlow,
	addEdge,
	type Connection,
	type Edge,
	type Node,
	type NodeTypes,
	ConnectionLineType,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {load as yamlLoad} from 'js-yaml';
import {MousePointer2, MoveRight} from 'lucide-react';

import {CanvasActionsContext} from './canvas-context';
import {AppCanvasNode, DbCanvasNode} from './nodes/railway-nodes';
import {
	ComposeGroupNode,
	ComposeServiceMiniNode,
	S3CanvasNode,
} from './nodes/compose-s3-nodes';
import {CustomEdge} from './edges/custom-edge';
import {ServiceLinkerModal} from './modals/service-linker-modal';
import {ServiceInspectorDrawer} from './drawer/service-inspector-drawer';

// ─── Types ────────────────────────────────────────────────────────────────────

interface RailwayCanvasProps {
	applications?: any[];
	databases?: any[];
	composeStacks?: any[];
	destinations?: any[];
	onOpenAddApp?: () => void;
	onOpenAddDb?: () => void;
	onOpenAddCompose?: () => void;
}

const nodeTypes: NodeTypes = {
	appNode: AppCanvasNode as any,
	dbNode: DbCanvasNode as any,
	s3Node: S3CanvasNode as any,
	composeGroupNode: ComposeGroupNode as any,
	composeMiniServiceNode: ComposeServiceMiniNode as any,
};
const edgeTypes = {customEdge: CustomEdge as any};

// ─── Helpers ─────────────────────────────────────────────────────────────────

function parseServices(yaml?: string) {
	if (!yaml?.trim()) return [{serviceName: 'web', image: 'nginx'}];
	try {
		const doc = yamlLoad(yaml) as any;
		const svcs = doc?.services ?? {};
		const list = Object.entries(svcs).map(([name, s]: [string, any]) => ({
			serviceName: name,
			image: (s?.image as string) || 'image',
		}));
		return list.length ? list : [{serviceName: 'web', image: 'nginx'}];
	} catch {
		return [{serviceName: 'web', image: 'nginx'}];
	}
}

const COL_X = (col: number) => 80 + col * 280;
const ROW_Y = (row: number) => 80 + row * 110;

function buildNodes(
	applications: any[],
	databases: any[],
	composeStacks: any[],
	destinations: any[],
	expanded: Record<number, boolean>,
): Node[] {
	const out: Node[] = [];

	applications.forEach((app, i) =>
		out.push({
			id: `app-${app.id}`,
			type: 'appNode',
			position: {x: COL_X(0), y: ROW_Y(i)},
			data: {
				id: app.id,
				name: app.name ?? app.app_name ?? 'App',
				status: app.status ?? 'running',
				framework: app.build_type ?? 'Docker',
				url: app.domain ? `https://${app.domain}` : undefined,
			},
		}),
	);

	databases.forEach((db, i) =>
		out.push({
			id: `db-${db.id}`,
			type: 'dbNode',
			position: {x: COL_X(1), y: ROW_Y(i)},
			data: {
				id: db.id,
				name: db.name ?? db.database_name ?? 'DB',
				dbType: db.db_type ?? 'postgres',
				status: db.status ?? 'running',
				port: db.port,
			},
		}),
	);

	composeStacks.forEach((cmp, i) => {
		const isExpanded = !!expanded[cmp.id];
		const children = parseServices(cmp.compose_file);
		const count = children.length;
		const gid = `cmp-${cmp.id}`;
		out.push({
			id: gid,
			type: 'composeGroupNode',
			position: {x: COL_X(2), y: ROW_Y(i * 2)},
			style: {
				width: isExpanded ? Math.max(count * 160 + 20, 360) : 185,
				height: isExpanded ? 140 : ('auto' as any),
			},
			data: {
				id: cmp.id,
				name: cmp.name ?? cmp.app_name ?? 'Stack',
				status: cmp.status ?? 'running',
				servicesCount: count,
				expanded: isExpanded,
			},
		});
		children.forEach((svc, idx) =>
			out.push({
				id: `${gid}-svc-${svc.serviceName}`,
				type: 'composeMiniServiceNode',
				parentId: gid,
				extent: 'parent',
				position: {x: 10 + idx * 158, y: 60},
				hidden: !isExpanded,
				data: {serviceName: svc.serviceName, image: svc.image},
			}),
		);
	});

	destinations.forEach((dest, i) =>
		out.push({
			id: `s3-${dest.id}`,
			type: 's3Node',
			position: {x: COL_X(3), y: ROW_Y(i)},
			data: {
				id: dest.id,
				name: dest.name ?? 'Storage',
				provider: dest.provider ?? 'S3',
				bucket: dest.bucket ?? '',
			},
		}),
	);

	return out;
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
	const {screenToFlowPosition} = useReactFlow();

	const [activeNode, setActiveNode] = useState<any | null>(null);
	const [expandedGroups, setExpandedGroups] = useState<
		Record<number, boolean>
	>({});
	const [mode, setMode] = useState<'select' | 'arrow'>('select');
	const [pendingConn, setPendingConn] = useState<any | null>(null);
	const [ctxMenu, setCtxMenu] = useState<{x: number; y: number} | null>(
		null,
	);
	const [arrowDrag, setArrowDrag] = useState<{
		ss: {x: number; y: number};
		sf: {x: number; y: number};
		cs: {x: number; y: number};
	} | null>(null);

	const canvasRef = useRef<HTMLDivElement>(null);
	const sigRef = useRef('');

	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);

	// Rebuild service nodes
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

	// Keyboard: A = arrow mode, Esc/V/S = select
	useEffect(() => {
		const h = (e: KeyboardEvent) => {
			const tag = (e.target as HTMLElement).tagName;
			if (tag === 'INPUT' || tag === 'TEXTAREA') return;
			if (e.key === 'a' || e.key === 'A') setMode('arrow');
			if (['Escape', 's', 'S', 'v', 'V'].includes(e.key)) {
				setMode('select');
				setArrowDrag(null);
			}
		};
		window.addEventListener('keydown', h);
		return () => window.removeEventListener('keydown', h);
	}, []);

	// Arrow drag — mouse handlers
	const onMouseDown = useCallback(
		(e: React.MouseEvent) => {
			if (mode !== 'arrow') return;
			if ((e.target as HTMLElement).closest('.react-flow__node')) return;
			e.preventDefault();
			const rect = canvasRef.current?.getBoundingClientRect() ?? {
				left: 0,
				top: 0,
			};
			const ss = {x: e.clientX - rect.left, y: e.clientY - rect.top};
			const sf = screenToFlowPosition({x: e.clientX, y: e.clientY});
			setArrowDrag({ss, sf, cs: ss});
		},
		[mode, screenToFlowPosition],
	);

	const onMouseMove = useCallback(
		(e: React.MouseEvent) => {
			if (!arrowDrag) return;
			const rect = canvasRef.current?.getBoundingClientRect() ?? {
				left: 0,
				top: 0,
			};
			setArrowDrag(prev =>
				prev
					? {
							...prev,
							cs: {x: e.clientX - rect.left, y: e.clientY - rect.top},
						}
					: null,
			);
		},
		[arrowDrag],
	);

	const onMouseUp = useCallback(
		(e: React.MouseEvent) => {
			if (!arrowDrag) return;
			const endFlow = screenToFlowPosition({x: e.clientX, y: e.clientY});
			const dx = endFlow.x - arrowDrag.sf.x,
				dy = endFlow.y - arrowDrag.sf.y;
			if (Math.sqrt(dx * dx + dy * dy) < 10) {
				setArrowDrag(null);
				return;
			}
			const ts = Date.now();
			const sid = `anc-${ts}-s`,
				eid = `anc-${ts}-e`;
			setNodes(prev => [
				...prev,
				{
					id: sid,
					type: 'arrowAnchor' as any,
					position: arrowDrag.sf,
					data: {},
				},
				{id: eid, type: 'arrowAnchor' as any, position: endFlow, data: {}},
			]);
			setEdges(prev => [
				...prev,
				{
					id: `arrow-${ts}`,
					source: sid,
					target: eid,
					type: 'customEdge',
					animated: false,
					data: {},
				},
			]);
			setArrowDrag(null);
		},
		[arrowDrag, screenToFlowPosition, setNodes, setEdges],
	);

	const onConnect = useCallback(
		(conn: Connection) => {
			const srcId = parseInt(conn.source.replace(/^[a-z]+-/, ''));
			const tgtId = parseInt(conn.target.replace(/^[a-z]+-/, ''));
			const srcApp = applications.find((a: any) => a.id === srcId);
			const tgtDb = databases.find((d: any) => d.id === tgtId);
			if (srcApp && tgtDb)
				setPendingConn({
					sourceId: srcId,
					targetId: tgtId,
					sourceName: srcApp.name ?? srcApp.app_name,
					targetName: tgtDb.name ?? tgtDb.database_name,
					targetDbType: tgtDb.db_type,
				});
			setEdges(eds =>
				addEdge({...conn, type: 'customEdge', data: {}}, eds),
			);
		},
		[applications, databases, setEdges],
	);

	const onPaneContextMenu = useCallback(
		(e: React.MouseEvent | MouseEvent) => {
			if (mode === 'arrow') return;
			e.preventDefault();
			const x = 'clientX' in e ? e.clientX : 0;
			const y = 'clientY' in e ? e.clientY : 0;
			const b = (
				e.currentTarget as HTMLElement
			)?.getBoundingClientRect() ?? {left: 0, top: 0};
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
					nodeTypes={nodeTypes}
					edgeTypes={edgeTypes}
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

				{/* Arrow preview line */}
				{mode === 'arrow' && arrowDrag && (
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
							cx={arrowDrag.ss.x}
							cy={arrowDrag.ss.y}
							r={3.5}
							fill="#58a6ff"
							opacity="0.8"
						/>
						<line
							x1={arrowDrag.ss.x}
							y1={arrowDrag.ss.y}
							x2={arrowDrag.cs.x}
							y2={arrowDrag.cs.y}
							stroke="#58a6ff"
							strokeWidth={1.5}
							strokeDasharray="5 4"
							opacity="0.7"
							markerEnd="url(#prev-mk)"
						/>
					</svg>
				)}

				{/* ── Minimal pill toolbar ── */}
				<div
					style={{
						position: 'absolute',
						top: 12,
						left: '50%',
						transform: 'translateX(-50%)',
						zIndex: 20,
						display: 'flex',
						alignItems: 'center',
						gap: 2,
						padding: '3px 5px',
						borderRadius: 99,
						background: '#161b22',
						border: '1px solid rgba(255,255,255,0.08)',
						boxShadow: '0 1px 8px rgba(0,0,0,0.3)',
					}}>
					{[
						{
							icon: <MousePointer2 size={13} />,
							label: 'Select  V',
							key: 'select' as const,
						},
						{
							icon: <MoveRight size={13} />,
							label: 'Arrow  A',
							key: 'arrow' as const,
						},
					].map(t => (
						<button
							key={t.key}
							type="button"
							title={t.label}
							onClick={() => {
								setMode(t.key);
								setArrowDrag(null);
							}}
							style={{
								display: 'flex',
								alignItems: 'center',
								gap: 5,
								padding: '4px 10px',
								borderRadius: 99,
								border: 'none',
								cursor: 'pointer',
								fontSize: 11.5,
								fontWeight: 500,
								background:
									mode === t.key
										? t.key === 'arrow'
											? 'rgba(88,166,255,0.15)'
											: 'rgba(255,255,255,0.08)'
										: 'transparent',
								color:
									mode === t.key
										? t.key === 'arrow'
											? '#58a6ff'
											: '#e6edf3'
										: '#6e7681',
								transition: 'all .12s',
							}}>
							{t.icon}
							{t.key === 'arrow' && mode === 'arrow'
								? 'Arrow'
								: t.key === 'select' && mode === 'select'
									? 'Select'
									: ''}
						</button>
					))}
				</div>

				{/* Arrow hint */}
				{mode === 'arrow' && (
					<div
						style={{
							position: 'absolute',
							bottom: 12,
							left: '50%',
							transform: 'translateX(-50%)',
							zIndex: 20,
							background: '#161b22',
							border: '1px solid rgba(88,166,255,0.3)',
							borderRadius: 99,
							padding: '4px 14px',
							fontSize: 11,
							color: '#58a6ff',
							fontWeight: 500,
							display: 'flex',
							alignItems: 'center',
							gap: 6,
							boxShadow: '0 1px 8px rgba(0,0,0,0.3)',
							pointerEvents: 'none',
						}}>
						<MoveRight size={12} />
						Drag to draw connection — press V to exit
					</div>
				)}

				{/* Right-click context menu */}
				{ctxMenu && (
					<div
						onClick={e => e.stopPropagation()}
						style={{
							position: 'absolute',
							top: ctxMenu.y,
							left: ctxMenu.x,
							zIndex: 50,
							background: '#161b22',
							border: '1px solid rgba(255,255,255,0.1)',
							borderRadius: 10,
							padding: 4,
							minWidth: 160,
							boxShadow: '0 8px 24px rgba(0,0,0,0.5)',
						}}>
						<p
							style={{
								margin: '2px 10px 6px',
								fontSize: 10,
								fontWeight: 600,
								color: '#6e7681',
								textTransform: 'uppercase',
								letterSpacing: '0.06em',
							}}>
							Add Service
						</p>
						{[
							{label: '🌐  Application', fn: onOpenAddApp},
							{label: '🗄️  Database', fn: onOpenAddDb},
							{label: '📦  Docker Compose', fn: onOpenAddCompose},
						].map(({label, fn}) => (
							<button
								key={label}
								type="button"
								onClick={() => {
									setCtxMenu(null);
									fn?.();
								}}
								style={{
									display: 'block',
									width: '100%',
									textAlign: 'left',
									padding: '6px 10px',
									border: 'none',
									borderRadius: 7,
									background: 'transparent',
									cursor: 'pointer',
									fontSize: 12.5,
									color: '#e6edf3',
									transition: 'background .1s',
								}}
								onMouseEnter={e =>
									(e.currentTarget.style.background =
										'rgba(255,255,255,0.06)')
								}
								onMouseLeave={e =>
									(e.currentTarget.style.background = 'transparent')
								}>
								{label}
							</button>
						))}
					</div>
				)}

				{pendingConn && (
					<ServiceLinkerModal
						isOpen
						onClose={() => setPendingConn(null)}
						sourceNode={{
							id: pendingConn.sourceId,
							name: pendingConn.sourceName,
							type: 'app',
						}}
						targetNode={{
							id: pendingConn.targetId,
							name: pendingConn.targetName,
							type: 'database',
							dbType: pendingConn.targetDbType,
						}}
						onConfirmLink={async () => setPendingConn(null)}
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
