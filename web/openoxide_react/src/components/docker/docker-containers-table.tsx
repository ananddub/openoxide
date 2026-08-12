import {useState, useMemo} from 'react';
import {Package, MoreVertical, Search, RefreshCw} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableHead,
	TableBody,
	TableRow,
	TableCell,
} from '#/components/ui/table';
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
	const [searchQuery, setSearchQuery] = useState('');
	const safeContainers = Array.isArray(containers) ? containers : [];

	const filteredContainers = useMemo(() => {
		if (!searchQuery.trim()) return safeContainers;
		const q = searchQuery.toLowerCase();
		return safeContainers.filter(
			(c) =>
				c.name.toLowerCase().includes(q) ||
				c.id.toLowerCase().includes(q) ||
				c.image.toLowerCase().includes(q)
		);
	}, [safeContainers, searchQuery]);

	if (isLoading && safeContainers.length === 0) {
		return (
			<div className="py-20 flex flex-col items-center justify-center text-xs text-muted-foreground gap-3">
				<RefreshCw className="size-6 animate-spin text-muted-foreground/70" />
				<span>Loading system Docker containers...</span>
			</div>
		);
	}

	if (safeContainers.length === 0) {
		return (
			<div className="py-20 flex flex-col items-center justify-center text-xs text-muted-foreground gap-3 text-center">
				<Package className="size-8 opacity-40 text-muted-foreground" />
				<div className="space-y-1">
					<p className="text-sm font-bold text-foreground">No Docker containers found</p>
					<p className="text-xs text-muted-foreground max-w-sm">
						There are currently no active Docker containers running on this server host.
					</p>
				</div>
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3.5 flex-1 min-h-0">
			{/* Search Filter Bar */}
			<div className="relative max-w-sm w-full">
				<Search className="size-3.5 text-muted-foreground absolute left-3 top-3" />
				<Input
					value={searchQuery}
					onChange={(e) => setSearchQuery(e.target.value)}
					placeholder="Search containers by name, id, image..."
					className="h-9 text-xs font-mono pl-9 bg-card border-border/60 shadow-2xs"
				/>
			</div>

			{/* Shadcn Table */}
			<div className="border border-border/60 rounded-xl overflow-hidden shadow-xs bg-card">
				<Table>
					<TableHeader className="bg-muted/40 sticky top-0 backdrop-blur-md">
						<TableRow className="border-b border-border/50 hover:bg-transparent">
							<TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-4 py-3">Container Name</TableHead>
							<TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-4 py-3">Container ID</TableHead>
							<TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-4 py-3">Image</TableHead>
							<TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-4 py-3">Status</TableHead>
							<TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-4 py-3 text-right">Actions</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody className="divide-y divide-border/20 text-xs">
						{filteredContainers.length === 0 ? (
							<TableRow>
								<TableCell colSpan={5} className="py-12 text-center text-muted-foreground text-xs font-sans">
									No containers matching search "{searchQuery}".
								</TableCell>
							</TableRow>
						) : (
							filteredContainers.map((c) => (
								<TableRow key={c.id} className="hover:bg-muted/30 transition-colors">
									{/* Name */}
									<TableCell className="py-3 px-4">
										<div className="flex items-center gap-2.5">
											<Package className="size-4 text-primary shrink-0" />
											<div className="flex flex-col gap-0.5">
												<span className="font-bold text-foreground text-xs">{c.name}</span>
												<span className="text-[10px] text-muted-foreground/70 font-mono">
													Created: {c.created}
												</span>
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
							))
						)}
					</TableBody>
				</Table>
			</div>
		</div>
	);
}
