import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {RefreshCw, Server} from 'lucide-react';
import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Titlebar} from '#/components/shared/titlebar';
import {useRemoteServerGet} from '#/api/live';
import {useServerMetrics} from '#/api/live/monitoring';
import {isLocalServerRow, serverHostLabel} from '#/api/utils';
import {TerminalCard} from '#/components/shared/terminal-card';
import {PrivateNetworkPanel} from '#/components/remote-servers/private-network';
import {GeistSwitch} from '#/components/shared/geist-switch';
import {ServerOverviewTab} from '#/components/remote-servers/server-overview-tab';
import {
	SERVER_TABS,
	SERVER_TAB_SWITCH_LABELS,
	isServerTabId,
	type ServerTabId,
} from '#/components/remote-servers/server-tabs';
import {
	ServerContainersPanel,
	ServerDiskPanel,
	ServerImagesPanel,
	ServerNetworksPanel,
	ServerVolumesPanel,
} from '#/components/remote-servers/docker';
import {ServerAgentPanel} from '#/components/remote-servers/agent';
import {ServerManagementPanel} from '#/components/remote-servers/management';
import {ServerSettingsTab} from '#/components/remote-servers/settings';
import type {RemoteServerResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/remote-servers/$id')({
	component: ServerDetailPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {tab?: ServerTabId} => ({
		tab:
			isServerTabId(search.tab) && search.tab !== 'overview'
				? search.tab
				: undefined,
	}),
});

function ServerDetailPage() {
	const {id} = Route.useParams();
	const {tab: rawTab} = useSearch({from: '/_app/remote-servers/$id'});
	const tab: ServerTabId = rawTab ?? 'overview';
	const navigate = useNavigate();
	const serverId = BigInt(id);
	const [refreshing, setRefreshing] = useState(false);

	const serverQuery = useRemoteServerGet(serverId);
	const metricsQuery = useServerMetrics(serverId);

	const server = serverQuery.data;
	const host = serverHostLabel(server?.ip_address || server?.ip);
	const status = (server?.server_status || 'INACTIVE').toUpperCase();
	const isHealthy = status === 'ACTIVE';

	// The agent serves samples newest-first (get_last_n → rev/take).
	const samples = metricsQuery.data ?? [];

	const handleRefresh = async () => {
		setRefreshing(true);
		try {
			await Promise.all([serverQuery.refetch(), metricsQuery.refetch()]);
		} finally {
			setRefreshing(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			{/* ── Titlebar: identity + status + tabs ───────────────────── */}
			<Titlebar
				breadcrumbs={[
					{label: 'Remote Servers', to: '/remote-servers', icon: Server},
					{label: server?.name || 'Loading...'},
				]}
				actions={
					<>
						<span
							className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
								isHealthy
									? 'bg-emerald-500/10 text-emerald-500'
									: 'bg-rose-500/10 text-rose-500'
							}`}>
							{isHealthy ? 'Healthy' : status.toLowerCase()}
						</span>
						<Button
							variant="outline"
							size="sm"
							onClick={handleRefresh}
							disabled={refreshing}>
							<RefreshCw
								className={`size-3.5 text-muted-foreground ${refreshing ? 'animate-spin' : ''}`}
							/>
							Refresh
						</Button>
					</>
				}>
				<GeistSwitch
					options={SERVER_TABS.map(tab => ({
						key: tab.id,
						label: SERVER_TAB_SWITCH_LABELS[tab.id],
						icon: tab.icon,
					}))}
					value={tab}
					onChange={key =>
						void navigate({
							to: '/remote-servers/$id',
							params: {id},
							search: {tab: key === 'overview' ? undefined : key},
							replace: true,
						})
					}
				/>
			</Titlebar>

			{/* ── One full-width column; each tab owns its own layout ─── */}
			{tab === 'overview' && (
				<ServerOverviewTab
					isLoading={metricsQuery.isLoading}
					isError={metricsQuery.isError}
					samples={samples}
					server={server ?? {}}
					host={host}
					isHealthy={isHealthy}
				/>
			)}

			{tab === 'network' && server && (
				<PrivateNetworkPanel
					server={server as unknown as RemoteServerResponse}
				/>
			)}

			{tab === 'containers' && (
				<ServerContainersPanel serverId={serverId} />
			)}

			{tab === 'images' && <ServerImagesPanel serverId={serverId} />}

			{tab === 'volumes' && <ServerVolumesPanel serverId={serverId} />}

			{tab === 'networks' && <ServerNetworksPanel serverId={serverId} />}

			{tab === 'disk' && <ServerDiskPanel serverId={serverId} />}

			{tab === 'agent' && <ServerAgentPanel serverId={serverId} />}

			{tab === 'management' && (
				<ServerManagementPanel serverId={serverId} />
			)}

			{tab === 'settings' && server && (
				<ServerSettingsTab server={server} />
			)}

			{tab === 'terminal' && server && (
				<TerminalCard
					app={{
						app_name: host,
						name: server.name || 'Server',
						server_id: Number(id),
						isRemoteServer: !isLocalServerRow(
							server.ip_address || server.ip,
						),
					}}
					enabled={true}
					asModal={false}
					hostModeDefault={isLocalServerRow(
						server.ip_address || server.ip,
					)}
				/>
			)}
		</div>
	);
}
