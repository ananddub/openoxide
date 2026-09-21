import {Globe, ExternalLink} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {TableRow, TableCell} from '#/components/ui/table';
import {
	TabSearchInput,
	TabFilterSelect,
	TabEmptyState,
	TabTable,
	type FilterOption,
} from './overview-tab-parts';

const SSL_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All'},
	{value: 'https', label: 'HTTPS Only'},
	{value: 'http', label: 'HTTP Only'},
];

const SORT_OPTIONS: FilterOption[] = [
	{value: 'newest', label: 'Newest First'},
	{value: 'oldest', label: 'Oldest First'},
];

function DomainRow({dom}: {dom: any}) {
	const fullUrl = `${dom.https ? 'https' : 'http'}://${dom.domain || dom.host}${dom.path || ''}`;
	return (
		<TableRow
			key={dom.id}
			className="border-b  transition-colors hover:bg-muted/40">
			<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
				<div className="flex items-center gap-2">
					<Globe className="size-4 shrink-0 text-primary" />
					<a
						href={fullUrl}
						target="_blank"
						rel="noreferrer"
						className="transition-colors hover:text-primary hover:underline">
						{dom.domain || dom.host}
					</a>
				</div>
			</TableCell>
			<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
				{dom.service_name || 'Application'}
			</TableCell>
			<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
				{dom.path || '/'} · Port: {dom.port || 80}
			</TableCell>
			<TableCell className="px-4 py-3.5">
				{dom.https ? (
					<Badge
						variant="outline"
						className="border-emerald-500/30 bg-emerald-500/10 text-[10px] font-semibold text-emerald-500">
						HTTPS (SSL)
					</Badge>
				) : (
					<Badge variant="secondary" className="text-[10px] font-semibold">
						HTTP
					</Badge>
				)}
			</TableCell>
			<TableCell className="px-4 py-3.5 text-xs font-medium text-muted-foreground">
				{dom.project_name || 'OpenOxide Project'}
			</TableCell>
			<TableCell className="px-4 py-3.5 text-right">
				<a
					href={fullUrl}
					target="_blank"
					rel="noreferrer"
					className="inline-flex items-center gap-1 text-xs font-semibold text-primary hover:underline">
					<ExternalLink className="size-3.5" />
					<span>Visit</span>
				</a>
			</TableCell>
		</TableRow>
	);
}

export function OverviewDomainsTab({
	items,
	search,
	onSearch,
	ssl,
	onSsl,
	sort,
	onSort,
}: {
	items: any[];
	search: string;
	onSearch: (v: string) => void;
	ssl: string;
	onSsl: (v: string) => void;
	sort: string;
	onSort: (v: string) => void;
}) {
	return (
		<div className="space-y-4">
			<div className="flex flex-wrap items-center justify-between gap-4">
				<h3 className="text-base font-semibold tracking-tight text-foreground">
					Domains{' '}
					<span className="text-sm font-normal text-muted-foreground">
						({items.length})
					</span>
				</h3>

				<div className="flex flex-wrap items-center gap-2.5">
					<TabSearchInput
						placeholder="Filter domains..."
						value={search}
						onChange={onSearch}
					/>
					<TabFilterSelect
						title="SSL"
						value={ssl}
						onChange={onSsl}
						options={SSL_OPTIONS}
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
					icon={Globe}
					title="No active domains configured"
					description="Configure custom domain routes in application settings"
				/>
			) : (
				<TabTable
					columns={[
						'Domain / Host',
						'Target Service',
						'Path & Port',
						'SSL Status',
						'Project',
						'Actions',
					]}>
					{items.map((dom: any) => (
						<DomainRow key={dom.id} dom={dom} />
					))}
				</TabTable>
			)}
		</div>
	);
}
