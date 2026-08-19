import {useCallback, useEffect, useRef, useState} from 'react';
import {
	ReactFlow,
	ReactFlowProvider,
	Background,
	BackgroundVariant,
	Controls,
	useNodesState,
	useEdgesState,
	type Edge,
	type Node,
	type NodeTypes,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {Plus, X, Check} from 'lucide-react';
import {
	siPostgresql,
	siMysql,
	siRedis,
	siMongodb,
	siNginx,
	siDocker,
	siNodedotjs,
} from 'simple-icons';
import {
	StackServiceCard,
	type StackServiceData,
} from './stack-service-card';
import {StackConfigPanel} from './stack-config-panel';

// ─── Edge style ───────────────────────────────────────────────────────────────

function StackEdge({
	sourceX,
	sourceY,
	targetX,
	targetY,
	selected,
	id,
}: any) {
	const mx = (sourceX + targetX) / 2;
	const path = `M${sourceX} ${sourceY} C${mx} ${sourceY} ${mx} ${targetY} ${targetX} ${targetY}`;
	const col = selected ? '#58a6ff' : '#30363d';
	return (
		<>
			<path
				d={path}
				fill="none"
				stroke="transparent"
				strokeWidth={14}
				className="react-flow__edge-interaction"
			/>
			<defs>
				<marker
					id={`sk-mk-${id}`}
					markerWidth="7"
					markerHeight="7"
					refX="5"
					refY="3.5"
					orient="auto"
					markerUnits="userSpaceOnUse">
					<path
						d="M0,0.5 L5.5,3.5 L0,6.5 L1,3.5 Z"
						fill={col}
						opacity="0.7"
					/>
				</marker>
			</defs>
			<path
				d={path}
				fill="none"
				stroke={col}
				strokeWidth={1.5}
				strokeDasharray="6 4"
				opacity={selected ? 0.9 : 0.45}
				markerEnd={`url(#sk-mk-${id})`}
				style={{transition: 'stroke .15s, opacity .15s'}}
			/>
		</>
	);
}

// ─── Preset templates ─────────────────────────────────────────────────────────

interface Template {
	key: string;
	label: string;
	sub: string;
	icon: string;
	color: string;
	data: Partial<StackServiceData>;
}

const TEMPLATES: Template[] = [
	{
		key: 'nginx',
		label: 'Nginx',
		sub: 'Web server / proxy',
		icon: siNginx.path,
		color: '#009639',
		data: {
			image: 'nginx:alpine',
			volumes: ['/etc/nginx/conf.d'],
			envVars: [],
		},
	},
	{
		key: 'node',
		label: 'Node.js',
		sub: 'Custom app',
		icon: siNodedotjs.path,
		color: '#339933',
		data: {
			image: 'node:20-alpine',
			volumes: [],
			envVars: [{key: 'NODE_ENV', value: 'production'}],
		},
	},
	{
		key: 'postgres',
		label: 'PostgreSQL',
		sub: 'Managed database',
		icon: siPostgresql.path,
		color: '#336791',
		data: {
			image: 'postgres:16-alpine',
			volumes: ['/var/lib/postgresql/data'],
			envVars: [
				{key: 'POSTGRES_DB', value: 'app'},
				{key: 'POSTGRES_USER', value: 'postgres'},
				{key: 'POSTGRES_PASSWORD', value: 'secret'},
			],
		},
	},
	{
		key: 'mysql',
		label: 'MySQL',
		sub: 'Managed database',
		icon: siMysql.path,
		color: '#4479A1',
		data: {
			image: 'mysql:8',
			volumes: ['/var/lib/mysql'],
			envVars: [
				{key: 'MYSQL_DATABASE', value: 'app'},
				{key: 'MYSQL_ROOT_PASSWORD', value: 'secret'},
			],
		},
	},
	{
		key: 'redis',
		label: 'Redis',
		sub: 'Cache / queue',
		icon: siRedis.path,
		color: '#FF4438',
		data: {image: 'redis:7-alpine', volumes: ['/data'], envVars: []},
	},
	{
		key: 'mongodb',
		label: 'MongoDB',
		sub: 'Document database',
		icon: siMongodb.path,
		color: '#47A248',
		data: {
			image: 'mongo:7',
			volumes: ['/data/db'],
			envVars: [{key: 'MONGO_INITDB_DATABASE', value: 'app'}],
		},
	},
	{
		key: 'custom',
		label: 'Custom',
		sub: 'Any Docker image',
		icon: siDocker.path,
		color: '#1e90ff',
		data: {image: 'your-image:latest', volumes: [], envVars: []},
	},
];

// ─── Add service modal ────────────────────────────────────────────────────────

function AddServiceModal({
	onAdd,
	onClose,
}: {
	onAdd: (t: Template, name: string) => void;
	onClose: () => void;
}) {
	const [selected, setSelected] = useState<Template | null>(null);
	const [name, setName] = useState('');

	const confirm = () => {
		if (!selected) return;
		const n =
			name.trim() || selected.label.toLowerCase().replace(/\s/g, '-');
		onAdd(selected, n);
		onClose();
	};

	return (
		<div
			onClick={onClose}
			style={{
				position: 'fixed',
				inset: 0,
				zIndex: 100,
				background: 'rgba(0,0,0,0.6)',
				backdropFilter: 'blur(4px)',
				display: 'flex',
				alignItems: 'center',
				justifyContent: 'center',
			}}>
			<div
				onClick={e => e.stopPropagation()}
				style={{
					width: 420,
					borderRadius: 14,
					background: '#161b22',
					border: '1px solid rgba(255,255,255,0.1)',
					boxShadow: '0 16px 48px rgba(0,0,0,0.6)',
					overflow: 'hidden',
				}}>
				{/* Header */}
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						justifyContent: 'space-between',
						padding: '16px 18px',
						borderBottom: '1px solid rgba(255,255,255,0.07)',
					}}>
					<span style={{fontSize: 14, fontWeight: 700, color: '#e6edf3'}}>
						Add Service
					</span>
					<button
						type="button"
						onClick={onClose}
						style={{
							background: 'transparent',
							border: 'none',
							cursor: 'pointer',
							color: '#6e7681',
							display: 'flex',
							alignItems: 'center',
						}}
						onMouseEnter={e => (e.currentTarget.style.color = '#e6edf3')}
						onMouseLeave={e => (e.currentTarget.style.color = '#6e7681')}>
						<X size={15} />
					</button>
				</div>

				{/* Service grid */}
				<div
					style={{
						padding: 14,
						display: 'grid',
						gridTemplateColumns: '1fr 1fr',
						gap: 8,
					}}>
					{TEMPLATES.map(t => (
						<button
							key={t.key}
							type="button"
							onClick={() => {
								setSelected(t);
								setName(t.label.toLowerCase());
							}}
							style={{
								display: 'flex',
								alignItems: 'center',
								gap: 10,
								padding: '10px 12px',
								borderRadius: 9,
								border: 'none',
								cursor: 'pointer',
								textAlign: 'left',
								background:
									selected?.key === t.key
										? 'rgba(88,166,255,0.1)'
										: 'rgba(255,255,255,0.03)',
								outline:
									selected?.key === t.key
										? '1px solid rgba(88,166,255,0.4)'
										: '1px solid rgba(255,255,255,0.06)',
								transition: 'all .1s',
							}}
							onMouseEnter={e => {
								if (selected?.key !== t.key)
									e.currentTarget.style.background =
										'rgba(255,255,255,0.06)';
							}}
							onMouseLeave={e => {
								if (selected?.key !== t.key)
									e.currentTarget.style.background =
										'rgba(255,255,255,0.03)';
							}}>
							<div
								style={{
									width: 30,
									height: 30,
									borderRadius: 7,
									flexShrink: 0,
									background: `${t.color}18`,
									display: 'flex',
									alignItems: 'center',
									justifyContent: 'center',
								}}>
								<svg
									viewBox="0 0 24 24"
									style={{width: 15, height: 15, fill: t.color}}>
									<path d={t.icon} />
								</svg>
							</div>
							<div>
								<p
									style={{
										margin: 0,
										fontSize: 12.5,
										fontWeight: 600,
										color: '#e6edf3',
									}}>
									{t.label}
								</p>
								<p style={{margin: 0, fontSize: 10.5, color: '#6e7681'}}>
									{t.sub}
								</p>
							</div>
						</button>
					))}
				</div>

				{/* Name input */}
				{selected && (
					<div style={{padding: '0 14px 14px'}}>
						<label
							style={{
								fontSize: 11,
								fontWeight: 600,
								color: '#6e7681',
								display: 'block',
								textTransform: 'uppercase',
								letterSpacing: '0.06em',
								marginBottom: 6,
							}}>
							Service name
						</label>
						<input
							autoFocus
							value={name}
							onChange={e => setName(e.target.value)}
							onKeyDown={e => e.key === 'Enter' && confirm()}
							placeholder={selected.label.toLowerCase()}
							style={{
								width: '100%',
								padding: '8px 10px',
								borderRadius: 7,
								background: '#0d1117',
								border: '1px solid rgba(255,255,255,0.1)',
								color: '#e6edf3',
								fontSize: 13,
								outline: 'none',
								boxSizing: 'border-box',
							}}
							onFocus={e =>
								(e.target.style.borderColor = 'rgba(88,166,255,0.5)')
							}
							onBlur={e =>
								(e.target.style.borderColor = 'rgba(255,255,255,0.1)')
							}
						/>
					</div>
				)}

				{/* Footer */}
				<div
					style={{
						padding: '12px 14px',
						borderTop: '1px solid rgba(255,255,255,0.06)',
						display: 'flex',
						justifyContent: 'flex-end',
						gap: 8,
					}}>
					<button
						type="button"
						onClick={onClose}
						style={{
							padding: '7px 16px',
							borderRadius: 7,
							border: '1px solid rgba(255,255,255,0.1)',
							background: 'transparent',
							color: '#6e7681',
							fontSize: 12.5,
							cursor: 'pointer',
						}}>
						Cancel
					</button>
					<button
						type="button"
						onClick={confirm}
						disabled={!selected}
						style={{
							padding: '7px 18px',
							borderRadius: 7,
							border: 'none',
							background: selected ? '#238636' : '#21262d',
							color: selected ? '#fff' : '#6e7681',
							fontSize: 12.5,
							fontWeight: 600,
							cursor: selected ? 'pointer' : 'not-allowed',
							display: 'flex',
							alignItems: 'center',
							gap: 6,
							transition: 'all .12s',
						}}>
						<Check size={13} /> Add Service
					</button>
				</div>
			</div>
		</div>
	);
}

