import {Package, MoreVertical} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuLabel,
	DropdownMenuGroup,
} from '#/components/ui/dropdown';
import type {GlobalContainerItem} from './docker-inspect-modal';

interface DockerContainersTableProps {
	containers: GlobalContainerItem[];
	isLoading?: boolean;
	onOpenModal: (container: GlobalContainerItem, type: 'logs' | 'config' | 'mount' | 'network') => void;
	onAction: (container: GlobalContainerItem, action: 'start' | 'stop' | 'restart' | 'kill') => void;
}

export function DockerContainersTable({containers, isLoading, onOpenModal, onAction}: DockerContainersTableProps) {
	const safeContainers = Array.isArray(containers) ? containers : [];

	if (isLoading && safeContainers.length === 0) {
		return (
			<section className="bg-card border border-border rounded-xl p-12 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2 shadow-sm">
				<Package className="w-6 h-6 animate-bounce text-primary" />
				<p>Loading Docker containers...</p>
			</section>
		);
	}

	if (safeContainers.length === 0) {
		return (
			<section className="bg-card border border-border rounded-xl p-12 flex flex-col items-center justify-center text-xs text-muted-foreground gap-2 shadow-sm text-center">
				<Package className="w-10 h-10 opacity-30 text-primary" />
				<p className="text-sm font-bold text-foreground">No Docker containers found</p>
				<p className="text-xs text-muted-foreground">There are currently no active Docker containers running on this server.</p>
			</section>
		);
	}

	return (
		<section className="bg-card border border-border rounded-xl overflow-hidden shadow-sm">
			<div className="overflow-x-auto">
				<table className="w-full text-left text-xs">
					<thead className="bg-muted/50 border-b border-border text-muted-foreground uppercase text-[10px] font-bold tracking-wider">
						<tr>
							<th className="py-3 px-4">Container Name</th>
							<th className="py-3 px-4">Container ID</th>
							<th className="py-3 px-4">Image</th>
							<th className="py-3 px-4">Status</th>
							<th className="py-3 px-4 text-right">Actions</th>
						</tr>
					</thead>
					<tbody className="divide-y divide-border/60">
						{safeContainers.map((c) => (
							<tr key={c.id} className="hover:bg-muted/20 transition-colors">
								{/* Container Name */}
								<td className="py-3.5 px-4">
									<div className="flex items-center gap-2.5">
										<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-primary shrink-0 border border-border/40">
											<Package className="w-4 h-4" />
										</div>
										<div className="flex flex-col gap-0.5">
											<span className="font-bold text-foreground">{c.name}</span>
											<span className="text-[10px] font-mono text-muted-foreground">
												Created: {c.created}
											</span>
										</div>
									</div>
								</td>

								{/* Container ID */}
								<td className="py-3.5 px-4 font-mono text-muted-foreground">
									<span className="bg-muted/40 px-2 py-0.5 rounded border border-border/40 text-foreground">
										{c.id}
									</span>
								</td>

								{/* Image */}
								<td className="py-3.5 px-4 font-mono text-muted-foreground truncate max-w-[200px]">
									{c.image}
								</td>

								{/* Status */}
								<td className="py-3.5 px-4">
									<div className="flex items-center gap-1.5">
										<span className={`h-2 w-2 rounded-full ${c.status === 'running' ? 'bg-emerald-500 animate-pulse' : 'bg-rose-500'}`} />
										<span className={`font-semibold capitalize text-[11px] ${c.status === 'running' ? 'text-emerald-400' : 'text-rose-400'}`}>
											{c.status}
										</span>
									</div>
								</td>

								{/* 3-Dots Action Menu Dropdown */}
								<td className="py-3.5 px-4 text-right">
									<DropdownMenu>
										<DropdownMenuTrigger render={
											<Button
												variant="ghost"
												size="icon"
												className="h-8 w-8 hover:bg-muted text-muted-foreground hover:text-foreground"
											>
												<MoreVertical className="w-4 h-4" />
											</Button>
										} />
										<DropdownMenuContent align="end" className="w-40 bg-card border-border shadow-xl">
											<DropdownMenuGroup>
												<DropdownMenuLabel className="text-[10px] uppercase font-bold text-muted-foreground px-2 py-1">
													View Options
												</DropdownMenuLabel>
												
												<DropdownMenuItem onClick={() => onOpenModal(c, 'logs')} className="text-xs cursor-pointer text-foreground">
													View Logs
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onOpenModal(c, 'config')} className="text-xs cursor-pointer text-foreground">
													View Config
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onOpenModal(c, 'mount')} className="text-xs cursor-pointer text-foreground">
													View Mounts
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onOpenModal(c, 'network')} className="text-xs cursor-pointer text-foreground">
													View Network
												</DropdownMenuItem>
											</DropdownMenuGroup>

											<DropdownMenuSeparator />

											<DropdownMenuGroup>
												<DropdownMenuLabel className="text-[10px] uppercase font-bold text-muted-foreground px-2 py-1">
													Actions
												</DropdownMenuLabel>

												<DropdownMenuItem onClick={() => onAction(c, 'start')} className="text-xs cursor-pointer text-foreground">
													Start
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onAction(c, 'stop')} className="text-xs cursor-pointer text-foreground">
													Stop
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onAction(c, 'restart')} className="text-xs cursor-pointer text-foreground">
													Restart
												</DropdownMenuItem>

												<DropdownMenuItem onClick={() => onAction(c, 'kill')} className="text-xs cursor-pointer text-rose-500 hover:text-rose-400">
													Kill
												</DropdownMenuItem>
											</DropdownMenuGroup>
										</DropdownMenuContent>
									</DropdownMenu>
								</td>
							</tr>
						))}
					</tbody>
				</table>
			</div>
		</section>
	);
}
