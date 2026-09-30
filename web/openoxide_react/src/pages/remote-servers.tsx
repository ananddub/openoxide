import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {ListRegion, SectionHeader} from '#/components/settings/list-card';
import {useRemoteServerList, useSshKeyItems} from '#/api/live';
import {RemoteServersList} from '#/components/remote-servers/remote-servers-list';
import {ServerSettingsPanel} from '#/components/remote-servers/server-settings-panel';
import {CreateServerModal} from '#/components/remote-servers/create-server-modal';

import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/remote-servers')({
	component: RemoteServersPage,
});

function RemoteServersPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [settingsServer, setSettingsServer] =
		useState<RemoteServerResponse | null>(null);

	// Live servers query; the table-watch stream keeps it fresh.
	const serversQuery = useRemoteServerList();
	const sshKeysQuery = useSshKeyItems();

	const servers = (serversQuery.data ??
		[]) as unknown as RemoteServerResponse[];
	const sshKeys = (sshKeysQuery.data ?? []) as unknown as SshKeyResponse[];
	const isServersLoading = serversQuery.isLoading;

	// Keep the panel's recorded row fresh while it is open.
	const liveSettingsServer = settingsServer
		? (servers.find(s => s.id === settingsServer.id) ?? settingsServer)
		: null;

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Remote Servers"
				description="Bare-metal and VPS hosts Openoxide provisions and deploys your services onto."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Server
					</Button>
				}
			/>

			<ListRegion>
				<RemoteServersList
					servers={servers}
					isLoading={isServersLoading}
					onAddServer={() => setIsCreateOpen(true)}
					onOpenSettings={server => setSettingsServer(server)}
				/>
			</ListRegion>

			{/* Monitoring for one server — live vitals + trends. */}
			<ServerSettingsPanel
				isOpen={!!liveSettingsServer}
				server={liveSettingsServer}
				onClose={() => setSettingsServer(null)}
			/>

			<CreateServerModal
				isOpen={isCreateOpen}
				sshKeys={sshKeys}
				editingServer={null}
				onClose={() => setIsCreateOpen(false)}
				onSuccess={() => {}}
			/>
		</div>
	);
}
