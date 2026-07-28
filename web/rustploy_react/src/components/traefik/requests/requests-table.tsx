import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableHead,
	TableBody,
	TableRow,
	TableCell,
} from '#/components/ui/table';

export interface TraefikLogEntry {
	timestamp: string;
	client_ip: string;
	method: string;
	path: string;
	status: number;
	duration_ms: number;
	service_name: string;
	router_name: string;
}

interface RequestsTableProps {
	items: TraefikLogEntry[];
	totalCount: number;
	page: number;
	pageSize: number;
	isLoading: boolean;
	visibleColumns: Record<string, boolean>;
	onPageChange: (newPage: number) => void;
}

export function RequestsTable({
	items,
	totalCount,
	page,
	pageSize,
	isLoading,
	visibleColumns,
	onPageChange,
}: RequestsTableProps) {
	const totalPages = Math.max(1, Math.ceil(totalCount / pageSize));
	const visibleCount = Object.values(visibleColumns).filter(Boolean).length;

	const getStatusBadge = (code: number) => {
		if (code >= 200 && code < 300) {
			return <Badge variant="outline" className="bg-emerald-500/10 text-emerald-500 border-emerald-500/20 font-mono text-[11px] px-1.5 py-0 font-semibold">{code} OK</Badge>;
		}
		if (code >= 300 && code < 400) {
			return <Badge variant="outline" className="bg-blue-500/10 text-blue-400 border-blue-500/20 font-mono text-[11px] px-1.5 py-0 font-semibold">{code} Redirect</Badge>;
		}
		if (code >= 400 && code < 500) {
			return <Badge variant="outline" className="bg-amber-500/10 text-amber-500 border-amber-500/20 font-mono text-[11px] px-1.5 py-0 font-semibold">{code} Error</Badge>;
		}
		return <Badge variant="outline" className="bg-rose-500/10 text-rose-500 border-rose-500/20 font-mono text-[11px] px-1.5 py-0 font-semibold">{code} Fatal</Badge>;
	};

	const getMethodBadge = (method: string) => {
		const m = method.toUpperCase();
		switch (m) {
			case 'GET':
				return <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded bg-emerald-500/15 text-emerald-400 border border-emerald-500/20">GET</span>;
			case 'POST':
				return <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded bg-blue-500/15 text-blue-400 border border-blue-500/20">POST</span>;
			case 'PUT':
				return <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded bg-amber-500/15 text-amber-400 border border-amber-500/20">PUT</span>;
			case 'DELETE':
				return <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded bg-rose-500/15 text-rose-400 border border-rose-500/20">DEL</span>;
			default:
				return <span className="text-[10px] font-bold font-mono px-2 py-0.5 rounded bg-muted text-muted-foreground">{m}</span>;
		}
	};

	return (
		<div className="flex-1 border border-border/60 rounded-xl overflow-hidden flex flex-col min-h-0 shadow-xs bg-card">
			<div className="flex-1 overflow-auto">
				<Table>
					<TableHeader className="bg-muted/40 sticky top-0 backdrop-blur-md z-10">
						<TableRow className="border-b border-border/50 hover:bg-transparent">
							{visibleColumns.method && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Method</TableHead>}
							{visibleColumns.path && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Request Path</TableHead>}
							{visibleColumns.status && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Status</TableHead>}
							{visibleColumns.latency && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Latency</TableHead>}
							{visibleColumns.client_ip && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Client IP</TableHead>}
							{visibleColumns.service && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3">Target Service</TableHead>}
							{visibleColumns.time && <TableHead className="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-3.5 py-3 text-right">Time</TableHead>}
						</TableRow>
					</TableHeader>
					<TableBody className="divide-y divide-border/20 font-mono text-[11px]">
						{isLoading && items.length === 0 ? (
							<TableRow>
								<TableCell colSpan={visibleCount} className="py-16 text-center text-muted-foreground font-sans text-xs">
									<RefreshCw className="size-5 animate-spin mx-auto mb-2 text-muted-foreground/70" />
									Loading Traefik request logs...
								</TableCell>
							</TableRow>
						) : items.length === 0 ? (
							<TableRow>
								<TableCell colSpan={visibleCount} className="py-16 text-center text-muted-foreground font-sans text-xs">
									No request logs matching the selected filters.
								</TableCell>
							</TableRow>
						) : (
							items.map((log, i) => (
								<TableRow key={`${log.timestamp}-${i}`} className="hover:bg-muted/30 transition-colors">
									{visibleColumns.method && <TableCell className="py-2.5 px-3.5">{getMethodBadge(log.method)}</TableCell>}
									{visibleColumns.path && (
										<TableCell className="py-2.5 px-3.5 font-semibold text-foreground truncate max-w-xs" title={log.path}>
											{log.path}
										</TableCell>
									)}
									{visibleColumns.status && <TableCell className="py-2.5 px-3.5">{getStatusBadge(log.status)}</TableCell>}
									{visibleColumns.latency && (
										<TableCell className="py-2.5 px-3.5 text-muted-foreground font-medium">
											{log.duration_ms.toFixed(1)}ms
										</TableCell>
									)}
									{visibleColumns.client_ip && <TableCell className="py-2.5 px-3.5 text-muted-foreground/80">{log.client_ip}</TableCell>}
									{visibleColumns.service && (
										<TableCell className="py-2.5 px-3.5 text-muted-foreground/70 truncate max-w-[140px]" title={log.service_name}>
											{log.service_name !== '-' ? log.service_name : log.router_name}
										</TableCell>
									)}
									{visibleColumns.time && (
										<TableCell className="py-2.5 px-3.5 text-right text-muted-foreground/60 text-[10px]">
											{log.timestamp ? new Date(log.timestamp).toLocaleTimeString() : '-'}
										</TableCell>
									)}
								</TableRow>
							))
						)}
					</TableBody>
				</Table>
			</div>

			<div className="flex items-center justify-between px-4 py-2.5 border-t border-border/40 bg-muted/20 text-xs text-muted-foreground shrink-0 font-sans">
				<span>
					Showing Page <strong className="text-foreground">{page}</strong> of <strong className="text-foreground">{totalPages}</strong> ({totalCount} total)
				</span>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						disabled={page <= 1}
						onClick={() => onPageChange(Math.max(1, page - 1))}
						className="h-7 px-3 text-xs font-medium cursor-pointer">
						Previous
					</Button>
					<Button
						variant="outline"
						size="sm"
						disabled={page >= totalPages}
						onClick={() => onPageChange(Math.min(totalPages, page + 1))}
						className="h-7 px-3 text-xs font-medium cursor-pointer">
						Next
					</Button>
				</div>
			</div>
		</div>
	);
}
