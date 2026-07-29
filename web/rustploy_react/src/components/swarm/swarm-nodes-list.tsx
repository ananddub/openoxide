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
	Trash2,
	ArrowUpRight,
	ArrowDownRight,
} from 'lucide-react';
import {toast} from 'sonner';

interface SwarmNodesListProps {
	nodes: Record<string, unknown>[];
	isLoading: boolean;
	onPromote: (nodeId: string, node?: Record<string, unknown>) => void;
	onDemote: (nodeId: string, node?: Record<string, unknown>) => void;
	onSetAvailability: (nodeId: string, availability: string, node?: Record<string, unknown>) => void;
	onRemoveNode: (nodeId: string, node?: Record<string, unknown>) => void;
}

const ROLE_FILTERS = ['All', 'Managers', 'Workers'] as const;
type RoleFilter = (typeof ROLE_FILTERS)[number];

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
		return nodes.filter((node: any) => {
			const hostname = String(
				node.hostname || node.Hostname || node.Description?.Hostname || node.description?.hostname || node.name || node.Name || '',
			);
			const nodeId = String(node.id || node.ID || '');
			const ipAddr = String(node.ip || node.address || node.Addr || node.Status?.Addr || node.ManagerStatus?.Addr || node.ip_address || '');

			const matchSearch =
				hostname.toLowerCase().includes(search.toLowerCase()) ||
				nodeId.toLowerCase().includes(search.toLowerCase()) ||
				ipAddr.toLowerCase().includes(search.toLowerCase());

			const roleStr = String(node.role || node.Role || node.Spec?.Role || node.spec?.role || '').toLowerCase();
			const managerStatusVal = node.ManagerStatus || node.manager_status || node.managerStatus;
			let isLeaderObj = false;
			let managerStatusStr = '';
			if (typeof managerStatusVal === 'object' && managerStatusVal !== null) {
				isLeaderObj = !!(managerStatusVal.Leader || managerStatusVal.leader);
				managerStatusStr = isLeaderObj ? 'leader' : 'reachable';
			} else {
				managerStatusStr = String(managerStatusVal || '').toLowerCase();
			}
			const isLeader = isLeaderObj || managerStatusStr.includes('leader') || !!node.is_leader || !!node.leader || !!node.Leader;
			const isManager = roleStr === 'manager' || isLeader || managerStatusStr === 'reachable' || managerStatusStr === 'manager' || !!node.is_manager || !!node.manager;

			const matchRole =
				roleFilter === 'All' ||
				(roleFilter === 'Managers' && isManager) ||
				(roleFilter === 'Workers' && !isManager);

			return matchSearch && matchRole;
		});
	}, [nodes, search, roleFilter]);

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
						placeholder="Search by hostname, node ID or IP…"
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
					{filtered.map((node: any) => {
						const roleStr = String(node.role || node.Role || node.Spec?.Role || node.spec?.role || '').toLowerCase();
						const managerStatusVal = node.ManagerStatus || node.manager_status || node.managerStatus;

						let managerStatusStr = '';
						let isLeaderObj = false;
						if (typeof managerStatusVal === 'object' && managerStatusVal !== null) {
							isLeaderObj = !!(managerStatusVal.Leader || managerStatusVal.leader);
							managerStatusStr = isLeaderObj ? 'leader' : 'reachable';
						} else {
							managerStatusStr = String(managerStatusVal || '').toLowerCase();
						}

						const isLeader = isLeaderObj || managerStatusStr.includes('leader') || !!node.is_leader || !!node.leader || !!node.Leader;
						const isManager = roleStr === 'manager' || isLeader || managerStatusStr === 'reachable' || managerStatusStr === 'manager' || !!node.is_manager || !!node.manager;

						const statusStr = String(node.status?.state || node.Status?.State || node.status || node.Status || node.state || node.State || 'ready').toLowerCase();
						const isReady = statusStr === 'ready';
						const availability = String(node.availability || node.Availability || node.Spec?.Availability || node.spec?.availability || 'active').toLowerCase();

						const hostname = node.hostname || node.Hostname || node.Description?.Hostname || node.description?.hostname || node.name || node.Name || 'Swarm Node';
						const nodeId = String(node.id || node.ID || '');
						const ipAddr = node.ip || node.address || node.Addr || node.Status?.Addr || node.ManagerStatus?.Addr || node.ip_address || null;
						const serverLabel = node._serverName ? String(node._serverName) : null;

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
											<Badge variant="outline" className="shrink-0 text-[10px] uppercase font-bold text-amber-500 border-amber-500/30 bg-amber-500/10 gap-1 px-1.5 py-0">
												<Crown className="w-2.5 h-2.5 fill-amber-500/20" />
												{isLeader ? 'Leader' : 'Manager'}
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
										{ipAddr && (
											<span className="flex items-center gap-1 shrink-0">
												· <Globe className="w-3 h-3" /> {ipAddr}
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
											<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onDemote(nodeId, node)}>
												<ArrowDownRight className="w-3.5 h-3.5" /> Demote to Worker
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
