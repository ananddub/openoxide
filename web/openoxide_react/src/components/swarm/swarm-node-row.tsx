import {Check, Copy, Crown, Globe, Server, ShieldAlert} from 'lucide-react';
import {cn} from '#/api/utils';
import {Badge} from '#/components/ui/badge';
import {ListRow, RowTile} from '#/components/settings/list-card';
import {nodeRole, type TaggedSwarmNode} from './swarm-node-shared';
import {SwarmNodeMenu} from './swarm-node-menu';

interface SwarmNodeRowProps {
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

/** A single swarm node row: identity, role + status chips, hover actions. */
export function SwarmNodeRow({
	node,
	isLastManager,
	copiedId,
	onCopyId,
	onPromote,
	onDemote,
	onSetAvailability,
	onRemoveNode,
}: SwarmNodeRowProps) {
	const {isManager, isLeader, isUnreachable} = nodeRole(node);
	const isReady = (node.status || '').toLowerCase() === 'ready';
	const availability = (node.availability || 'active').toLowerCase();
	const nodeId = node.id || '';

	const roleBadge = isManager ? (
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
	) : (
		<Badge
			variant="secondary"
			className="shrink-0 py-0 text-[10px] font-bold uppercase">
			Worker
		</Badge>
	);

	const statusBadge = (
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

	return (
		<ListRow
			icon={
				<RowTile>
					{isManager ? (
						<Crown className="size-[18px] text-amber-500" />
					) : (
						<Server className="size-[18px] text-foreground/70" />
					)}
				</RowTile>
			}
			title={node.hostname || 'Swarm Node'}
			badges={
				<>
					{roleBadge}
					{statusBadge}
				</>
			}
			subtitle={
				<span className="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px]">
					<span>{nodeId ? nodeId.slice(0, 14) : 'No ID'}</span>
					{node.ip_address && (
						<span className="flex items-center gap-1">
							<Globe className="size-3" />
							{node.ip_address}
						</span>
					)}
					<span className="uppercase">{availability}</span>
					{node._serverName && (
						<span className="rounded bg-muted/50 px-1.5 py-0.5 font-sans text-[10px]">
							{node._serverName}
						</span>
					)}
				</span>
			}>
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
		</ListRow>
	);
}
