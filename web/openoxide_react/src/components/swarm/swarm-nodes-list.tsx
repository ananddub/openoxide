import {useState, useMemo} from 'react';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Server, Search} from 'lucide-react';
import {toast} from 'sonner';
import {
	ROLE_FILTERS,
	nodeRole,
	type RoleFilter,
	type TaggedSwarmNode,
} from './swarm-node-shared';
import {SwarmNodeRow} from './swarm-node-row';

export type {TaggedSwarmNode} from './swarm-node-shared';

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
						className="flex items-center gap-4 rounded-lg px-4 py-4">
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

			{!nodes || nodes.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-xl bg-card py-20 text-center ring-1 ring-border/50">
					<div className="flex size-12 items-center justify-center rounded-full bg-muted">
						<Server className="size-5 text-muted-foreground" />
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
				<div className="flex flex-col items-center justify-center gap-2 rounded-xl bg-card py-14 text-center ring-1 ring-border/50">
					<Search className="size-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No nodes match your filter
					</p>
					<button
						type="button"
						onClick={() => {
							setSearch('');
							setRoleFilter('All');
						}}
						className="cursor-pointer text-xs font-medium text-primary hover:underline">
						Clear filters
					</button>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border/50 overflow-hidden rounded-xl bg-card ring-1 ring-border/50">
					{filtered.map(node => {
						const clusterKey = String(node._serverId ?? 'local');
						return (
							<SwarmNodeRow
								key={node.id}
								node={node}
								isLastManager={
									nodeRole(node).isManager &&
									(managersPerCluster.get(clusterKey) ?? 0) <= 1
								}
								copiedId={copiedId}
								onCopyId={handleCopyId}
								onPromote={onPromote}
								onDemote={onDemote}
								onSetAvailability={onSetAvailability}
								onRemoveNode={onRemoveNode}
							/>
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
