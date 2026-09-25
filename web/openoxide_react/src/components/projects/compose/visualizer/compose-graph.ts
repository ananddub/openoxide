import type {Node, Edge, NodeTypes} from '@xyflow/react';
import {getDagreLayout} from './compose-dagre';
import type {ComposeService} from './compose-parse';
import type {VolumeBackupData} from './compose-backup-node';
import type {CronJobData} from './compose-cron-node';
import type {ComposeDomainData} from './compose-domain-node';
import {ComposeServiceNode} from './compose-service-node';
import {ComposeEdge} from './compose-edge';
import {ComposeBackupNode} from './compose-backup-node';
import {ComposeCronNode} from './compose-cron-node';
import {ComposeDomainNode} from './compose-domain-node';

export type ServiceActionHandlers = {
	onAddDomain?: (s: ComposeService) => void;
	onAddSchedule?: (s: ComposeService) => void;
	onAddBackup?: (s: ComposeService) => void;
	onOpenTerminal?: (s: ComposeService) => void;
	onViewLogs?: (s: ComposeService) => void;
	readOnly?: boolean;
	monitorId?: number;
	monitorType?: 'application' | 'database' | 'compose';
};

// ─── Build ReactFlow Elements ─────────────────────────────────────────────────

export function buildGraph(
	services: ComposeService[] = [],
	backups: VolumeBackupData[] = [],
	schedules: CronJobData[] = [],
	domains: ComposeDomainData[] = [],
	actionHandlers?: ServiceActionHandlers,
): {nodes: Node[]; edges: Edge[]} {
	const rawNodes: Node[] = [];
	const rawEdges: Edge[] = [];

	const safeServices = Array.isArray(services) ? services : [];
	const serviceNames = new Set(safeServices.map(s => s.name));
	const edgeKeys = new Set<string>();

	const addEdge = (edge: Edge) => {
		const key = `${edge.source}->${edge.target}`;
		if (!edgeKeys.has(key)) {
			edgeKeys.add(key);
			rawEdges.push(edge);
		}
	};

	/** Resolve which service a satellite node attaches to (first service fallback). */
	const attachTarget = (raw?: string) =>
		raw && serviceNames.has(raw)
			? raw
			: serviceNames.size > 0
				? safeServices[0]?.name
				: undefined;

	// 1. Service Nodes
	safeServices.forEach(svc => {
		rawNodes.push({
			id: svc.name,
			type: 'composeService',
			data: {...svc, ...actionHandlers},
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
				});
			}
		});
	});

	// 2. Domain Nodes
	(Array.isArray(domains) ? domains : []).forEach((d, idx) => {
		const domId = `domain-${d.domain || (d as any).id || idx}`;
		rawNodes.push({
			id: domId,
			type: 'composeDomain',
			data: d,
			position: {x: 0, y: 0},
		});

		const target = attachTarget(
			d.service || (d as any).service_name || (d as any).app_name,
		);
		if (target) {
			addEdge({
				id: `e-${target}-${domId}`,
				source: target,
				target: domId,
				type: 'composeEdge',
				animated: true,
			});
		}
	});

	// 3. Volume Backup Nodes
	(Array.isArray(backups) ? backups : []).forEach((b, idx) => {
		const bId = `backup-${(b as any).id || b.name || idx}`;
		rawNodes.push({
			id: bId,
			type: 'composeBackup',
			data: b,
			position: {x: 0, y: 0},
		});

		const target = attachTarget(
			b.service || (b as any).service_name || (b as any).app_name,
		);
		if (target) {
			addEdge({
				id: `e-${target}-${bId}`,
				source: target,
				target: bId,
				type: 'composeEdge',
				animated: true,
			});
		}
	});

	// 4. CronJob Nodes
	(Array.isArray(schedules) ? schedules : []).forEach((c, idx) => {
		const cId = `cron-${(c as any).id || c.name || idx}`;
		rawNodes.push({
			id: cId,
			type: 'composeCron',
			data: c,
			position: {x: 0, y: 0},
		});

		const target = attachTarget(
			c.target || (c as any).service_name || (c as any).app_name,
		);
		if (target) {
			addEdge({
				id: `e-${cId}-${target}`,
				source: cId,
				target,
				type: 'composeEdge',
				animated: true,
			});
		}
	});

	return getDagreLayout(rawNodes, rawEdges, 'LR');
}

// ─── Custom node / edge types ─────────────────────────────────────────────────

export const composeNodeTypes: NodeTypes = {
	composeService: ComposeServiceNode as any,
	composeBackup: ComposeBackupNode as any,
	composeCron: ComposeCronNode as any,
	composeDomain: ComposeDomainNode as any,
};

export const composeEdgeTypes = {
	composeEdge: ComposeEdge as any,
};
