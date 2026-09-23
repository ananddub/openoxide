import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {SectionHeader} from '#/components/settings/list-card';
import {
	useRemoteServerList,
	useRemoteServerActivate,
	useRemoteServerDeactivate,
	useSshKeyItems,
} from '#/api/live';
import {RemoteServersList} from '#/components/remote-servers/remote-servers-list';
import {CreateServerModal} from '#/components/remote-servers/create-server-modal';
import {SetupServerModal} from '#/components/remote-servers/setup-server-modal';
import {MigrateServerModal} from '#/components/remote-servers/migrate-server-modal';
import {DeleteServerModal} from '#/components/remote-servers/delete-server-modal';
import {TerminalCard} from '#/components/shared/terminal-card';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/remote-servers')({
	component: RemoteServersPage,
});

function RemoteServersPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingServer, setEditingServer] =
		useState<RemoteServerResponse | null>(null);
	const [deletingServer, setDeletingServer] =
		useState<RemoteServerResponse | null>(null);
	const [setupServer, setSetupServer] =
		useState<RemoteServerResponse | null>(null);
	const [terminalServer, setTerminalServer] =
		useState<RemoteServerResponse | null>(null);
	const [migratingServer, setMigratingServer] =
		useState<RemoteServerResponse | null>(null);

	// Live servers query; the table-watch stream keeps it fresh.
	const serversQuery = useRemoteServerList();
	const sshKeysQuery = useSshKeyItems();

	const servers = (serversQuery.data ??
		[]) as unknown as RemoteServerResponse[];
	const sshKeys = (sshKeysQuery.data ?? []) as unknown as SshKeyResponse[];
	const isServersLoading = serversQuery.isLoading;

	const activateMutation = useRemoteServerActivate();
	const deactivateMutation = useRemoteServerDeactivate();

	const handleToggleStatus = async (server: RemoteServerResponse) => {
		const isActive =
			(server.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';
		try {
			if (isActive) {
				await deactivateMutation.mutateAsync(server.id);
				toast.success(`Server "${server.name}" disabled`);
			} else {
				await activateMutation.mutateAsync(server.id);
				toast.success(`Server "${server.name}" activated`);
			}
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Remote Servers"
				description="Bare-metal and VPS hosts Openoxide provisions and deploys your services onto."
				action={
					<Button
						onClick={() => {
							setEditingServer(null);
							setIsCreateOpen(true);
						}}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Server
					</Button>
				}
			/>

			<RemoteServersList
				servers={servers}
				isLoading={isServersLoading}
				onAddServer={() => {
					setEditingServer(null);
					setIsCreateOpen(true);
				}}
				onEditServer={server => {
					setEditingServer(server);
					setIsCreateOpen(true);
				}}
				onDeleteServer={server => setDeletingServer(server)}
				onSetupServer={server => setSetupServer(server)}
				onToggleStatus={handleToggleStatus}
				onOpenTerminal={server => setTerminalServer(server)}
				onMigrateServer={server => setMigratingServer(server)}
			/>

			<CreateServerModal
				isOpen={isCreateOpen}
				sshKeys={sshKeys}
				editingServer={editingServer}
				onClose={() => {
					setIsCreateOpen(false);
					setEditingServer(null);
				}}
				onSuccess={() => {}}
			/>

			<SetupServerModal
				isOpen={!!setupServer}
				server={setupServer}
				onClose={() => setSetupServer(null)}
			/>

			<MigrateServerModal
				isOpen={!!migratingServer}
				server={migratingServer}
				servers={servers}
				onClose={() => setMigratingServer(null)}
			/>

			<DeleteServerModal
				isOpen={!!deletingServer}
				server={deletingServer}
				onClose={() => setDeletingServer(null)}
				onSuccess={() => {}}
			/>

			{terminalServer && (
				<TerminalCard
					app={{
						app_name:
							terminalServer.ip_address || terminalServer.name || 'server',
						name: terminalServer.name || 'Remote Server',
						server_id: terminalServer.id,
						isRemoteServer: true,
					}}
					enabled={true}
					onClose={() => setTerminalServer(null)}
				/>
			)}
		</div>
	);
}
