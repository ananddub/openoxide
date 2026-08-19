import {Search, Filter, Columns3, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
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
	const visibleCount =
		Object.values(visibleColumns).filter(Boolean).length;

	return (
		<div className="flex shrink-0 flex-col items-center justify-between gap-3 sm:flex-row">
			<div className="flex w-full max-w-lg flex-1 items-center gap-2.5 sm:w-auto">
				<div className="relative flex-1">
					<Search className="absolute top-3 left-3 size-3.5 text-muted-foreground" />
					<Input
						value={searchQuery}
						onChange={e => onSearchChange(e.target.value)}
						placeholder="Filter path, method..."
						className="h-9 border-border/60 bg-card pl-9 font-mono text-xs shadow-2xs"
					/>
				</div>

				<Select
					value={statusFilter}
					onValueChange={v => onStatusFilterChange(v || 'all')}>
					<SelectTrigger className="h-9 w-[185px] shrink-0 gap-2 border-border/60 bg-card text-xs font-medium shadow-2xs">
						<Filter className="size-3.5 shrink-0 text-muted-foreground" />
						<SelectValue placeholder="Status Filter" />
					</SelectTrigger>
					<SelectContent className="w-[210px] border-border bg-card p-1.5 text-xs shadow-md">
						{STATUS_PRIORITIES.map(opt => (
							<SelectItem
								key={opt.value}
								value={opt.value}
								className="cursor-pointer text-xs font-medium">
								{opt.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			<div className="flex shrink-0 items-center gap-2">
				<DropdownMenu>
					<DropdownMenuTrigger className="inline-flex h-9 cursor-pointer items-center justify-center gap-2 rounded-md border border-border/60 bg-card px-3 text-xs font-medium shadow-2xs transition-colors hover:bg-accent">
						<Columns3 className="size-3.5 text-muted-foreground" /> Columns
						({visibleCount})
					</DropdownMenuTrigger>
					<DropdownMenuContent
						align="end"
						className="w-[190px] border-border bg-card p-1.5 text-xs shadow-md">
						{ALL_COLUMNS.map(col => (
							<DropdownMenuCheckboxItem
								key={col.id}
								checked={Boolean(visibleColumns[col.id])}
								onCheckedChange={() => onToggleColumn(col.id)}
								className="cursor-pointer rounded-sm px-2 py-1 text-xs font-medium">
								{col.label}
							</DropdownMenuCheckboxItem>
						))}
					</DropdownMenuContent>
				</DropdownMenu>

				<Badge
					variant="secondary"
					className="h-9 border border-border/40 px-3.5 font-mono text-xs font-semibold">
					Total:{' '}
					<strong className="ml-1 text-foreground">{totalCount}</strong>
				</Badge>

				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isLoading}
					className="h-9 cursor-pointer gap-2 border-border/60 px-3 text-xs font-medium shadow-2xs">
					<RefreshCw
						className={`size-3.5 text-muted-foreground ${isLoading ? 'animate-spin' : ''}`}
					/>{' '}
					Refresh
				</Button>
			</div>
		</div>
	);
}
