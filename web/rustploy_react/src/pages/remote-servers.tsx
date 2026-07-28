import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {RemoteServersHeader} from '#/components/remote-servers/remote-servers-header';
import {RemoteServersList} from '#/components/remote-servers/remote-servers-list';
import {CreateServerModal} from '#/components/remote-servers/create-server-modal';
import {SetupServerModal} from '#/components/remote-servers/setup-server-modal';
import {DeleteServerModal} from '#/components/remote-servers/delete-server-modal';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

import type {RemoteServerResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/remote-servers')({
	component: RemoteServersPage,
});

function RemoteServersPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingServer, setEditingServer] = useState<RemoteServerResponse | null>(null);
	const [deletingServer, setDeletingServer] = useState<RemoteServerResponse | null>(null);
	const [setupServer, setSetupServer] = useState<RemoteServerResponse | null>(null);
	const [terminalServer, setTerminalServer] = useState<RemoteServerResponse | null>(null);

	const {
		data: rawServers = [],
		isLoading: isServersLoading,
		refetch,
		isRefetching,
	} = $api.useQuery('get', '/remote-servers');

	const {data: rawSshKeys = []} = $api.useQuery('get', '/ssh-keys');

	const servers = Array.isArray(rawServers) ? (rawServers as RemoteServerResponse[]) : [];
	const sshKeys = Array.isArray(rawSshKeys) ? rawSshKeys : [];

	const activateMutation = $api.useMutation('post', '/remote-servers/{id}/activate');
	const deactivateMutation = $api.useMutation('post', '/remote-servers/{id}/deactivate');

	const handleToggleStatus = async (server: RemoteServerResponse) => {
		const isActive = (server.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';
		try {
			if (isActive) {
				await deactivateMutation.mutateAsync({params: {path: {id: server.id}}});
				toast.success(`Server "${server.name}" disabled`);
			} else {
				await activateMutation.mutateAsync({params: {path: {id: server.id}}});
				toast.success(`Server "${server.name}" activated`);
			}
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-6 p-6 max-w-7xl mx-auto w-full">
			<RemoteServersHeader
				onOpenCreate={() => {
					setEditingServer(null);
					setIsCreateOpen(true);
				}}
				onRefresh={refetch}
				isRefetching={isRefetching}
			/>

			<RemoteServersList
				servers={servers}
				sshKeys={sshKeys}
				isLoading={isServersLoading}
				onEditServer={server => {
					setEditingServer(server);
					setIsCreateOpen(true);
				}}
				onDeleteServer={server => setDeletingServer(server)}
				onSetupServer={server => setSetupServer(server)}
				onToggleStatus={handleToggleStatus}
				onOpenTerminal={server => setTerminalServer(server)}
			/>

			<CreateServerModal
				isOpen={isCreateOpen}
				sshKeys={sshKeys}
				editingServer={editingServer}
				onClose={() => {
					setIsCreateOpen(false);
					setEditingServer(null);
				}}
				onSuccess={refetch}
			/>

			<SetupServerModal
				isOpen={!!setupServer}
				server={setupServer}
				onClose={() => setSetupServer(null)}
			/>

			<DeleteServerModal
				isOpen={!!deletingServer}
				server={deletingServer}
				onClose={() => setDeletingServer(null)}
				onSuccess={refetch}
			/>

			<TerminalModal
				app={{
					app_name: terminalServer?.ip_address || terminalServer?.name || 'server',
					name: terminalServer?.name || 'Remote Server',
					server_id: terminalServer?.id,
					isRemoteServer: true,
				}}
				open={!!terminalServer}
				onClose={() => setTerminalServer(null)}
			/>
		</div>
	);
}
