import {useEffect, useMemo, useState, useCallback, useRef} from 'react';
import {
	ReactFlow,
	ReactFlowProvider,
	Controls,
	Background,
	BackgroundVariant,
	useNodesState,
	useEdgesState,
	useReactFlow,
	type Node,
	type Edge,
	type NodeTypes,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {load as yamlLoad} from 'js-yaml';
import dagre from '@dagrejs/dagre';
import {ComposeServiceNode} from './compose-service-node';
import {ComposeEdge} from './compose-edge';
import {
	ComposeBackupNode,
	type VolumeBackupData,
} from './compose-backup-node';
import {ComposeCronNode, type CronJobData} from './compose-cron-node';
import {
	ComposeDomainNode,
	type ComposeDomainData,
} from './compose-domain-node';
import {CanvasContextMenu} from './canvas-context-menu';

// ─── Types ────────────────────────────────────────────────────────────────────

export interface ComposeService extends Record<string, unknown> {
	name: string;
	image: string;
	dependsOn: string[];
	envVars: Record<string, string>;
	volumes: string[];
	ports: string[];
}

// ─── YAML parser ─────────────────────────────────────────────────────────────

export function parseComposeFile(yaml: string): ComposeService[] {
	if (!yaml || !yaml.trim()) return [];
	try {
		let cleanYaml = yaml.trim();
		if (cleanYaml.startsWith('```')) {
			cleanYaml = cleanYaml
				.replace(/^```[a-zA-Z]*\n?/, '')
				.replace(/\n?```$/, '')
				.trim();
		}
		if (cleanYaml.includes('\\n') && !cleanYaml.includes('\n')) {
			cleanYaml = cleanYaml.replace(/\\n/g, '\n');
		}

		let doc: any = null;
		try {
			doc = yamlLoad(cleanYaml);
		} catch (e) {
			try {
				doc = JSON.parse(cleanYaml);
			} catch {}
		}

		if (!doc) return [];
		if (typeof doc === 'string') {
			try {
				doc = JSON.parse(doc);
			} catch {
				try {
					doc = yamlLoad(doc);
				} catch {}
			}
		}

		let svcs = doc?.services;
		if (
			!svcs &&
			typeof doc === 'object' &&
			doc !== null &&
			!Array.isArray(doc)
		) {
			const possibleSvcs: Record<string, any> = {};
			Object.entries(doc).forEach(([k, v]: [string, any]) => {
				if (
					k !== 'version' &&
					k !== 'volumes' &&
					k !== 'networks' &&
					k !== 'configs' &&
					k !== 'secrets' &&
					typeof v === 'object' &&
					v !== null &&
					!Array.isArray(v)
				) {
					if (
						v.image ||
						v.build ||
						v.ports ||
						v.environment ||
						v.depends_on ||
						v.volumes
					) {
						possibleSvcs[k] = v;
					}
				}
			});
			if (Object.keys(possibleSvcs).length > 0) {
				svcs = possibleSvcs;
			}
		}

		if (typeof svcs !== 'object' || svcs === null || Array.isArray(svcs))
			return [];

		return Object.entries(svcs).map(([name, s]: [string, any]) => {
			let dependsOn: string[] = [];
			if (Array.isArray(s?.depends_on))
				dependsOn = s.depends_on.map(String);
			else if (
				s?.depends_on &&
				typeof s.depends_on === 'object' &&
				s.depends_on !== null
			)
				dependsOn = Object.keys(s.depends_on);

			const envRaw = s?.environment ?? s?.env ?? {};
			const envVars: Record<string, string> = {};
			if (Array.isArray(envRaw)) {
				envRaw.forEach((line: any) => {
					const str = String(line);
					const idx = str.indexOf('=');
					if (idx >= 0) envVars[str.slice(0, idx)] = str.slice(idx + 1);
					else envVars[str] = '';
				});
			} else if (envRaw && typeof envRaw === 'object' && envRaw !== null) {
				Object.entries(envRaw).forEach(([k, v]) => {
					envVars[k] = v == null ? '' : String(v);
				});
			}

			const volumes = (Array.isArray(s?.volumes) ? s.volumes : []).map(
				(v: any) => {
					const str = typeof v === 'string' ? v : (v?.source ?? String(v));
					const parts = str.split(':');
					return parts.length > 1 ? parts[1] : parts[0];
				},
			);

			const ports = (Array.isArray(s?.ports) ? s.ports : []).map(
				(p: any) =>
					typeof p === 'object' ? `${p.target ?? p.published}` : String(p),
			);

			let imageStr = s?.image ?? '';
			if (!imageStr && s?.build) {
				if (typeof s.build === 'string') imageStr = `build: ${s.build}`;
				else if (typeof s.build === 'object' && s.build?.context)
					imageStr = `build: ${s.build.context}`;
				else imageStr = 'build';
			}

			return {
				name,
				image: imageStr || 'container',
				dependsOn,
				envVars,
				volumes,
				ports,
			};
		});
	} catch (e) {
		console.warn('[ComposeVisualizer] YAML parse error:', e);
		return [];
	}
}

// ─── Dagre Automatic Layout ───────────────────────────────────────────────────

function getDagreLayout(
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

	rawNodes.forEach(n => {
		g.setNode(n.id, {width: nodeW, height: nodeH});
	});

	rawEdges.forEach(e => {
		g.setEdge(e.source, e.target);
	});

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

// ─── Build ReactFlow Elements ─────────────────────────────────────────────────

function buildGraph(
	services: ComposeService[] = [],
	backups: VolumeBackupData[] = [],
	schedules: CronJobData[] = [],
	domains: ComposeDomainData[] = [],
	actionHandlers?: {
		onAddDomain?: (s: ComposeService) => void;
		onAddSchedule?: (s: ComposeService) => void;
		onAddBackup?: (s: ComposeService) => void;
		onOpenTerminal?: (s: ComposeService) => void;
		onViewLogs?: (s: ComposeService) => void;
	},
): {nodes: Node[]; edges: Edge[]} {
	const rawNodes: Node[] = [];
	const rawEdges: Edge[] = [];

	const safeServices = Array.isArray(services) ? services : [];
	const safeBackups = Array.isArray(backups) ? backups : [];
	const safeSchedules = Array.isArray(schedules) ? schedules : [];
	const safeDomains = Array.isArray(domains) ? domains : [];

	const serviceNames = new Set(safeServices.map(s => s.name));
	const edgeKeys = new Set<string>();

	const addEdge = (edge: Edge) => {
		const key = `${edge.source}->${edge.target}`;
		if (!edgeKeys.has(key)) {
			edgeKeys.add(key);
			rawEdges.push(edge);
		}
	};

	// 1. Service Nodes
	safeServices.forEach(svc => {
		rawNodes.push({
			id: svc.name,
			type: 'composeService',
			data: {
				...svc,
				...actionHandlers,
			},
			position: {x: 0, y: 0},
		});

		const deps = Array.isArray(svc.dependsOn) ? svc.dependsOn : [];
		deps.forEach(dep => {
			if (serviceNames.has(dep)) {
				addEdge({
					id: `e-${dep}-${svc.name}`,
					source: dep,
					target: svc.name,
					type: 'composeEdge',
					animated: true,
					style: {stroke: '#30363d', strokeWidth: 2},
				});
			}
		});
	});

	// 2. Domain Nodes
	safeDomains.forEach((d, idx) => {
		const domKey = d.domain || (d as any).id || idx;
		const domId = `domain-${domKey}`;
		rawNodes.push({
			id: domId,
			type: 'composeDomain',
			data: d,
			position: {x: 0, y: 0},
		});

		const rawTarget =
			d.service || (d as any).service_name || (d as any).app_name;
		const targetSvc =
			rawTarget && serviceNames.has(rawTarget)
				? rawTarget
				: safeServices.length > 0
					? safeServices[0]?.name
					: undefined;

		if (targetSvc && serviceNames.has(targetSvc)) {
			addEdge({
				id: `e-${targetSvc}-${domId}`,
				source: targetSvc,
				target: domId,
				type: 'composeEdge',
				animated: true,
				style: {stroke: '#10b981', strokeWidth: 2},
			});
		}
	});

	// 3. Volume Backup Nodes
	if (safeBackups.length > 0) {
		safeBackups.forEach((b, idx) => {
			const bKey = (b as any).id || b.name || idx;
			const bId = `backup-${bKey}`;
			rawNodes.push({
				id: bId,
				type: 'composeBackup',
				data: b,
				position: {x: 0, y: 0},
			});

			const rawTarget =
				b.service || (b as any).service_name || (b as any).app_name;
			const targetSvc =
				rawTarget && serviceNames.has(rawTarget)
					? rawTarget
					: safeServices.length > 0
						? safeServices[0]?.name
						: undefined;

			if (targetSvc && serviceNames.has(targetSvc)) {
				addEdge({
					id: `e-${targetSvc}-${bId}`,
					source: targetSvc,
					target: bId,
					type: 'composeEdge',
					animated: true,
					style: {
						stroke: '#38bdf8',
						strokeWidth: 2,
						strokeDasharray: '5,5',
					},
				});
			}
		});
	}

	// 4. CronJob Nodes
	if (safeSchedules.length > 0) {
		safeSchedules.forEach((c, idx) => {
			const cKey = (c as any).id || c.name || idx;
			const cId = `cron-${cKey}`;
			rawNodes.push({
				id: cId,
				type: 'composeCron',
				data: c,
				position: {x: 0, y: 0},
			});

			const rawTarget =
				c.target || (c as any).service_name || (c as any).app_name;
			const targetSvc =
				rawTarget && serviceNames.has(rawTarget)
					? rawTarget
					: safeServices.length > 0
						? safeServices[0]?.name
						: undefined;

			if (targetSvc && serviceNames.has(targetSvc)) {
				addEdge({
					id: `e-${cId}-${targetSvc}`,
					source: cId,
					target: targetSvc,
					type: 'composeEdge',
					animated: true,
					style: {stroke: '#c084fc', strokeWidth: 2},
				});
			}
		});
	}

	return getDagreLayout(rawNodes, rawEdges, 'LR');
}

// ─── Custom node / edge types ─────────────────────────────────────────────────

const nodeTypes: NodeTypes = {
	composeService: ComposeServiceNode as any,
	composeBackup: ComposeBackupNode as any,
	composeCron: ComposeCronNode as any,
	composeDomain: ComposeDomainNode as any,
};

const edgeTypes = {
	composeEdge: ComposeEdge as any,
};

// ─── Inner Canvas Component ───────────────────────────────────────────────────

interface CanvasInnerProps {
	services: ComposeService[];
	backups?: VolumeBackupData[];
	schedules?: CronJobData[];
	domains?: ComposeDomainData[];
	gitBuildPath?: string;
	isGitSource?: boolean;
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
	onViewDeployLogs?: (service: ComposeService) => void;
	onEditDomain?: (domainData: any) => void;
	onDeleteDomain?: (domainData: any) => void;
	onEditSchedule?: (scheduleData: any) => void;
	onDeleteSchedule?: (scheduleData: any) => void;
	onEditBackup?: (backupData: any) => void;
	onDeleteBackup?: (backupData: any) => void;
}

function CanvasInner({
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
		targetType: 'service' | 'domain' | 'schedule' | 'backup';
		targetData: any;
	} | null>(null);

	useEffect(() => {
		const actionHandlers = {
			onAddDomain,
			onAddSchedule,
			onAddBackup,
			onOpenTerminal,
			onViewLogs,
		};
		const {nodes: n, edges: e} = buildGraph(
			services,
			backups,
			schedules,
			domains,
			actionHandlers,
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
			let targetType: 'service' | 'domain' | 'schedule' | 'backup' =
				'service';
			if (node.type === 'composeDomain') targetType = 'domain';
			else if (node.type === 'composeCron') targetType = 'schedule';
			else if (node.type === 'composeBackup') targetType = 'backup';
			else if (node.type === 'composeService') targetType = 'service';

			setContextMenu({
				x: event.clientX,
				y: event.clientY,
				targetType,
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
				nodeTypes={nodeTypes}
				edgeTypes={edgeTypes}
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

				{/* Legend Badge */}
				{(services || []).length > 0 && (
					<div
						style={{
							position: 'absolute',
							top: 12,
							left: 16,
							zIndex: 5,
							display: 'flex',
							alignItems: 'center',
							gap: 12,
							padding: '6px 12px',
							borderRadius: 8,
							background: 'rgba(22,27,34,0.85)',
							backdropFilter: 'blur(8px)',
							border: '1px solid rgba(255,255,255,0.08)',
							fontSize: 10.5,
							color: '#8b949e',
						}}>
						<div style={{display: 'flex', alignItems: 'center', gap: 5}}>
							<span
								style={{
									width: 8,
									height: 8,
									borderRadius: 2,
									background: '#30363d',
								}}
							/>{' '}
							Service (Click for Actions)
						</div>
						<div style={{display: 'flex', alignItems: 'center', gap: 5}}>
							<span
								style={{
									width: 8,
									height: 8,
									borderRadius: 2,
									background: '#10b981',
								}}
							/>{' '}
							Domain (Click to Delete)
						</div>
						<div style={{display: 'flex', alignItems: 'center', gap: 5}}>
							<span
								style={{
									width: 8,
									height: 8,
									borderRadius: 2,
									background: '#38bdf8',
								}}
							/>{' '}
							Volume Backup (Click to Delete)
						</div>
						<div style={{display: 'flex', alignItems: 'center', gap: 5}}>
							<span
								style={{
									width: 8,
									height: 8,
									borderRadius: 2,
									background: '#c084fc',
								}}
							/>{' '}
							CronJob (Click to Delete)
						</div>
					</div>
				)}

				{/* Empty state overlay */}
				{(services || []).length === 0 && (
					<div
						style={{
							position: 'absolute',
							inset: 0,
							zIndex: 5,
							display: 'flex',
							flexDirection: 'column',
							alignItems: 'center',
							justifyContent: 'center',
							gap: 8,
							pointerEvents: 'none',
							padding: 24,
							textAlign: 'center',
						}}>
						<svg
							viewBox="0 0 24 24"
							style={{
								width: 32,
								height: 32,
								opacity: 0.25,
								fill: '#e6edf3',
							}}>
							<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
						</svg>
						<p
							style={{
								color: '#e6edf3',
								fontSize: 13,
								fontWeight: 600,
								margin: 0,
							}}>
							{isGitSource
								? `Configured File: ${gitBuildPath || 'docker-compose.yml'}`
								: 'No services found'}
						</p>
						<p
							style={{
								color: '#8b949e',
								fontSize: 11.5,
								maxWidth: 360,
								margin: 0,
								lineHeight: 1.5,
							}}>
							{isGitSource
								? 'Deploy/Build the project to fetch this file from Git and auto-wire your service dependency graph.'
								: 'Save a valid docker-compose.yml to visualize the dependency graph'}
						</p>
					</div>
				)}
			</ReactFlow>

			{/* Context Menu */}
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

// ─── Public export ────────────────────────────────────────────────────────────

export interface ComposeVisualizerProps {
	composeFile?: string | null;
	stackName?: string;
	customServices?: ComposeService[];
	backups?: VolumeBackupData[];
	schedules?: CronJobData[];
	domains?: ComposeDomainData[];
	gitBuildPath?: string;
	isGitSource?: boolean;
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
	onViewDeployLogs?: (service: ComposeService) => void;
	onEditDomain?: (domainData: any) => void;
	onDeleteDomain?: (domainData: any) => void;
	onEditSchedule?: (scheduleData: any) => void;
	onDeleteSchedule?: (scheduleData: any) => void;
	onEditBackup?: (backupData: any) => void;
	onDeleteBackup?: (backupData: any) => void;
}

export function ComposeVisualizer({
	composeFile,
	stackName,
	customServices,
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
}: ComposeVisualizerProps) {
	const services = useMemo(() => {
		if (customServices && customServices.length > 0) return customServices;
		const parsed = parseComposeFile(composeFile ?? '');
		if (parsed.length > 0) return parsed;

		if (stackName && stackName.trim()) {
			return [
				{
					name: stackName.trim(),
					image: 'docker-compose stack',
					dependsOn: [],
					envVars: {},
					volumes: [],
					ports: [],
				},
			];
		}
		return [];
	}, [composeFile, customServices, stackName]);

	return (
		<div
			style={{
				height: 'calc(100vh - 320px)',
				minHeight: 410,
				borderRadius: 14,
				overflow: 'hidden',
				border: '1px solid rgba(255,255,255,0.08)',
				background: '#0d1117',
			}}>
			<ReactFlowProvider>
				<CanvasInner
					services={services}
					backups={backups}
					schedules={schedules}
					domains={domains}
					gitBuildPath={gitBuildPath}
					isGitSource={isGitSource}
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
			</ReactFlowProvider>
		</div>
	);
}
