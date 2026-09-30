import {
	Check,
	Copy,
	Crown,
	Globe,
	Server,
	ShieldAlert,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {Badge} from '#/components/ui/badge';
import {RowTile} from '#/components/settings/list-card';
import {SwarmNodeMenu} from './swarm-node-menu';
import {nodeRole, type TaggedSwarmNode} from './swarm-node-shared';

/** Icon tile — manager nodes carry the crown. */
export function SwarmNodeTile({node}: {node: TaggedSwarmNode}) {
	return nodeRole(node).isManager ? (
		<RowTile>
			<Crown className="size-4 text-amber-500" />
		</RowTile>
	) : (
		<RowTile>
			<Server className="size-4" />
		</RowTile>
	);
}

/** Role chip — read from the node spec, so a manager stays a manager when unreachable. */
export function SwarmRoleBadge({node}: {node: TaggedSwarmNode}) {
	const {isManager, isLeader, isUnreachable} = nodeRole(node);

	if (!isManager) {
		return (
			<Badge
				variant="secondary"
				className="shrink-0 py-0 text-[10px] font-bold uppercase">
				Worker
			</Badge>
		);
	}

	return (
		<Badge
			variant="outline"
			className={cn(
				'shrink-0 gap-1 px-1.5 py-0 text-[10px] font-bold uppercase',
				isUnreachable
					? 'border-rose-500/30 bg-rose-500/10 text-rose-500'
					: 'border-amber-500/30 bg-amber-500/10 text-amber-500',
			)}>
			{isUnreachable ? (
				<ShieldAlert className="size-2.5" />
			) : (
				<Crown className="size-2.5 fill-amber-500/20" />
			)}
			{isLeader
				? 'Leader'
				: isUnreachable
					? 'Manager (Unreachable)'
					: 'Manager'}
		</Badge>
	);
}

/** Runtime state chip — the only status indicator on a node. */
export function SwarmStatusBadge({node}: {node: TaggedSwarmNode}) {
	const isReady = (node.status || '').toLowerCase() === 'ready';

	return (
		<Badge
			variant="outline"
			className={cn(
				'shrink-0 gap-1.5 px-1.5 py-0 text-[10px] font-medium',
				isReady
					? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
					: 'border-rose-500/30 bg-rose-500/10 text-rose-500',
			)}>
			<span
				className={cn(
					'size-1.5 rounded-full',
					isReady ? 'bg-emerald-500' : 'bg-rose-500',
				)}
			/>
			{isReady ? 'Ready' : 'Down'}
		</Badge>
	);
}

/** Secondary line: node ID · address · scheduling state. */
export function SwarmNodeMeta({
	node,
	availability,
}: {
	node: TaggedSwarmNode;
	availability: string;
}) {
	const nodeId = node.id || '';

	return (
		<span className="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px]">
			<span>{nodeId ? nodeId.slice(0, 14) : 'No ID'}</span>
			{node.ip_address && (
				<span className="flex items-center gap-1">
					<Globe className="size-3" />
					{node.ip_address}
				</span>
			)}
			<span className="uppercase">{availability}</span>
		</span>
	);
}

interface SwarmNodeActionsProps {
	node: TaggedSwarmNode;
	isLastManager: boolean;
	copiedId: string | null;
	onCopyId: (id: string) => void;
	onPromote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onDemote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onSetAvailability: (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => void;
	onRemoveNode: (nodeId: string, node?: TaggedSwarmNode) => void;
}

/** Copy ID + the 3-dot menu, used by both the row and the grid card. */
export function SwarmNodeActions({
	node,
	isLastManager,
	copiedId,
	onCopyId,
	onPromote,
	onDemote,
	onSetAvailability,
	onRemoveNode,
}: SwarmNodeActionsProps) {
	const nodeId = node.id || '';
	const {isManager} = nodeRole(node);
	const availability = (node.availability || 'active').toLowerCase();

	return (
		<>
			{nodeId && (
				<button
					type="button"
					onClick={() => onCopyId(nodeId)}
					title="Copy Node ID"
					className="flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted/60 hover:text-foreground">
					{copiedId === nodeId ? (
						<Check className="size-4 text-emerald-500" />
					) : (
						<Copy className="size-4" />
					)}
				</button>
			)}
			<SwarmNodeMenu
				node={node}
				nodeId={nodeId}
				isManager={isManager}
				isLastManager={isLastManager}
				availability={availability}
				onPromote={onPromote}
				onDemote={onDemote}
				onSetAvailability={onSetAvailability}
				onRemoveNode={onRemoveNode}
			/>
		</>
	);
}
