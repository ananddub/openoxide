import {useMemo, useState} from 'react';
import {Plus, RefreshCw, Search, Server} from 'lucide-react';
import {toast} from 'sonner';
import {cn} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	ListCard,
	ListEmpty,
	ListSkeleton,
	Toolbar,
	ToolbarSearch,
} from '#/components/settings/list-card';
import type {RemoteServerResponse} from '#/types/api-helpers';
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
	servers: RemoteServerResponse[];
	selectedServerId: string;
	onSelectServer: (id: string) => void;
	isRefreshing: boolean;
	onRefresh: () => void;
	isSwarmActive: boolean;
	onOpenTokens: () => void;
	onPromote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onDemote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onSetAvailability: (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => void;
	onRemoveNode: (nodeId: string, node?: TaggedSwarmNode) => void;
}

function engineLabel(
	servers: RemoteServerResponse[],
	selectedServerId: string,
) {
	if (selectedServerId === 'all') return 'All Engines';
	if (selectedServerId === 'local') return 'Local Engine';
	const srv = servers.find(s => String(s.id) === selectedServerId);
	return srv ? srv.name : 'Select Engine';
}

/** Nodes surface — house toolbar (search + filters left, actions right) over ONE bordered list card. */
export function SwarmNodesList({
	nodes,
	isLoading,
	servers,
	selectedServerId,
	onSelectServer,
	isRefreshing,
	onRefresh,
	isSwarmActive,
	onOpenTokens,
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

	// Docker refuses to demote a swarm's last manager, so disable that action up front.
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

	return (
		<div className="flex flex-col gap-3">
			<Toolbar
				action={
					<>
						<Button
							variant="outline"
							onClick={onRefresh}
							disabled={isRefreshing}
							className="h-9 gap-1.5">
							<RefreshCw
								className={cn(
									'size-3.5',
									isRefreshing && 'animate-spin',
								)}
							/>
							Refresh
						</Button>
						{isSwarmActive && (
							<Button
								onClick={onOpenTokens}
								className="h-9 gap-1.5">
								<Plus className="size-3.5" />
								New Node
							</Button>
						)}
					</>
				}>
				<ToolbarSearch
					placeholder="Search by hostname or node ID…"
					value={search}
					onChange={setSearch}
					className="sm:max-w-xs"
				/>
				<Select
					value={roleFilter}
					onValueChange={v => setRoleFilter(v as RoleFilter)}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<SelectValue>
							{roleFilter === 'All' ? 'All Roles' : roleFilter}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card">
						{ROLE_FILTERS.map(f => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Roles' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
				<Select
					value={selectedServerId}
					onValueChange={val => val && onSelectServer(val)}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<Server className="size-3.5 shrink-0 text-muted-foreground" />
						<SelectValue>
							{engineLabel(servers, selectedServerId)}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="all">
							All Clusters &amp; Remote Servers
						</SelectItem>
						<SelectItem value="local">
							Local Engine (Docker)
						</SelectItem>
						{servers.map(srv => (
							<SelectItem key={srv.id} value={String(srv.id)}>
								{srv.name} ({srv.ip_address})
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Toolbar>

			{isLoading ? (
				<ListSkeleton rows={3} />
			) : !nodes || nodes.length === 0 ? (
				<ListEmpty
					icon={Server}
					title="No Swarm nodes discovered"
					description="Make sure Docker Swarm is initialized on this host engine."
					action={
						isSwarmActive ? (
							<Button
								onClick={onOpenTokens}
								className="h-9 gap-1.5 text-xs font-semibold">
								<Plus className="size-3.5" />
								New Node
							</Button>
						) : undefined
					}
				/>
			) : filtered.length === 0 ? (
				<ListEmpty
					icon={Search}
					title="No matching nodes"
					description="No nodes match your current search or role filter."
					action={
						<Button
							variant="ghost"
							onClick={() => {
								setSearch('');
								setRoleFilter('All');
							}}
							className="text-xs font-semibold text-primary">
							Clear filters
						</Button>
					}
				/>
			) : (
				<ListCard>
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
				</ListCard>
			)}

			{hasFilters && filtered.length > 0 && (
				<p className="px-1 text-xs text-muted-foreground">
					Showing {filtered.length} of {nodes.length} nodes
				</p>
			)}
		</div>
	);
}