// ─── Canvas ───────────────────────────────────────────────────────────────────

const nodeTypes: NodeTypes = {stackService: StackServiceCard as any};
const edgeTypes = {stackEdge: StackEdge};

const COL_W = 340;
const ROW_H = 200;

function CanvasInner({environmentId}: {environmentId: number}) {
	const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
	const [showAdd, setShowAdd] = useState(false);
	const [selected, setSelected] = useState<Node | null>(null);
	const counter = useRef(0);

	// Auto-layout: arrange in columns of 3
	const nextPos = (total: number): {x: number; y: number} => {
		const col = Math.floor(total / 3);
		const row = total % 3;
		return {x: 80 + col * COL_W, y: 60 + row * ROW_H};
	};

	// Add service
	const addService = useCallback(
		(t: Template, name: string) => {
			const id = `svc-${++counter.current}`;
			const pos = nextPos(nodes.length);
			const node: Node = {
				id,
				type: 'stackService',
				position: pos,
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

	// ── Dependency auto-wire ──────────────────────────────────────────────────
	// Parses  ${{ServiceName.VARIABLE}}  in env var values.
	// Triggers on blur of any env var input (via onUpdate → setNodes → nodes change).
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

	const onNodeClick = useCallback(
		(_: any, node: Node) => setSelected(node),
		[],
	);
	const onPaneClick = useCallback(() => {
		setSelected(null);
	}, []);

	const updateNode = useCallback(
		(nodeId: string, data: Partial<StackServiceData>) => {
			setNodes(ns =>
				ns.map(n => {
					if (n.id !== nodeId) return n;
					return {...n, data: {...n.data, ...data}};
				}),
			);
			// Sync selected separately so dep-wire useEffect gets clean nodes state
			setSelected(prev => {
				if (!prev || prev.id !== nodeId) return prev;
				return {...prev, data: {...prev.data, ...data}};
			});
		},
		[setNodes],
	);

	const deleteNode = useCallback(
		(nodeId: string) => {
			setNodes(ns => ns.filter(n => n.id !== nodeId));
			setEdges(es =>
				es.filter(e => e.source !== nodeId && e.target !== nodeId),
			);
			setSelected(null);
		},
		[setNodes, setEdges],
	);

	return (
		<div
			style={{
				display: 'flex',
				width: '100%',
				height: '100%',
				position: 'relative',
			}}>
			{/* Canvas */}
			<div style={{flex: 1, position: 'relative'}}>
				<ReactFlow
					nodes={nodes}
					edges={edges}
					nodeTypes={nodeTypes}
					edgeTypes={edgeTypes}
					onNodesChange={onNodesChange}
					onEdgesChange={onEdgesChange}
					onNodeClick={onNodeClick}
					onPaneClick={onPaneClick}
					nodesConnectable={false}
					fitView
					fitViewOptions={{padding: 0.3}}
					proOptions={{hideAttribution: true}}
					style={{background: '#0d1117'}}>
					{/* Grid lines — Railway2 style */}
					<Background
						variant={BackgroundVariant.Lines}
						gap={40}
						lineWidth={0.5}
						color="rgba(255,255,255,0.04)"
						style={{background: '#0d1117'}}
					/>
					<Controls
						position="bottom-left"
						style={{
							background: '#161b22',
							border: '1px solid rgba(255,255,255,0.08)',
							borderRadius: 8,
						}}
					/>
				</ReactFlow>

				{/* + button — top right exactly like Railway */}
				<div
					style={{position: 'absolute', top: 14, right: 14, zIndex: 30}}>
					<button
						type="button"
						onClick={() => setShowAdd(true)}
						style={{
							width: 34,
							height: 34,
							borderRadius: 8,
							background: '#21262d',
							border: '1px solid rgba(255,255,255,0.1)',
							display: 'flex',
							alignItems: 'center',
							justifyContent: 'center',
							cursor: 'pointer',
							color: '#e6edf3',
							transition: 'all .12s',
						}}
						onMouseEnter={e => {
							e.currentTarget.style.background = '#30363d';
							e.currentTarget.style.borderColor = 'rgba(255,255,255,0.2)';
						}}
						onMouseLeave={e => {
							e.currentTarget.style.background = '#21262d';
							e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)';
						}}
						title="Add service">
						<Plus size={16} />
					</button>
				</div>

				{/* Empty state */}
				{nodes.length === 0 && (
					<div
						style={{
							position: 'absolute',
							inset: 0,
							display: 'flex',
							flexDirection: 'column',
							alignItems: 'center',
							justifyContent: 'center',
							pointerEvents: 'none',
							zIndex: 5,
							gap: 10,
						}}>
						<p
							style={{
								color: 'rgba(255,255,255,0.18)',
								fontSize: 14,
								fontWeight: 500,
								margin: 0,
							}}>
							Your Swarm stack is empty
						</p>
						<p
							style={{
								color: 'rgba(255,255,255,0.1)',
								fontSize: 12,
								margin: 0,
							}}>
							Click{' '}
							<strong style={{color: 'rgba(255,255,255,0.2)'}}>+</strong>{' '}
							to add your first service
						</p>
					</div>
				)}
			</div>

			{/* Right config panel */}
			{selected && (
				<StackConfigPanel
					node={selected}
					environmentId={environmentId}
					onUpdate={updateNode}
					onDelete={deleteNode}
					onClose={() => setSelected(null)}
				/>
			)}

			{/* Add modal */}
			{showAdd && (
				<AddServiceModal
					onAdd={addService}
					onClose={() => setShowAdd(false)}
				/>
			)}
		</div>
	);
}

export function StackCanvas({environmentId}: {environmentId: number}) {
	return (
		<ReactFlowProvider>
			<CanvasInner environmentId={environmentId} />
		</ReactFlowProvider>
	);
}
