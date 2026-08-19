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
	const visibleCount =
		Object.values(visibleColumns).filter(Boolean).length;

	const getStatusBadge = (code: number) => {
		if (code >= 200 && code < 300) {
			return (
				<Badge
					variant="outline"
					className="border-emerald-500/20 bg-emerald-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-emerald-500">
					{code} OK
				</Badge>
			);
		}
		if (code >= 300 && code < 400) {
			return (
				<Badge
					variant="outline"
					className="border-blue-500/20 bg-blue-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-blue-400">
					{code} Redirect
				</Badge>
			);
		}
		if (code >= 400 && code < 500) {
			return (
				<Badge
					variant="outline"
					className="border-amber-500/20 bg-amber-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-amber-500">
					{code} Error
				</Badge>
			);
		}
		return (
			<Badge
				variant="outline"
				className="border-rose-500/20 bg-rose-500/10 px-1.5 py-0 font-mono text-[11px] font-semibold text-rose-500">
				{code} Fatal
			</Badge>
		);
	};

	const getMethodBadge = (method: string) => {
		const m = method.toUpperCase();
		switch (m) {
			case 'GET':
				return (
					<span className="rounded border border-emerald-500/20 bg-emerald-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-emerald-400">
						GET
					</span>
				);
			case 'POST':
				return (
					<span className="rounded border border-blue-500/20 bg-blue-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-blue-400">
						POST
					</span>
				);
			case 'PUT':
				return (
					<span className="rounded border border-amber-500/20 bg-amber-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-amber-400">
						PUT
					</span>
				);
			case 'DELETE':
				return (
					<span className="rounded border border-rose-500/20 bg-rose-500/15 px-2 py-0.5 font-mono text-[10px] font-bold text-rose-400">
						DEL
					</span>
				);
			default:
				return (
					<span className="rounded bg-muted px-2 py-0.5 font-mono text-[10px] font-bold text-muted-foreground">
						{m}
					</span>
				);
		}
	};

	return (
		<div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
			<div className="flex-1 overflow-auto">
				<Table>
					<TableHeader className="sticky top-0 z-10 bg-muted/40 backdrop-blur-md">
						<TableRow className="border-b border-border/50 hover:bg-transparent">
							{visibleColumns.method && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Method
								</TableHead>
							)}
							{visibleColumns.path && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Request Path
								</TableHead>
							)}
							{visibleColumns.status && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Status
								</TableHead>
							)}
							{visibleColumns.latency && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Latency
								</TableHead>
							)}
							{visibleColumns.client_ip && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Client IP
								</TableHead>
							)}
							{visibleColumns.service && (
								<TableHead className="px-3.5 py-3 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Target Service
								</TableHead>
							)}
							{visibleColumns.time && (
								<TableHead className="px-3.5 py-3 text-right text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
									Time
								</TableHead>
							)}
						</TableRow>
					</TableHeader>
					<TableBody className="divide-y divide-border/20 font-mono text-[11px]">
						{isLoading && items.length === 0 ? (
							<TableRow>
								<TableCell
									colSpan={visibleCount}
									className="py-16 text-center font-sans text-xs text-muted-foreground">
									<RefreshCw className="mx-auto mb-2 size-5 animate-spin text-muted-foreground/70" />
									Loading Traefik request logs...
								</TableCell>
							</TableRow>
						) : items.length === 0 ? (
							<TableRow>
								<TableCell
									colSpan={visibleCount}
									className="py-16 text-center font-sans text-xs text-muted-foreground">
									No request logs matching the selected filters.
								</TableCell>
							</TableRow>
						) : (
							items.map((log, i) => (
								<TableRow
									key={`${log.timestamp}-${i}`}
									className="transition-colors hover:bg-muted/30">
									{visibleColumns.method && (
										<TableCell className="px-3.5 py-2.5">
											{getMethodBadge(log.method)}
										</TableCell>
									)}
									{visibleColumns.path && (
										<TableCell
											className="max-w-xs truncate px-3.5 py-2.5 font-semibold text-foreground"
											title={log.path}>
											{log.path}
										</TableCell>
									)}
									{visibleColumns.status && (
										<TableCell className="px-3.5 py-2.5">
											{getStatusBadge(log.status)}
										</TableCell>
									)}
									{visibleColumns.latency && (
										<TableCell className="px-3.5 py-2.5 font-medium text-muted-foreground">
											{log.duration_ms.toFixed(1)}ms
										</TableCell>
									)}
									{visibleColumns.client_ip && (
										<TableCell className="px-3.5 py-2.5 text-muted-foreground/80">
											{log.client_ip}
										</TableCell>
									)}
									{visibleColumns.service && (
										<TableCell
											className="max-w-[140px] truncate px-3.5 py-2.5 text-muted-foreground/70"
											title={log.service_name}>
											{log.service_name !== '-'
												? log.service_name
												: log.router_name}
										</TableCell>
									)}
									{visibleColumns.time && (
										<TableCell className="px-3.5 py-2.5 text-right text-[10px] text-muted-foreground/60">
											{log.timestamp
												? new Date(log.timestamp).toLocaleTimeString()
												: '-'}
										</TableCell>
									)}
								</TableRow>
							))
						)}
					</TableBody>
				</Table>
			</div>

			<div className="flex shrink-0 items-center justify-between border-t border-border/40 bg-muted/20 px-4 py-2.5 font-sans text-xs text-muted-foreground">
				<span>
					Showing Page <strong className="text-foreground">{page}</strong>{' '}
					of <strong className="text-foreground">{totalPages}</strong> (
					{totalCount} total)
				</span>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						disabled={page <= 1}
						onClick={() => onPageChange(Math.max(1, page - 1))}
						className="h-7 cursor-pointer px-3 text-xs font-medium">
						Previous
					</Button>
					<Button
						variant="outline"
						size="sm"
						disabled={page >= totalPages}
						onClick={() => onPageChange(Math.min(totalPages, page + 1))}
						className="h-7 cursor-pointer px-3 text-xs font-medium">
						Next
					</Button>
				</div>
			</div>
		</div>
	);
}
