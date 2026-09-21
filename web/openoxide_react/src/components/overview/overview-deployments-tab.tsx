import {Rocket, Loader2, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	TabSearchInput,
	TabFilterSelect,
	TabEmptyState,
	TabTable,
	type FilterOption,
} from './overview-tab-parts';
import {DeploymentItem} from '#/components/deployments/deployment-item';

const STATUS_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All Statuses'},
	{value: 'success', label: 'Success'},
	{value: 'failed', label: 'Failed'},
	{value: 'running', label: 'Running'},
];

function DeploymentTable({
	items,
	onSelect,
	onCancel,
	onDelete,
}: {
	items: any[];
	onSelect: (dep: any) => void;
	onCancel: (id: any) => void;
	onDelete?: (id: any) => void;
}) {
	return (
		<TabTable
			columns={[
				'ID',
				'Deployment',
				'Type',
				'Status',
				'Created',
				'Actions',
			]}>
			{items.map((dep: any) => (
				<DeploymentItem
					key={dep.id}
					deployment={dep}
					onViewLogs={() => onSelect(dep)}
					onViewError={() => onSelect(dep)}
					onCancel={onCancel}
					onDelete={onDelete}
				/>
			))}
		</TabTable>
	);
}

export function OverviewDeploymentsTab({
	history,
	queue,
	subTab,
	onSubTab,
	search,
	onSearch,
	status,
	onStatus,
	onClear,
	onSelect,
	onCancel,
	onDelete,
}: {
	history: any[];
	queue: any[];
	subTab: 'history' | 'queue';
	onSubTab: (v: 'history' | 'queue') => void;
	search: string;
	onSearch: (v: string) => void;
	status: string;
	onStatus: (v: string) => void;
	onClear: () => void;
	onSelect: (dep: any) => void;
	onCancel: (id: any) => void;
	onDelete: (id: any) => void;
}) {
	return (
		<div className="space-y-4">
			<div className="flex flex-wrap items-center justify-between gap-4 border-b  pb-3">
				<div className="flex items-center gap-2">
					<Button
						variant={subTab === 'history' ? 'secondary' : 'ghost'}
						size="sm"
						onClick={() => onSubTab('history')}
						className="h-8 text-xs font-medium">
						Deployments ({history.length})
					</Button>
					<Button
						variant={subTab === 'queue' ? 'secondary' : 'ghost'}
						size="sm"
						onClick={() => onSubTab('queue')}
						className="flex h-8 items-center gap-1.5 text-xs font-medium">
						{queue.length > 0 && (
							<Loader2 className="size-3 animate-spin text-amber-500" />
						)}
						Queue ({queue.length})
					</Button>
				</div>

				{subTab === 'history' && (
					<div className="flex flex-wrap items-center gap-2.5">
						<TabSearchInput
							placeholder="Filter deployments..."
							value={search}
							onChange={onSearch}
						/>
						<TabFilterSelect
							title="Status"
							value={status}
							onChange={onStatus}
							options={STATUS_OPTIONS}
							width="w-[150px]"
						/>
						<Button
							variant="outline"
							size="sm"
							onClick={onClear}
							className="flex h-9 items-center gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
							<Trash2 className="size-3.5" /> Clear History
						</Button>
					</div>
				)}
			</div>

			{subTab === 'history' ? (
				history.length === 0 ? (
					<TabEmptyState
						icon={Rocket}
						title="No deployment history found"
						description="Deployments will appear here once triggered"
					/>
				) : (
					<DeploymentTable
						items={history}
						onSelect={onSelect}
						onCancel={onCancel}
						onDelete={onDelete}
					/>
				)
			) : queue.length === 0 ? (
				<TabEmptyState
					icon={Rocket}
					title="Queue is empty"
					description="Active or building deployments will appear here in real time"
				/>
			) : (
				<DeploymentTable
					items={queue}
					onSelect={onSelect}
					onCancel={onCancel}
				/>
			)}
		</div>
	);
}
