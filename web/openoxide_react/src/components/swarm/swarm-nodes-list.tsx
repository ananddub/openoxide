import {useMemo, useState} from 'react';
import {Plus, Search, Server, ShieldAlert} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	GridSkeleton,
	ListCard,
	ListEmpty,
	ListSkeleton,
	Toolbar,
	ToolbarSearch,
	useListView,
	ViewGrid,
	ViewToggle,
} from '#/components/settings/list-card';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {
	ROLE_FILTERS,
	nodeRole,
	type RoleFilter,
	type TaggedSwarmNode,
} from './swarm-node-shared';
import {SwarmNodeRow} from './swarm-node-row';
import {SwarmNodeCard} from './swarm-node-card';

export type {TaggedSwarmNode} from './swarm-node-shared';

interface SwarmNodesListProps {
	nodes: TaggedSwarmNode[];
	isLoading: boolean;
	servers: RemoteServerResponse[];
	selectedServerId: string;
	onSelectServer: (id: string) => void;
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
	if (selectedServerId === 'local') return 'Local Engine';
	const srv = servers.find(s => String(s.id) === selectedServerId);
	return srv ? srv.name : 'Select Engine';
}

/** Nodes surface — house toolbar (search + role/engine filters + view toggle) over grid cards or ONE bordered list card. */
export function SwarmNodesList({
	nodes,
	isLoading,
	servers,
	selectedServerId,
	onSelectServer,
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
	const [view, setView] = useListView('swarm-nodes-view');

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
	const managerCount = useMemo(
		() => nodes.filter(n => nodeRole(n).isManager).length,
		[nodes],
	);

	const hasFilters = search !== '' || roleFilter !== 'All';

	const nodeProps = (node: TaggedSwarmNode) => ({
		node,
		isLastManager: nodeRole(node).isManager && managerCount <= 1,
		copiedId,
		onCopyId: handleCopyId,
		onPromote,
		onDemote,
		onSetAvailability,
		onRemoveNode,
	});

	return (
		<div className="flex w-full flex-col gap-3">
			<Toolbar
				action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					placeholder="Search by hostname or node ID…"
					value={search}
					onChange={setSearch}
					className="min-w-[220px] flex-1 sm:max-w-none"
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
				view === 'grid' ? (
					<GridSkeleton cards={3} />
				) : (
					<ListSkeleton rows={3} />
				)
			) : !nodes || nodes.length === 0 ? (
				<ListEmpty
					icon={isSwarmActive ? Server : ShieldAlert}
					title={
						isSwarmActive
							? 'No Swarm nodes discovered'
							: 'Docker Swarm is inactive'
					}
					description={
						isSwarmActive
							? 'This engine reports no cluster members yet. Add a node to get started.'
							: 'This host engine runs standalone. Initialize Docker Swarm or join an existing manager node.'
					}
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
			) : view === 'grid' ? (
				<ViewGrid>
					{filtered.map(node => (
						<SwarmNodeCard key={node.id} {...nodeProps(node)} />
					))}
				</ViewGrid>
			) : (
				<ListCard>
					{filtered.map(node => (
						<SwarmNodeRow key={node.id} {...nodeProps(node)} />
					))}
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
