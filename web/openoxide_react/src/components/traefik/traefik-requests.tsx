import {useState, useMemo} from 'react';
import {RefreshCw} from 'lucide-react';
import {
	useTraefikRequestLogs,
	useTraefikRequestsStatus,
	useTraefikRequestsToggle,
} from '#/api/live';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {RemoteServerItem} from './traefik-types';
import {RequestsHeader} from './requests/requests-header';
import {RequestsFilterBar} from './requests/requests-filter-bar';
import {RequestsActivatePanel} from './requests/requests-activate-panel';
import {matchesStatusFilter} from './requests/request-filters';
import {
	RequestsTable,
	type TraefikLogEntry,
} from './requests/requests-table';

interface TraefikRequestsProps {
	selectedServerId: string;
	onSelectServer?: (id: string) => void;
	servers?: RemoteServerItem[];
}

export function TraefikRequestsView({
	selectedServerId,
	onSelectServer,
	servers = [],
}: TraefikRequestsProps) {
	const [page, setPage] = useState(1);
	const [searchQuery, setSearchQuery] = useState('');
	const [statusFilter, setStatusFilter] = useState<string>('all');
	const [cronInput, setCronInput] = useState('0 0 * * *');
	const [isToggling, setIsToggling] = useState(false);

	const [visibleColumns, setVisibleColumns] = useState<
		Record<string, boolean>
	>({
		method: true,
		path: true,
		status: true,
		latency: true,
		client_ip: true,
		service: true,
		time: true,
	});

	const toggleColumn = (colId: string) => {
		setVisibleColumns(prev => {
			const count = Object.values(prev).filter(Boolean).length;
			if (prev[colId] && count <= 1) {
				toast.warning('At least one column must remain visible');
				return prev;
			}
			return {...prev, [colId]: !prev[colId]};
		});
	};

	const parsedServerId =
		selectedServerId !== 'local' ? Number(selectedServerId) : undefined;

	// Query Request Activation Status
	const {
		data: rawStatus,
		isLoading: isStatusLoading,
		refetch: refetchStatus,
	} = useTraefikRequestsStatus();

	const status = useMemo(() => {
		if (!rawStatus)
			return {is_active: false, cron_expression: '0 0 * * *'};
		const s = rawStatus as unknown as Record<string, unknown>;
		return {
			is_active: Boolean(s.is_active),
			cron_expression: String(s.cron_expression || '0 0 * * *'),
		};
	}, [rawStatus]);

	// Query Requests Logs Table
	const {
		data: rawLogs,
		isLoading: isLogsLoading,
		refetch: refetchLogs,
	} = useTraefikRequestLogs(parsedServerId ? BigInt(parsedServerId) : 0n);

	const logsData = useMemo(() => {
		if (!rawLogs)
			return {
				items: [] as TraefikLogEntry[],
				total_count: 0,
				page: 1,
				page_size: 25,
			};
		const l = rawLogs as unknown as Record<string, unknown>;
		const items = Array.isArray(l.items)
			? (l.items as TraefikLogEntry[])
			: [];

		const filteredItems = items.filter(item =>
			matchesStatusFilter(item, statusFilter),
		);

		return {
			items: filteredItems,
			total_count: filteredItems.length,
			page: Number(l.page || 1),
			page_size: Number(l.page_size || 25),
		};
	}, [rawLogs, statusFilter]);

	const toggleMutation = useTraefikRequestsToggle();

	const handleToggle = async (enable: boolean) => {
		setIsToggling(true);
		try {
			// The ToggleRequests RPC takes only the enable flag; the request log stream itself is per-server.
			await toggleMutation.mutateAsync({enabled: enable});
			toast.success(
				enable
					? 'Requests logging activated successfully!'
					: 'Requests logging deactivated.',
			);
			refetchStatus();
			if (enable) refetchLogs();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsToggling(false);
		}
	};

	if (isStatusLoading) {
		return (
			<div className="flex h-full flex-1 flex-col items-center justify-center gap-3 p-12 shadow-xs">
				<RefreshCw className="size-6 animate-spin text-muted-foreground" />
				<span className="text-xs font-medium text-muted-foreground">
					Checking Traefik requests status...
				</span>
			</div>
		);
	}

	return (
		<div className="flex h-full flex-1 flex-col gap-4 overflow-hidden">
			<RequestsHeader
				isActive={status.is_active}
				isToggling={isToggling}
				onToggle={handleToggle}
				cronInput={cronInput}
				onCronChange={setCronInput}
				selectedServerId={selectedServerId}
				onSelectServer={onSelectServer}
				servers={servers}
			/>

			<div className="flex min-h-0 flex-1 flex-col gap-3.5 overflow-hidden">
				{!status.is_active ? (
					<RequestsActivatePanel
						isToggling={isToggling}
						onActivate={() => handleToggle(true)}
					/>
				) : (
					<div className="flex min-h-0 flex-1 flex-col gap-3.5 overflow-hidden">
						<RequestsFilterBar
							searchQuery={searchQuery}
							onSearchChange={val => {
								setSearchQuery(val);
								setPage(1);
							}}
							statusFilter={statusFilter}
							onStatusFilterChange={setStatusFilter}
							visibleColumns={visibleColumns}
							onToggleColumn={toggleColumn}
							totalCount={logsData.total_count}
							isLoading={isLogsLoading}
							onRefresh={() => refetchLogs()}
						/>

						<RequestsTable
							items={logsData.items}
							totalCount={logsData.total_count}
							page={page}
							pageSize={25}
							isLoading={isLogsLoading}
							visibleColumns={visibleColumns}
							onPageChange={setPage}
						/>
					</div>
				)}
			</div>
		</div>
	);
}
