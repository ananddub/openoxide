import {useState} from 'react';
import {RefreshCw, Rocket, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import {GeistSwitch} from '#/components/shared/geist-switch';
import {
	TabSearchInput,
	TabFilterSelect,
	TabEmptyState,
	type FilterOption,
} from './overview-tab-parts';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
import {useAutoExpandBuildLogs} from '#/components/projects/detail/deployments/use-auto-expand-build-logs';

const STATUS_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All Statuses'},
	{value: 'success', label: 'Success'},
	{value: 'failed', label: 'Failed'},
	{value: 'running', label: 'Running'},
];

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
	onRefresh,
	refreshing,
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
	onRefresh: () => void;
	refreshing: boolean;
}) {
	// Row-click opens the streaming log panel inline (drag-resizable) —
	// one shared expand state so switching sub-tabs closes the other.
	const [expandedId, setExpandedId] = useState<number | null>(null);
	const items = subTab === 'history' ? history : queue;
	useAutoExpandBuildLogs(items, setExpandedId);

	return (
		<div className="space-y-4">
			<div className="flex flex-wrap items-center justify-between gap-4 border-b pb-3">
				<GeistSwitch
					options={[
						{
							key: 'history',
							label: 'Deployments',
							count: history.length,
						},
						{key: 'queue', label: 'Queue', count: queue.length},
					]}
					value={subTab}
					onChange={onSubTab}
				/>

				<div className="flex flex-wrap items-center gap-2.5">
					{subTab === 'history' && (
						<>
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
						</>
					)}
					<Button
						variant="ghost"
						size="sm"
						onClick={onRefresh}
						disabled={refreshing}
						className="flex h-9 items-center gap-1.5 text-xs font-medium text-muted-foreground hover:text-foreground">
						<RefreshCw
							className={cn(
								'size-3.5',
								refreshing && 'animate-spin',
							)}
						/>
						Refresh
					</Button>
					<Button
						variant="ghost"
						size="sm"
						onClick={onClear}
						className="flex h-9 items-center gap-1.5 text-xs font-medium text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
						<Trash2 className="size-3.5" /> Clear History
					</Button>
				</div>
			</div>

			{items.length === 0 ? (
				subTab === 'history' ? (
					<TabEmptyState
						icon={Rocket}
						title="No deployment history found"
						description="Deployments will appear here once triggered"
					/>
				) : (
					<TabEmptyState
						icon={Rocket}
						title="Queue is empty"
						description="Active or building deployments will appear here in real time"
					/>
				)
			) : (
				<DeploymentsList
					deployments={items}
					expandedId={expandedId}
					onToggleExpand={id =>
						setExpandedId(prev => (prev === id ? null : id))
					}
				/>
			)}
		</div>
	);
}
