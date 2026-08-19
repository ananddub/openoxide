import {Package, MoreVertical} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {TableRow, TableCell} from '#/components/ui/table';
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

interface DockerContainerRowProps {
	container: GlobalContainerItem;
	onOpenModal: (
		container: GlobalContainerItem,
		type: 'logs' | 'config' | 'mount' | 'network',
	) => void;
	onAction: (
		container: GlobalContainerItem,
		action: 'start' | 'stop' | 'restart' | 'kill',
	) => void;
}

export function DockerContainerRow({
	container: c,
	onOpenModal,
	onAction,
}: DockerContainerRowProps) {
	return (
		<TableRow className="transition-colors hover:bg-muted/30">
			{/* Name */}
			<TableCell className="px-4 py-3">
				<div className="flex items-center gap-2.5">
					<Package className="size-4 shrink-0 text-primary" />
					<div className="flex flex-col gap-0.5">
						<span className="text-xs font-bold text-foreground">
							{c.name}
						</span>
						<span className="font-mono text-[10px] text-muted-foreground/70">
							Created: {c.created}
						</span>
					</div>
				</div>
			</TableCell>

			{/* ID */}
			<TableCell className="px-4 py-3 font-mono text-xs">
				<span className="rounded border border-border/40 bg-muted/40 px-2 py-0.5 font-semibold text-foreground">
					{c.id}
				</span>
			</TableCell>

			{/* Image */}
			<TableCell
				className="max-w-[220px] truncate px-4 py-3 font-mono text-xs text-muted-foreground"
				title={c.image}>
				{c.image}
			</TableCell>

			{/* Status */}
			<TableCell className="px-4 py-3">
				{c.status === 'running' ? (
					<Badge
						variant="outline"
						className="gap-1.5 border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 font-mono text-[11px] font-semibold text-emerald-500">
						<span className="size-1.5 animate-pulse rounded-full bg-emerald-500" />{' '}
						Running
					</Badge>
				) : (
					<Badge
						variant="outline"
						className="gap-1.5 border-rose-500/20 bg-rose-500/10 px-2 py-0.5 font-mono text-[11px] font-semibold text-rose-500">
						<span className="size-1.5 rounded-full bg-rose-500" /> Stopped
					</Badge>
				)}
			</TableCell>

			{/* Actions Dropdown */}
			<TableCell className="px-4 py-3 text-right">
				<DropdownMenu>
					<DropdownMenuTrigger className="inline-flex size-8 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
						<MoreVertical className="size-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent
						align="end"
						className="w-44 border-border bg-card p-1 text-xs shadow-md">
						<DropdownMenuGroup>
							<DropdownMenuLabel className="px-2 py-1 text-[10px] font-bold text-muted-foreground uppercase">
								Inspect Options
							</DropdownMenuLabel>
							<DropdownMenuItem
								onClick={() => onOpenModal(c, 'logs')}
								className="cursor-pointer text-xs">
								View Logs
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onOpenModal(c, 'config')}
								className="cursor-pointer text-xs">
								View Config
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onOpenModal(c, 'mount')}
								className="cursor-pointer text-xs">
								View Mounts
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onOpenModal(c, 'network')}
								className="cursor-pointer text-xs">
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
								className="cursor-pointer text-xs">
								Start
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onAction(c, 'stop')}
								className="cursor-pointer text-xs">
								Stop
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onAction(c, 'restart')}
								className="cursor-pointer text-xs">
								Restart
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={() => onAction(c, 'kill')}
								className="cursor-pointer text-xs text-rose-500 hover:text-rose-400">
								Kill Container
							</DropdownMenuItem>
						</DropdownMenuGroup>
					</DropdownMenuContent>
				</DropdownMenu>
			</TableCell>
		</TableRow>
	);
}
