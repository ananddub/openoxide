import {Search, Filter, Columns3, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuCheckboxItem,
} from '#/components/ui/dropdown';

export const STATUS_PRIORITIES = [
	{label: 'All Statuses', value: 'all'},
	{label: '200 - 299 (Success)', value: '2xx'},
	{label: '300 - 399 (Redirect)', value: '3xx'},
	{label: '400 - 499 (Client Error)', value: '4xx'},
	{label: '500 - 599 (Server Error)', value: '5xx'},
	{label: '100 - 199 (Info)', value: '1xx'},
];

export const ALL_COLUMNS = [
	{id: 'method', label: 'Method'},
	{id: 'path', label: 'Request Path'},
	{id: 'status', label: 'Status'},
	{id: 'latency', label: 'Latency'},
	{id: 'client_ip', label: 'Client IP'},
	{id: 'service', label: 'Target Service'},
	{id: 'time', label: 'Time'},
];

interface RequestsFilterBarProps {
	searchQuery: string;
	onSearchChange: (val: string) => void;
	statusFilter: string;
	onStatusFilterChange: (val: string) => void;
	visibleColumns: Record<string, boolean>;
	onToggleColumn: (id: string) => void;
	totalCount: number;
	isLoading: boolean;
	onRefresh: () => void;
}

export function RequestsFilterBar({
	searchQuery,
	onSearchChange,
	statusFilter,
	onStatusFilterChange,
	visibleColumns,
	onToggleColumn,
	totalCount,
	isLoading,
	onRefresh,
}: RequestsFilterBarProps) {
	const visibleCount = Object.values(visibleColumns).filter(Boolean).length;

	return (
		<div className="flex flex-col sm:flex-row items-center justify-between gap-3 shrink-0">
			<div className="flex items-center gap-2.5 w-full sm:w-auto flex-1 max-w-lg">
				<div className="relative flex-1">
					<Search className="size-3.5 text-muted-foreground absolute left-3 top-3" />
					<Input
						value={searchQuery}
						onChange={(e) => onSearchChange(e.target.value)}
						placeholder="Filter path, method..."
						className="h-9 text-xs font-mono pl-9 bg-card border-border/60 shadow-2xs"
					/>
				</div>

				<Select value={statusFilter} onValueChange={(v) => onStatusFilterChange(v || 'all')}>
					<SelectTrigger className="w-[185px] h-9 text-xs font-medium bg-card border-border/60 gap-2 shrink-0 shadow-2xs">
						<Filter className="size-3.5 text-muted-foreground shrink-0" />
						<SelectValue placeholder="Status Filter" />
					</SelectTrigger>
					<SelectContent className="bg-card border-border text-xs w-[210px] p-1.5 shadow-md">
						{STATUS_PRIORITIES.map((opt) => (
							<SelectItem key={opt.value} value={opt.value} className="text-xs font-medium cursor-pointer">
								{opt.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			<div className="flex items-center gap-2 shrink-0">
				<DropdownMenu>
					<DropdownMenuTrigger className="inline-flex items-center justify-center gap-2 h-9 px-3 text-xs font-medium border border-border/60 bg-card hover:bg-accent rounded-md cursor-pointer transition-colors shadow-2xs">
						<Columns3 className="size-3.5 text-muted-foreground" /> Columns ({visibleCount})
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="bg-card border-border w-[190px] text-xs p-1.5 shadow-md">
						{ALL_COLUMNS.map((col) => (
							<DropdownMenuCheckboxItem
								key={col.id}
								checked={Boolean(visibleColumns[col.id])}
								onCheckedChange={() => onToggleColumn(col.id)}
								className="text-xs font-medium cursor-pointer py-1 px-2 rounded-sm">
								{col.label}
							</DropdownMenuCheckboxItem>
						))}
					</DropdownMenuContent>
				</DropdownMenu>

				<Badge variant="secondary" className="h-9 text-xs font-mono px-3.5 border border-border/40 font-semibold">
					Total: <strong className="ml-1 text-foreground">{totalCount}</strong>
				</Badge>

				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isLoading}
					className="h-9 px-3 text-xs font-medium border-border/60 gap-2 cursor-pointer shadow-2xs">
					<RefreshCw className={`size-3.5 text-muted-foreground ${isLoading ? 'animate-spin' : ''}`} /> Refresh
				</Button>
			</div>
		</div>
	);
}
