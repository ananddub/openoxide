import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
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
} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';

interface SwarmNodesListProps {
	nodes: any[];
	isLoading: boolean;
	onPromote: (nodeId: string) => void;
	onDemote: (nodeId: string) => void;
	onSetAvailability: (nodeId: string, availability: string) => void;
	onRemoveNode: (nodeId: string) => void;
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
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-2">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-16 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (!nodes || nodes.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-8 text-center flex flex-col items-center justify-center rounded-xl">
				<Server className="w-6 h-6 text-muted-foreground mb-2" />
				<h3 className="text-xs font-bold text-foreground">No Nodes Discovered</h3>
			</Card>
		);
	}

	return (
		<div className="flex flex-col gap-3 py-2">
			<div className="flex items-center justify-between">
				<h3 className="text-xs font-bold text-foreground tracking-wide uppercase">
					Cluster Nodes ({nodes.length})
				</h3>
			</div>

			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 w-full">
				{nodes.map(node => {
					const isManager = (node.role || '').toLowerCase() === 'manager';
					const isReady = (node.status || '').toLowerCase() === 'ready';
					const availability = (node.availability || 'active').toLowerCase();

					return (
						<Card
							key={node.id}
							className={`bg-card border transition-all rounded-xl shadow-sm ${
								isManager ? 'border-amber-500/40 bg-amber-500/[0.02]' : 'border-border hover:border-border/80'
							}`}
						>
							<CardContent className="p-3 flex items-center justify-between gap-2">
								{/* Left: Status Dot, Node Name & Master/Worker Badge */}
								<div className="flex items-center gap-2 min-w-0 flex-1">
									<span
										className={`block w-2.5 h-2.5 rounded-full shrink-0 ${
											isReady ? 'bg-emerald-500 shadow-sm shadow-emerald-500/50' : 'bg-rose-500'
										}`}
										title={`Status: ${node.status || 'Unknown'}`}
									/>

									<div className="flex flex-col min-w-0 flex-1">
										<div className="flex items-center gap-1.5 min-w-0">
											<h4 className="text-xs font-bold text-foreground truncate">{node.hostname || node.name || 'Node'}</h4>

											{/* Master / Manager Node Visual Badge */}
											{isManager ? (
												<span className="inline-flex items-center gap-1 text-[9px] font-bold px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-500 border border-amber-500/30 shrink-0">
													<Crown className="w-2.5 h-2.5 fill-amber-500/20" />
													Master
												</span>
											) : (
												<span className="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-muted text-muted-foreground shrink-0">
													Worker
												</span>
											)}
										</div>

										<div className="flex items-center gap-1 text-[11px] font-mono text-muted-foreground truncate mt-0.5">
											<span className="truncate">{node.id ? node.id.slice(0, 12) : 'No ID'}</span>
											<button
												type="button"
												onClick={() => handleCopyId(node.id)}
												className="text-muted-foreground/60 hover:text-foreground shrink-0"
												title="Copy Node ID"
											>
												{copiedId === node.id ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
											</button>
										</div>
									</div>
								</div>

								{/* Right: Actions Dropdown */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										{isManager ? (
											<DropdownMenuItem
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => onDemote(node.id)}
											>
												Demote to Worker
											</DropdownMenuItem>
										) : (
											<DropdownMenuItem
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => onPromote(node.id)}
											>
												Promote to Manager
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											disabled={availability === 'active'}
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetAvailability(node.id, 'active')}
										>
											Set Active
										</DropdownMenuItem>

										<DropdownMenuItem
											disabled={availability === 'pause'}
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetAvailability(node.id, 'pause')}
										>
											Set Pause
										</DropdownMenuItem>

										<DropdownMenuItem
											disabled={availability === 'drain'}
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetAvailability(node.id, 'drain')}
										>
											Set Drain
										</DropdownMenuItem>

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onRemoveNode(node.id)}
										>
											Remove Node
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</CardContent>
						</Card>
					);
				})}
			</div>
		</div>
	);
}
