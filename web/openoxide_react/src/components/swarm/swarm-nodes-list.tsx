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
	onSetAvailability: (nodeId: string, availability: string, node?: TaggedSwarmNode) => void;
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
	const isUnreachable = isManager && (node.reachability || '').toLowerCase() === 'unreachable';
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
					<div key={i} className="flex items-center gap-4 px-4 py-4 border border-border rounded-lg">
						<Skeleton className="w-8 h-8 rounded-full shrink-0" />
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
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search by hostname or node ID…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="pl-8 h-8 text-xs"
					/>
				</div>

				<Select value={roleFilter} onValueChange={v => setRoleFilter(v as RoleFilter)}>
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
				<div className="flex flex-col items-center justify-center gap-3 py-20 text-center border border-dashed border-border rounded-lg">
					<div className="w-12 h-12 rounded-full bg-muted flex items-center justify-center">
						<Server className="w-5 h-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">No Swarm nodes discovered</p>
						<p className="text-xs text-muted-foreground mt-0.5">
							Make sure Docker Swarm is initialized on this host engine.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 py-14 text-center border border-dashed border-border rounded-lg">
					<Search className="w-5 h-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">No nodes match your filter</p>
				</div>
			) : (
				/* ── List ── */
				<div className="border border-border rounded-lg overflow-hidden divide-y divide-border">
					{filtered.map(node => {
						const {isManager, isLeader, isUnreachable} = nodeRole(node);

						const isReady = (node.status || '').toLowerCase() === 'ready';
						const availability = (node.availability || 'active').toLowerCase();

						const hostname = node.hostname || 'Swarm Node';
						const nodeId = node.id || '';
						const serverLabel = node._serverName || null;
						const clusterKey = String(node._serverId ?? 'local');
						const isLastManager = isManager && (managersPerCluster.get(clusterKey) ?? 0) <= 1;

						const dotCls = isReady ? 'bg-emerald-500' : 'bg-rose-500';

						return (
							<div
								key={nodeId}
								className="group flex items-center gap-4 px-4 py-3.5 bg-card hover:bg-accent/30 transition-colors"
							>
								{/* Icon + status dot */}
								<div className="relative shrink-0">
									<div className="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
										{isManager ? (
											<Crown className="w-4 h-4 text-amber-500" />
										) : (
											<Server className="w-4 h-4 text-foreground/70" />
										)}
									</div>
									<span className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-background ${dotCls}`} />
								</div>

								{/* Info */}
								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2 mb-0.5">
										<span className="text-sm font-medium text-foreground truncate">{hostname}</span>
										{isManager ? (
											<Badge
												variant="outline"
												className={`shrink-0 text-[10px] uppercase font-bold gap-1 px-1.5 py-0 ${
													isUnreachable
														? 'text-rose-500 border-rose-500/30 bg-rose-500/10'
														: 'text-amber-500 border-amber-500/30 bg-amber-500/10'
												}`}
											>
												{isUnreachable ? (
													<ShieldAlert className="w-2.5 h-2.5" />
												) : (
													<Crown className="w-2.5 h-2.5 fill-amber-500/20" />
												)}
												{isLeader ? 'Leader' : isUnreachable ? 'Manager (Unreachable)' : 'Manager'}
											</Badge>
										) : (
											<Badge variant="secondary" className="shrink-0 text-[10px] uppercase font-mono py-0">
												Worker
											</Badge>
										)}
										<Badge variant="outline" className="shrink-0 text-[10px] font-mono py-0">
											{availability}
										</Badge>
										{serverLabel && (
											<span className="text-[10px] text-muted-foreground bg-muted/50 px-1.5 py-0.5 rounded shrink-0">
												{serverLabel}
											</span>
										)}
									</div>
									<div className="flex items-center gap-2 text-[11px] font-mono text-muted-foreground truncate">
										<span className="truncate">{nodeId ? nodeId.slice(0, 14) : 'No ID'}</span>
										{node.ip_address && (
											<span className="flex items-center gap-1 shrink-0">
												· <Globe className="w-3 h-3" /> {node.ip_address}
											</span>
										)}
									</div>
								</div>

								{/* Hover actions */}
								<div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
									{nodeId && (
										<button
											onClick={() => handleCopyId(nodeId)}
											title="Copy Node ID"
											className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
										>
											{copiedId === nodeId ? <Check className="w-4 h-4 text-emerald-500" /> : <Copy className="w-4 h-4" />}
										</button>
									)}
								</div>

								<Separator orientation="vertical" className="h-5 opacity-0 group-hover:opacity-100 transition-opacity" />

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger render={<button className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer" />}>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										{isManager ? (
											<DropdownMenuItem
												disabled={isLastManager}
												title={isLastManager ? "Can't demote the last manager of a swarm" : undefined}
												className="gap-2 cursor-pointer"
												onClick={() => onDemote(nodeId, node)}
											>
												<ArrowDownRight className="w-3.5 h-3.5" />
												{isLastManager ? 'Demote to Worker (last manager)' : 'Demote to Worker'}
											</DropdownMenuItem>
										) : (
											<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onPromote(nodeId, node)}>
												<ArrowUpRight className="w-3.5 h-3.5" /> Promote to Manager
											</DropdownMenuItem>
										)}
										<DropdownMenuSeparator />
										<DropdownMenuItem
											disabled={availability === 'active'}
											className="gap-2 cursor-pointer"
											onClick={() => onSetAvailability(nodeId, 'active', node)}
										>
											<ShieldCheck className="w-3.5 h-3.5" /> Set Active
										</DropdownMenuItem>
										<DropdownMenuItem
											disabled={availability === 'pause'}
											className="gap-2 cursor-pointer"
											onClick={() => onSetAvailability(nodeId, 'pause', node)}
										>
											Set Pause
										</DropdownMenuItem>
										<DropdownMenuItem
											disabled={availability === 'drain'}
											className="gap-2 cursor-pointer"
											onClick={() => onSetAvailability(nodeId, 'drain', node)}
										>
											Set Drain
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="gap-2 cursor-pointer text-destructive focus:text-destructive"
											onClick={() => onRemoveNode(nodeId, node)}
										>
											<Trash2 className="w-3.5 h-3.5" /> Remove Node
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
				<p className="text-xs text-muted-foreground px-1">
					Showing {filtered.length} of {nodes.length} nodes
				</p>
			)}
		</div>
	);
}
