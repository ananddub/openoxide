import {Badge} from '#/components/ui/badge';
import {
	Crown,
	Server,
	Check,
	Copy,
	Globe,
	ShieldAlert,
} from 'lucide-react';
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

/** A single swarm node row: identity, role badges, status and actions. */
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
	const hostname = node.hostname || 'Swarm Node';
	const nodeId = node.id || '';
	const serverLabel = node._serverName || null;
	const dotCls = isReady ? 'bg-emerald-500' : 'bg-rose-500';

	return (
		<div
			key={nodeId}
			className="group flex items-center gap-3.5 px-5 py-3 transition-colors hover:bg-muted/40">
			<div className="relative shrink-0">
				<div className="flex size-9 items-center justify-center rounded-xl bg-muted/60 transition-colors group-hover:bg-muted">
					{isManager ? (
						<Crown className="size-[18px] text-amber-500" />
					) : (
						<Server className="size-[18px] text-foreground/70" />
					)}
				</div>
				<span
					title={isReady ? 'Ready' : 'Not ready'}
					className={`absolute -right-0.5 -bottom-0.5 size-2.5 rounded-full border-2 border-card ${dotCls}`}
				/>
			</div>

			<div className="min-w-0 flex-1">
				<div className="mb-0.5 flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{hostname}
					</span>
					{isManager ? (
						<Badge
							variant="outline"
							className={`shrink-0 gap-1 px-1.5 py-0 text-[10px] font-bold uppercase ${
								isUnreachable
									? 'border-rose-500/30 bg-rose-500/10 text-rose-500'
									: 'border-amber-500/30 bg-amber-500/10 text-amber-500'
							}`}>
							{isUnreachable ? (
								<ShieldAlert className="h-2.5 w-2.5" />
							) : (
								<Crown className="h-2.5 w-2.5 fill-amber-500/20" />
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
							className="shrink-0 py-0 font-mono text-[10px] uppercase">
							Worker
						</Badge>
					)}
					<Badge
						variant="outline"
						className="shrink-0 py-0 font-mono text-[10px]">
						{availability}
					</Badge>
					{serverLabel && (
						<span className="shrink-0 rounded bg-muted/50 px-1.5 py-0.5 text-[10px] text-muted-foreground">
							{serverLabel}
						</span>
					)}
				</div>
				<div className="flex items-center gap-2 truncate font-mono text-[11px] text-muted-foreground">
					<span className="truncate">
						{nodeId ? nodeId.slice(0, 14) : 'No ID'}
					</span>
					{node.ip_address && (
						<span className="flex shrink-0 items-center gap-1">
							· <Globe className="h-3 w-3" /> {node.ip_address}
						</span>
					)}
				</div>
			</div>

			<span
				className={`inline-flex shrink-0 items-center gap-1.5 text-xs font-medium ${isReady ? 'text-emerald-500' : 'text-rose-500'}`}>
				<span
					className={`size-2.5 rounded-full border-2 ${isReady ? 'border-emerald-500' : 'border-rose-500'}`}
				/>
				<span className="hidden sm:inline">
					{isReady ? 'Ready' : 'Down'}
				</span>
			</span>

			<div className="flex shrink-0 items-center gap-2.5 opacity-0 transition-opacity group-hover:opacity-100">
				{nodeId && (
					<button
						onClick={() => onCopyId(nodeId)}
						title="Copy Node ID"
						className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
						{copiedId === nodeId ? (
							<Check className="size-4 text-emerald-500" />
						) : (
							<Copy className="size-4" />
						)}
					</button>
				)}
			</div>

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
		</div>
	);
}
