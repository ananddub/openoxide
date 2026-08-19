import {useState, useMemo} from 'react';
import {RefreshCw, AlertCircle, Zap} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {RemoteServerItem} from './traefik-types';
import {RequestsHeader} from './requests/requests-header';
import {RequestsFilterBar} from './requests/requests-filter-bar';
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
	} = $api.useQuery('get', '/traefik/requests/status', {
		params: {
			query: {
				server_id: parsedServerId,
			} as any,
		},
	});

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
	} = $api.useQuery(
		'get',
		'/traefik/requests/logs',
		{
			params: {
				query: {
					server_id: parsedServerId,
					page,
					page_size: 25,
					search: searchQuery || undefined,
				} as any,
			},
		},
		{
			enabled: status.is_active,
		},
	);

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

		const filteredItems = items.filter(item => {
			if (statusFilter === 'all') return true;
			const code = Number(item.status);
			if (statusFilter === '1xx') return code >= 100 && code < 200;
			if (statusFilter === '2xx') return code >= 200 && code < 300;
			if (statusFilter === '3xx') return code >= 300 && code < 400;
			if (statusFilter === '4xx') return code >= 400 && code < 500;
			if (statusFilter === '5xx') return code >= 500 && code < 600;
			return true;
		});

		return {
			items: filteredItems,
			total_count: filteredItems.length,
			page: Number(l.page || 1),
			page_size: Number(l.page_size || 25),
		};
	}, [rawLogs, statusFilter]);

	const toggleMutation = $api.useMutation(
		'post',
		'/traefik/requests/toggle',
	);

	const handleToggle = async (enable: boolean) => {
		setIsToggling(true);
		try {
			await toggleMutation.mutateAsync({
				body: {
					server_id: parsedServerId,
					enable,
				},
			});
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
					<div className="mx-auto mt-10 mb-auto flex w-full max-w-xl flex-col items-center gap-3.5 py-6 text-center">
						<AlertCircle className="mb-1 size-8 text-muted-foreground/60" />
						<div className="space-y-1">
							<h3 className="text-sm font-bold text-foreground">
								Requests are not activated
							</h3>
							<p className="max-w-md text-xs leading-relaxed text-muted-foreground">
								Activate requests to see incoming traffic statistics and
								monitor your application's usage. After activation, reload
								Traefik for changes to take effect.
							</p>
						</div>
						<Button
							variant="default"
							size="sm"
							onClick={() => handleToggle(true)}
							disabled={isToggling}
							className="mt-2 h-9 cursor-pointer gap-2 px-5 text-xs font-semibold">
							{isToggling ? (
								<RefreshCw className="size-3.5 animate-spin" />
							) : (
								<Zap className="size-3.5 fill-primary-foreground" />
							)}
							Activate Requests
						</Button>
					</div>
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
