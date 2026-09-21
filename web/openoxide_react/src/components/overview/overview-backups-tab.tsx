import {Clock} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {TableRow, TableCell} from '#/components/ui/table';
import {
	TabSearchInput,
	TabFilterSelect,
	TabEmptyState,
	TabTable,
	type FilterOption,
} from './overview-tab-parts';

const TYPE_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All Types'},
	{value: 'volume', label: 'Volume'},
	{value: 'database', label: 'Database'},
];

const SORT_OPTIONS: FilterOption[] = [
	{value: 'newest', label: 'Newest First'},
	{value: 'oldest', label: 'Oldest First'},
];

export function OverviewBackupsTab({
	items,
	search,
	onSearch,
	type,
	onType,
	sort,
	onSort,
}: {
	items: any[];
	search: string;
	onSearch: (v: string) => void;
	type: string;
	onType: (v: string) => void;
	sort: string;
	onSort: (v: string) => void;
}) {
	return (
		<div className="space-y-4">
			<div className="flex flex-wrap items-center justify-between gap-4">
				<h3 className="text-base font-semibold tracking-tight text-foreground">
					Backups{' '}
					<span className="text-sm font-normal text-muted-foreground">
						({items.length})
					</span>
				</h3>

				<div className="flex flex-wrap items-center gap-2.5">
					<TabSearchInput
						placeholder="Filter backups..."
						value={search}
						onChange={onSearch}
					/>
					<TabFilterSelect
						title="Type"
						value={type}
						onChange={onType}
						options={TYPE_OPTIONS}
					/>
					<TabFilterSelect
						title="Sort"
						value={sort}
						onChange={onSort}
						options={SORT_OPTIONS}
						width="w-[155px]"
					/>
				</div>
			</div>

			{items.length === 0 ? (
				<TabEmptyState
					icon={Clock}
					title="No volume backups found"
					description="Scheduled volume and database backups will appear here"
				/>
			) : (
				<TabTable
					columns={[
						'Backup Name',
						'Type',
						'Status',
						'Destination',
						'Created',
						'Actions',
					]}>
					{items.map((b: any) => (
						<TableRow
							key={b.id}
							className="border-b  transition-colors hover:bg-muted/40">
							<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
								<div className="flex items-center gap-2">
									<Clock className="size-4 shrink-0 text-primary" />
									<span>{b.name || `Backup #${b.id}`}</span>
								</div>
							</TableCell>
							<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
								<Badge variant="outline" className="font-mono text-[10px]">
									{b.backup_type || 'Volume'}
								</Badge>
							</TableCell>
							<TableCell className="px-4 py-3.5">
								<div className="inline-flex items-center gap-1.5 rounded-md bg-success-bg px-2 py-1 text-xs font-medium text-success">
									<span className="size-1.5 shrink-0 rounded-full bg-success-solid" />
									<span>{b.status || 'DONE'}</span>
								</div>
							</TableCell>
							<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
								{b.destination || 'Local Storage'}
							</TableCell>
							<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
								{b.created_at
									? new Date(
											Number(b.created_at) * 1000,
										).toLocaleDateString()
									: 'N/A'}
							</TableCell>
							<TableCell className="px-4 py-3.5 text-right">
								<Button
									size="sm"
									variant="outline"
									className="h-7 px-2.5 text-xs font-semibold">
									Download
								</Button>
							</TableCell>
						</TableRow>
					))}
				</TabTable>
			)}
		</div>
	);
}
