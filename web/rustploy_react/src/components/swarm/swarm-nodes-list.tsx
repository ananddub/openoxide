import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {Badge} from '#/components/ui/badge';
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
	Globe,
} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';

interface SwarmNodesListProps {
	nodes: Record<string, unknown>[];
	isLoading: boolean;
	onPromote: (nodeId: string, node?: Record<string, unknown>) => void;
	onDemote: (nodeId: string, node?: Record<string, unknown>) => void;
	onSetAvailability: (nodeId: string, availability: string, node?: Record<string, unknown>) => void;
	onRemoveNode: (nodeId: string, node?: Record<string, unknown>) => void;
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

	const handleCopyId = (id: string) => {
		navigator.clipboard.writeText(id);
		setCopiedId(id);
		toast.success('Node ID copied to clipboard');
		setTimeout(() => setCopiedId(null), 2000);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4 py-2">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-28 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (!nodes || nodes.length === 0) {
		return (
			<Card className="bg-card border-border shadow-2xs p-8 text-center flex flex-col items-center justify-center rounded-xl">
				<Server className="w-8 h-8 text-muted-foreground/60 mb-2" />
				<h3 className="text-sm font-bold text-foreground">No Nodes Discovered</h3>
				<p className="text-xs text-muted-foreground mt-1">Make sure Docker Swarm is active on this host engine.</p>
			</Card>
		);
	}

	return (
		<div className="flex flex-col gap-3 py-2">
			<div className="flex items-center justify-between">
				<h3 className="text-xs font-bold text-muted-foreground uppercase tracking-wider">
					Cluster Nodes ({nodes.length})
				</h3>
			</div>

			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4 w-full">
				{nodes.map((node: any) => {
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

					return (
						<Card
							key={nodeId}
							className={`transition-all rounded-xl shadow-2xs flex flex-col justify-between ${
								isManager
									? 'border-amber-500/40 bg-amber-500/[0.03] dark:bg-amber-500/[0.02]'
									: 'bg-card border border-border hover:border-border/80'
							}`}
						>
							<CardContent className="p-4 flex flex-col justify-between h-full gap-3">
								{/* Header: Hostname, Role Badge, Dropdown Menu */}
								<div className="flex items-start justify-between gap-2">
									<div className="flex items-start gap-2.5 min-w-0 flex-1">
										<div className={`p-2 rounded-lg shrink-0 mt-0.5 border ${
											isManager
												? 'bg-amber-500/10 text-amber-500 border-amber-500/30'
												: 'bg-muted/40 text-muted-foreground border-border/40'
										}`}>
											{isManager ? <Crown className="size-4 text-amber-500" /> : <Server className="size-4 text-muted-foreground" />}
										</div>

										<div className="flex flex-col min-w-0 flex-1">
											<div className="flex items-center gap-1.5 min-w-0 flex-wrap">
												<h4 className="text-xs font-bold text-foreground truncate" title={hostname}>
													{hostname}
												</h4>

												{/* Role Badge */}
												{isManager ? (
													<Badge variant="outline" className="text-[10px] uppercase font-bold tracking-wider text-amber-500 border-amber-500/30 bg-amber-500/10 gap-1 px-1.5 py-0">
														<Crown className="size-2.5 fill-amber-500/20" />
														{isLeader ? 'Leader' : 'Manager'}
													</Badge>
												) : (
													<Badge variant="secondary" className="text-[10px] uppercase font-bold tracking-wider px-1.5 py-0">
														Worker
													</Badge>
												)}

												{/* Server Host Origin Badge */}
												{serverLabel && (
													<span className="text-[9px] font-mono font-medium px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground border border-border/40 shrink-0">
														{serverLabel}
													</span>
												)}
											</div>

											{/* ID with Copy Button */}
											<div className="flex items-center gap-1 text-[10px] font-mono text-muted-foreground truncate mt-1">
												<span className="truncate">{nodeId ? nodeId.slice(0, 14) : 'No ID'}</span>
												{nodeId && (
													<button
														type="button"
														onClick={() => handleCopyId(nodeId)}
														className="text-muted-foreground/60 hover:text-foreground shrink-0 cursor-pointer"
														title="Copy Node ID"
													>
														{copiedId === nodeId ? <Check className="size-3 text-emerald-500" /> : <Copy className="size-3" />}
													</button>
												)}
											</div>
										</div>
									</div>

									{/* Right: Actions Dropdown Menu */}
									<DropdownMenu>
										<DropdownMenuTrigger
											render={
												<Button variant="ghost" size="icon" className="size-7 text-muted-foreground hover:text-foreground shrink-0 rounded-md">
													<MoreVertical className="size-3.5" />
												</Button>
											}
										/>
										<DropdownMenuContent align="end" className="w-44 bg-popover border-border shadow-md rounded-xl p-1 text-xs z-50">
											{isManager ? (
												<DropdownMenuItem
													className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-xs font-semibold"
													onClick={() => onDemote(nodeId, node)}
												>
													Demote to Worker
												</DropdownMenuItem>
											) : (
												<DropdownMenuItem
													className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-xs font-semibold"
													onClick={() => onPromote(nodeId, node)}
												>
													Promote to Manager
												</DropdownMenuItem>
											)}

											<DropdownMenuSeparator className="my-1 border-border/40" />

											<DropdownMenuItem
												disabled={availability === 'active'}
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-xs font-semibold"
												onClick={() => onSetAvailability(nodeId, 'active', node)}
											>
												Set Active
											</DropdownMenuItem>

											<DropdownMenuItem
												disabled={availability === 'pause'}
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-xs font-semibold"
												onClick={() => onSetAvailability(nodeId, 'pause', node)}
											>
												Set Pause
											</DropdownMenuItem>

											<DropdownMenuItem
												disabled={availability === 'drain'}
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-xs font-semibold"
												onClick={() => onSetAvailability(nodeId, 'drain', node)}
											>
												Set Drain
											</DropdownMenuItem>

											<DropdownMenuSeparator className="my-1 border-border/40" />

											<DropdownMenuItem
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg text-destructive text-xs font-semibold focus:text-destructive focus:bg-destructive/10"
												onClick={() => onRemoveNode(nodeId, node)}
											>
												Remove Node
											</DropdownMenuItem>
										</DropdownMenuContent>
									</DropdownMenu>
								</div>

								{/* Bottom Metadata Info: Status & Availability Badges */}
								<div className="flex items-center justify-between border-t border-border/40 pt-2.5 mt-auto gap-2">
									<div className="flex items-center gap-1.5">
										<span
											className={`size-2 rounded-full shrink-0 ${
												isReady ? 'bg-emerald-500' : 'bg-rose-500'
											}`}
										/>
										<span className="text-[10px] font-mono font-bold uppercase text-muted-foreground">
											{isReady ? 'Ready' : statusStr}
										</span>
									</div>

									<div className="flex items-center gap-1.5">
										{ipAddr && (
											<span className="text-[10px] font-mono text-muted-foreground/80 flex items-center gap-1">
												<Globe className="size-2.5" />
												{ipAddr}
											</span>
										)}
										<span className={`text-[9px] font-mono uppercase font-bold px-1.5 py-0.5 rounded border ${
											availability === 'active'
												? 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20'
												: availability === 'drain'
												? 'text-amber-500 bg-amber-500/10 border-amber-500/20'
												: 'text-muted-foreground bg-muted border-border/40'
										}`}>
											{availability}
										</span>
									</div>
								</div>
							</CardContent>
						</Card>
					);
				})}
			</div>
		</div>
	);
}
