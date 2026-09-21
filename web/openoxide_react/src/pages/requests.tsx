import {useState, useMemo} from 'react';
import {useRemoteServerList} from '#/api/live';
import {TraefikRequestsView} from '#/components/traefik/traefik-requests';
import type {RemoteServerItem} from '#/components/traefik/traefik-types';

// Rendered as the "Requests" tab of pages/infra.tsx — no standalone route.

export function RequestsPanel() {
	const [selectedServerId, setSelectedServerId] =
		useState<string>('local');

	// Live servers query; the table-watch stream keeps it fresh.
	const {data: rawServers} = useRemoteServerList();
	const servers: RemoteServerItem[] = useMemo(() => {
		const list = Array.isArray(rawServers) ? rawServers : [];
		return list.map((item: unknown) => {
			const s = item as Record<string, unknown>;
			return {
				id: Number(s.id || 0),
				name: String(s.name || `Server ${s.id}`),
				ip_address: s.ip_address ? String(s.ip_address) : undefined,
			};
		});
	}, [rawServers]);

	return (
		<div className="flex h-full min-h-0 w-full flex-1 flex-col gap-2 overflow-hidden">
			{/* Requests Analytics View with Single Integrated Titlebar */}
			<TraefikRequestsView
				selectedServerId={selectedServerId}
				onSelectServer={setSelectedServerId}
				servers={servers}
			/>
		</div>
	);
}
