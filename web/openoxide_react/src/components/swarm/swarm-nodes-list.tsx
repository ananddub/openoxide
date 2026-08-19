import {useState, useMemo} from 'react';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	Server,
	MoreVertical,
	Check,
	Copy,
	Crown,
	Search,
	Globe,
	ShieldCheck,
	ShieldAlert,
	Trash2,
	ArrowUpRight,
	ArrowDownRight,
} from 'lucide-react';
import {toast} from 'sonner';

import type {SwarmNode} from '#/types/api-helpers';

export type TaggedSwarmNode = SwarmNode & {
	_serverId?: number;
	_serverName?: string;
};

interface SwarmNodesListProps {
	nodes: TaggedSwarmNode[];
	isLoading: boolean;
	onPromote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onDemote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onSetAvailability: (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => void;
	onRemoveNode: (nodeId: string, node?: TaggedSwarmNode) => void;
}

const ROLE_FILTERS = ['All', 'Managers', 'Workers'] as const;
type RoleFilter = (typeof ROLE_FILTERS)[number];

/**
 * `role`/`is_leader`/`reachability` come from `docker node inspect`'s
 * `Spec.Role` and `ManagerStatus` — the authoritative config values, not a
 * string derived from reachability. A manager stays "manager" even when
 * Docker can't currently reach it (reachability: "unreachable").
 */
function nodeRole(node: SwarmNode) {
	const isManager = node.role === 'manager';
	const isLeader = node.is_leader;
	const isUnreachable =
		isManager && (node.reachability || '').toLowerCase() === 'unreachable';
	return {isManager, isLeader, isUnreachable};
}

export function SwarmNodesList({
	nodes,
	isLoading,
	onPromote,
	onDemote,
	onSetAvailability,
	onRemoveNode,
}: SwarmNodesListProps) {
	const [copiedId, setCopiedId] = useState<string | null>(null);
	const [search, setSearch] = useState('');
	const [roleFilter, setRoleFilter] = useState<RoleFilter>('All');

	const handleCopyId = (id: string) => {
		navigator.clipboard.writeText(id);
		setCopiedId(id);
		toast.success('Node ID copied to clipboard');
		setTimeout(() => setCopiedId(null), 2000);
	};

	const filtered = useMemo(() => {
		return nodes.filter(node => {
			const hostname = node.hostname || '';
			const nodeId = node.id || '';

			const matchSearch =
				hostname.toLowerCase().includes(search.toLowerCase()) ||
				nodeId.toLowerCase().includes(search.toLowerCase());

			const {isManager} = nodeRole(node);
			const matchRole =
				roleFilter === 'All' ||
				(roleFilter === 'Managers' && isManager) ||
				(roleFilter === 'Workers' && !isManager);

			return matchSearch && matchRole;
		});
	}, [nodes, search, roleFilter]);

	// Docker refuses to demote a swarm's last manager (it would leave the
	// cluster without anyone to run Raft consensus). Count managers per
	// cluster so we can disable that action up front instead of surfacing a
	// raw Docker RPC error after the click.
	const managersPerCluster = useMemo(() => {
		const counts = new Map<string, number>();
		for (const n of nodes) {
			if (!nodeRole(n).isManager) continue;
			const key = String(n._serverId ?? 'local');
			counts.set(key, (counts.get(key) ?? 0) + 1);
		}
		return counts;
	}, [nodes]);

	const hasFilters = search !== '' || roleFilter !== 'All';

	/* ── Loading ── */
	if (isLoading) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div
						key={i}
						className="flex items-center gap-4 rounded-lg border border-border px-4 py-4">
						<Skeleton className="h-8 w-8 shrink-0 rounded-full" />
						<div className="flex-1 space-y-2">
							<Skeleton className="h-3.5 w-40" />
							<Skeleton className="h-3 w-56" />
						</div>
						<Skeleton className="h-5 w-16 rounded-full" />
					</div>
				))}
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
				<div className="relative flex-1">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						placeholder="Search by hostname or node ID…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-8 pl-8 text-xs"
					/>
				</div>

				<Select
					value={roleFilter}
					onValueChange={v => setRoleFilter(v as RoleFilter)}>
					<SelectTrigger size="sm" className="h-8 w-36 text-xs">
						<SelectValue placeholder="Role" />
					</SelectTrigger>
					<SelectContent>
						{ROLE_FILTERS.map(f => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Roles' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty state ── */}
			{!nodes || nodes.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-border py-20 text-center">
					<div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<Server className="h-5 w-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">
							No Swarm nodes discovered
						</p>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Make sure Docker Swarm is initialized on this host engine.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-border py-14 text-center">
					<Search className="h-5 w-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No nodes match your filter
					</p>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border overflow-hidden rounded-lg border border-border">
					{filtered.map(node => {
						const {isManager, isLeader, isUnreachable} = nodeRole(node);

						const isReady = (node.status || '').toLowerCase() === 'ready';
						const availability = (
							node.availability || 'active'
						).toLowerCase();

						const hostname = node.hostname || 'Swarm Node';
						const nodeId = node.id || '';
						const serverLabel = node._serverName || null;
						const clusterKey = String(node._serverId ?? 'local');
						const isLastManager =
							isManager && (managersPerCluster.get(clusterKey) ?? 0) <= 1;

						const dotCls = isReady ? 'bg-emerald-500' : 'bg-rose-500';

						return (
							<div
								key={nodeId}
								className="group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30">
								{/* Icon + status dot */}
								<div className="relative shrink-0">
									<div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
										{isManager ? (
											<Crown className="h-4 w-4 text-amber-500" />
										) : (
											<Server className="h-4 w-4 text-foreground/70" />
										)}
									</div>
									<span
										className={`absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background ${dotCls}`}
									/>
								</div>

								{/* Info */}
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

								{/* Hover actions */}
								<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
									{nodeId && (
										<button
											onClick={() => handleCopyId(nodeId)}
											title="Copy Node ID"
											className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
											{copiedId === nodeId ? (
												<Check className="h-4 w-4 text-emerald-500" />
											) : (
												<Copy className="h-4 w-4" />
											)}
										</button>
									)}
								</div>

								<Separator
									orientation="vertical"
									className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
								/>

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
										}>
										<MoreVertical className="h-4 w-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										{isManager ? (
											<DropdownMenuItem
												disabled={isLastManager}
												title={
													isLastManager
														? "Can't demote the last manager of a swarm"
														: undefined
												}
												className="cursor-pointer gap-2"
												onClick={() => onDemote(nodeId, node)}>
												<ArrowDownRight className="h-3.5 w-3.5" />
												{isLastManager
													? 'Demote to Worker (last manager)'
													: 'Demote to Worker'}
											</DropdownMenuItem>
										) : (
											<DropdownMenuItem
												className="cursor-pointer gap-2"
												onClick={() => onPromote(nodeId, node)}>
												<ArrowUpRight className="h-3.5 w-3.5" /> Promote to
												Manager
											</DropdownMenuItem>
										)}
										<DropdownMenuSeparator />
										<DropdownMenuItem
											disabled={availability === 'active'}
											className="cursor-pointer gap-2"
											onClick={() =>
												onSetAvailability(nodeId, 'active', node)
											}>
											<ShieldCheck className="h-3.5 w-3.5" /> Set Active
										</DropdownMenuItem>
										<DropdownMenuItem
											disabled={availability === 'pause'}
											className="cursor-pointer gap-2"
											onClick={() =>
												onSetAvailability(nodeId, 'pause', node)
											}>
											Set Pause
										</DropdownMenuItem>
										<DropdownMenuItem
											disabled={availability === 'drain'}
											className="cursor-pointer gap-2"
											onClick={() =>
												onSetAvailability(nodeId, 'drain', node)
											}>
											Set Drain
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="cursor-pointer gap-2 text-destructive focus:text-destructive"
											onClick={() => onRemoveNode(nodeId, node)}>
											<Trash2 className="h-3.5 w-3.5" /> Remove Node
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						);
					})}
				</div>
			)}

			{/* Result count */}
			{hasFilters && filtered.length > 0 && (
				<p className="px-1 text-xs text-muted-foreground">
					Showing {filtered.length} of {nodes.length} nodes
				</p>
			)}
		</div>
	);
}
