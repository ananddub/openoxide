import {load as yamlLoad} from 'js-yaml';
import type {Node, NodeTypes} from '@xyflow/react';
import {AppCanvasNode, DbCanvasNode} from './nodes/railway-nodes';
import {ComposeGroupNode} from './nodes/compose-group-node';
import {ComposeServiceMiniNode} from './nodes/compose-service-mini-node';
import {S3CanvasNode} from './nodes/s3-canvas-node';
import {CustomEdge} from './edges/custom-edge';

export const railwayNodeTypes: NodeTypes = {
	appNode: AppCanvasNode as any,
	dbNode: DbCanvasNode as any,
	s3Node: S3CanvasNode as any,
	composeGroupNode: ComposeGroupNode as any,
	composeMiniServiceNode: ComposeServiceMiniNode as any,
};
export const railwayEdgeTypes = {customEdge: CustomEdge as any};

// ─── Helpers ─────────────────────────────────────────────────────────────────

export function parseServices(yaml?: string) {
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

export function buildNodes(
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
