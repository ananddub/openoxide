import { Package, MoreVertical } from 'lucide-react';
import { Badge } from '#/components/ui/badge';
import { TableRow, TableCell } from '#/components/ui/table';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuLabel,
	DropdownMenuGroup,
} from '#/components/ui/dropdown';
import type { GlobalContainerItem } from './docker-inspect-modal';

interface DockerContainerRowProps {
	container: GlobalContainerItem;
	onOpenModal: (container: GlobalContainerItem, type: 'logs' | 'config' | 'mount' | 'network') => void;
	onAction: (container: GlobalContainerItem, action: 'start' | 'stop' | 'restart' | 'kill') => void;
}

export function DockerContainerRow({ container: c, onOpenModal, onAction }: DockerContainerRowProps) {
	return (
		<TableRow className="hover:bg-muted/30 transition-colors">
			{/* Name */}
			<TableCell className="py-3 px-4">
				<div className="flex items-center gap-2.5">
					<Package className="size-4 text-primary shrink-0" />
					<div className="flex flex-col gap-0.5">
						<span className="font-bold text-foreground text-xs">{c.name}</span>
						<span className="text-[10px] text-muted-foreground/70 font-mono">Created: {c.created}</span>
					</div>
				</div>
			</TableCell>

			{/* ID */}
			<TableCell className="py-3 px-4 font-mono text-xs">
				<span className="bg-muted/40 px-2 py-0.5 rounded border border-border/40 text-foreground font-semibold">
					{c.id}
				</span>
			</TableCell>

			{/* Image */}
			<TableCell className="py-3 px-4 font-mono text-muted-foreground text-xs truncate max-w-[220px]" title={c.image}>
				{c.image}
			</TableCell>

			{/* Status */}
			<TableCell className="py-3 px-4">
				{c.status === 'running' ? (
					<Badge variant="outline" className="bg-emerald-500/10 text-emerald-500 border-emerald-500/20 font-mono text-[11px] px-2 py-0.5 font-semibold gap-1.5">
						<span className="size-1.5 rounded-full bg-emerald-500 animate-pulse" /> Running
					</Badge>
				) : (
					<Badge variant="outline" className="bg-rose-500/10 text-rose-500 border-rose-500/20 font-mono text-[11px] px-2 py-0.5 font-semibold gap-1.5">
						<span className="size-1.5 rounded-full bg-rose-500" /> Stopped
					</Badge>
				)}
			</TableCell>

			{/* Actions Dropdown */}
			<TableCell className="py-3 px-4 text-right">
				<DropdownMenu>
					<DropdownMenuTrigger className="inline-flex items-center justify-center size-8 rounded-md hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer transition-colors">
						<MoreVertical className="size-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-44 bg-card border-border shadow-md text-xs p-1">
						<DropdownMenuGroup>
							<DropdownMenuLabel className="text-[10px] uppercase font-bold text-muted-foreground px-2 py-1">
								Inspect Options
							</DropdownMenuLabel>
							<DropdownMenuItem onClick={() => onOpenModal(c, 'logs')} className="text-xs cursor-pointer">
								View Logs
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onOpenModal(c, 'config')} className="text-xs cursor-pointer">
								View Config
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onOpenModal(c, 'mount')} className="text-xs cursor-pointer">
								View Mounts
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onOpenModal(c, 'network')} className="text-xs cursor-pointer">
								View Network
							</DropdownMenuItem>
						</DropdownMenuGroup>

						<DropdownMenuSeparator />

						<DropdownMenuGroup>
							<DropdownMenuLabel className="text-[10px] uppercase font-bold text-muted-foreground px-2 py-1">
								Actions
							</DropdownMenuLabel>
							<DropdownMenuItem onClick={() => onAction(c, 'start')} className="text-xs cursor-pointer">
								Start
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onAction(c, 'stop')} className="text-xs cursor-pointer">
								Stop
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onAction(c, 'restart')} className="text-xs cursor-pointer">
								Restart
							</DropdownMenuItem>
							<DropdownMenuItem onClick={() => onAction(c, 'kill')} className="text-xs cursor-pointer text-rose-500 hover:text-rose-400">
								Kill Container
							</DropdownMenuItem>
						</DropdownMenuGroup>
					</DropdownMenuContent>
				</DropdownMenu>
			</TableCell>
		</TableRow>
	);
}
