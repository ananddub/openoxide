import {useState, useMemo} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {useRemoteServerList} from 'virtual:openoxide-live';
import {TraefikRequestsView} from '#/components/traefik/traefik-requests';
import type {RemoteServerItem} from '#/components/traefik/traefik-types';

export const Route = createFileRoute('/_app/requests')({
	component: RequestsPage,
});

function RequestsPage() {
	const [selectedServerId, setSelectedServerId] = useState<string>('local');

	// Query Remote Servers list
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
		<div className="flex flex-col gap-2 w-full h-[calc(100vh-7rem)] p-3 animate-in fade-in duration-200">
			{/* Requests Analytics View with Single Integrated Titlebar */}
			<TraefikRequestsView
				selectedServerId={selectedServerId}
				onSelectServer={setSelectedServerId}
				servers={servers}
			/>
		</div>
	);
}
