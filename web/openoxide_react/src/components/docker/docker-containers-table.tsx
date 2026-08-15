import { useState, useMemo } from 'react';
import { Package, RefreshCw } from 'lucide-react';
import { Table, TableHeader, TableHead, TableBody, TableRow, TableCell } from '#/components/ui/table';
import { DockerStatusFilter } from './docker-status-filter';
import { DockerContainerRow } from './docker-container-row';
import type { GlobalContainerItem } from './docker-inspect-modal';

interface DockerContainersTableProps {
	containers: GlobalContainerItem[];
	isLoading?: boolean;
	onOpenModal: (container: GlobalContainerItem, type: 'logs' | 'config' | 'mount' | 'network') => void;
	onAction: (container: GlobalContainerItem, action: 'start' | 'stop' | 'restart' | 'kill') => void;
}

export function DockerContainersTable({ containers, isLoading, onOpenModal, onAction }: DockerContainersTableProps) {
	const [searchQuery, setSearchQuery] = useState('');
	const [statusFilter, setStatusFilter] = useState<'all' | 'running' | 'stopped'>('all');
	const safeContainers = Array.isArray(containers) ? containers : [];

	const runningCount = useMemo(() => safeContainers.filter((c) => c.status === 'running').length, [safeContainers]);
	const stoppedCount = useMemo(() => safeContainers.filter((c) => c.status !== 'running').length, [safeContainers]);

	const filteredContainers = useMemo(() => {
		let list = safeContainers;
		if (statusFilter === 'running') {
			list = list.filter((c) => c.status === 'running');
		} else if (statusFilter === 'stopped') {
			list = list.filter((c) => c.status !== 'running');
		}

		if (!searchQuery.trim()) return list;
		const q = searchQuery.toLowerCase();
		return list.filter((c) => c.name.toLowerCase().includes(q) || c.id.toLowerCase().includes(q) || c.image.toLowerCase().includes(q));
	}, [safeContainers, searchQuery, statusFilter]);

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
					<p className="text-xs text-muted-foreground max-w-sm">There are currently no active Docker containers running on this host.</p>
				</div>
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3.5 flex-1 min-h-0">
			{/* Search & Status Filter Bar */}
			<DockerStatusFilter
				searchQuery={searchQuery}
				onSearchChange={setSearchQuery}
				statusFilter={statusFilter}
				onStatusFilterChange={setStatusFilter}
				totalCount={safeContainers.length}
				runningCount={runningCount}
				stoppedCount={stoppedCount}
			/>

			{/* Limited Height Scrollable Table Container */}
			<div className="border border-border/60 rounded-xl shadow-xs bg-card overflow-hidden">
				<div className="max-h-[calc(100vh-260px)] min-h-[300px] overflow-y-auto relative scrollbar-thin scrollbar-thumb-border/40">
					<Table>
						<TableHeader className="bg-card sticky top-0 z-10 border-b border-border/50 backdrop-blur-md shadow-2xs">
							<TableRow className="hover:bg-transparent">
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
										No containers matching filter criteria.
									</TableCell>
								</TableRow>
							) : (
								filteredContainers.map((c) => (
									<DockerContainerRow key={c.id} container={c} onOpenModal={onOpenModal} onAction={onAction} />
								))
							)}
						</TableBody>
					</Table>
				</div>
			</div>
		</div>
	);
}
