import {
	Box,
	Layers,
	MoreVertical,
	Terminal,
	Server,
	HardDrive,
	Network,
	Play,
	Square,
	RotateCw,
} from 'lucide-react';
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
import type {ContainerItem} from './container-inspect-modal';

interface ComposeContainersTableProps {
	containers: ContainerItem[];
	onOpenModal: (
		container: ContainerItem,
		type: 'logs' | 'config' | 'mount' | 'network',
	) => void;
	onAction: (
		container: ContainerItem,
		action: 'start' | 'stop' | 'restart' | 'kill',
	) => void;
}

export function ComposeContainersTable({
	containers,
	onOpenModal,
	onAction,
}: ComposeContainersTableProps) {
	if (containers.length === 0) {
		return (
			<section className="flex flex-col items-center justify-center rounded-xl border border-border/80 bg-card p-12 text-center shadow-sm">
				<div className="mb-3 flex size-12 items-center justify-center rounded-2xl bg-muted text-muted-foreground">
					<Box className="h-6 w-6" />
				</div>
				<h4 className="text-sm font-bold text-foreground">
					No Containers Deployed
				</h4>
				<p className="mt-1 max-w-sm text-xs text-muted-foreground">
					There are no active live Docker containers for this stack. Deploy
					or start the stack to launch containers.
				</p>
			</section>
		);
	}

	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card shadow-sm">
			<div className="overflow-x-auto">
				<table className="w-full text-left text-xs">
					<thead className="border-b border-border bg-muted/50 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
						<tr>
							<th className="px-4 py-3">Container & Service</th>
							<th className="px-4 py-3">Container ID</th>
							<th className="px-4 py-3">Image</th>
							<th className="px-4 py-3">Status</th>
							<th className="px-4 py-3 text-right">Actions</th>
						</tr>
					</thead>
					<tbody className="divide-y divide-border/60">
						{containers.map(c => (
							<tr
								key={c.id}
								className="transition-colors hover:bg-muted/20">
								{/* Container & Service Name */}
								<td className="px-4 py-3.5">
									<div className="flex items-center gap-2.5">
										<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-border/40 bg-secondary text-primary">
											<Box className="h-4 w-4" />
										</div>
										<div className="flex flex-col gap-0.5">
											<span className="font-mono font-bold text-foreground">
												{c.name}
											</span>
											<span className="flex items-center gap-1 font-mono text-[10px] text-muted-foreground">
												<Layers className="h-3 w-3 text-primary" />{' '}
												Service:{' '}
												<b className="text-foreground">{c.service}</b>
											</span>
										</div>
									</div>
								</td>

								{/* Container ID */}
								<td className="px-4 py-3.5 font-mono text-muted-foreground">
									<span className="rounded border border-border/40 bg-muted/40 px-2 py-0.5 text-foreground">
										{c.id}
									</span>
								</td>

								{/* Image */}
								<td className="max-w-[180px] truncate px-4 py-3.5 font-mono text-muted-foreground">
									{c.image}
								</td>

								{/* Status */}
								<td className="px-4 py-3.5">
									<div className="flex items-center gap-1.5">
										<span
											className={`h-2 w-2 rounded-full ${c.status === 'running' ? 'animate-pulse bg-emerald-500' : 'bg-rose-500'}`}
										/>
										<span
											className={`text-[11px] font-semibold capitalize ${c.status === 'running' ? 'text-emerald-400' : 'text-rose-400'}`}>
											{c.statusText || c.status}
										</span>
									</div>
								</td>

								{/* 3-Dots Action Menu Dropdown */}
								<td className="px-4 py-3.5 text-right">
									<DropdownMenu>
										<DropdownMenuTrigger
											render={
												<Button
													variant="ghost"
													size="icon"
													className="h-8 w-8 text-muted-foreground hover:bg-muted hover:text-foreground">
													<MoreVertical className="h-4 w-4" />
												</Button>
											}
										/>
										<DropdownMenuContent
											align="end"
											className="w-44 border-border bg-card shadow-xl">
											<DropdownMenuGroup>
												<DropdownMenuLabel className="px-2 py-1 text-[10px] font-bold text-muted-foreground uppercase">
													View Options
												</DropdownMenuLabel>

												<DropdownMenuItem
													onClick={() => onOpenModal(c, 'logs')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<Terminal className="h-3.5 w-3.5 text-amber-400" />{' '}
													View Logs
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onOpenModal(c, 'config')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<Server className="h-3.5 w-3.5 text-primary" />{' '}
													View Config
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onOpenModal(c, 'mount')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<HardDrive className="h-3.5 w-3.5 text-sky-400" />{' '}
													View Mounts
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onOpenModal(c, 'network')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<Network className="h-3.5 w-3.5 text-emerald-400" />{' '}
													View Network
												</DropdownMenuItem>
											</DropdownMenuGroup>

											<DropdownMenuSeparator />

											<DropdownMenuGroup>
												<DropdownMenuLabel className="px-2 py-1 text-[10px] font-bold text-muted-foreground uppercase">
													Actions
												</DropdownMenuLabel>

												<DropdownMenuItem
													onClick={() => onAction(c, 'start')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<Play className="h-3.5 w-3.5 text-emerald-400" />{' '}
													Start
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onAction(c, 'stop')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<Square className="h-3.5 w-3.5 text-rose-400" />{' '}
													Stop
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onAction(c, 'restart')}
													className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
													<RotateCw className="h-3.5 w-3.5 text-sky-400" />{' '}
													Restart
												</DropdownMenuItem>

												<DropdownMenuItem
													onClick={() => onAction(c, 'kill')}
													className="cursor-pointer text-xs text-rose-500 hover:text-rose-400">
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
