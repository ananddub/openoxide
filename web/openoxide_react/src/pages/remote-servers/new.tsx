import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {Info, KeyRound, Network, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Titlebar} from '#/components/shared/titlebar';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {ServerFormFields} from '#/components/remote-servers/server-form-fields';
import {useRemoteServerCreate, useSshKeyItems} from '#/api/live';
import type {SshKeyResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/remote-servers/new')({
	component: AddServerPage,
});

function AddServerPage() {
	const navigate = useNavigate();
	const [name, setName] = useState('');
	const [ipAddress, setIpAddress] = useState('');
	const [port, setPort] = useState('22');
	const [username, setUsername] = useState('root');
	const [sshKeyId, setSshKeyId] = useState<string>('');
	const [description, setDescription] = useState('');
	const [submitting, setSubmitting] = useState(false);

	const sshKeysQuery = useSshKeyItems();
	const sshKeys = (sshKeysQuery.data ?? []) as unknown as SshKeyResponse[];
	const createMutation = useRemoteServerCreate();

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !ipAddress || !username) {
			toast.error('Server Name, IP Address, and Username are required');
			return;
		}
		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name,
				ip_address: ipAddress,
				port: parseInt(port) || 22,
				username,
				ssh_key_id: sshKeyId ? parseInt(sshKeyId) : undefined,
				server_type: 'DEPLOY',
				description: description || undefined,
			});
			toast.success('Remote Server added successfully');
			void navigate({to: '/remote-servers'});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			{/* ── Vercel-style titlebar ── */}
			<Titlebar
				breadcrumbs={[
					{
						label: 'Remote Servers',
						to: '/remote-servers',
						icon: Server,
					},
					{label: 'Add Server'},
				]}
			/>

			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				{/* ── Form card ── */}
				<form
					onSubmit={handleSubmit}
					className="flex min-w-0 flex-col gap-4 rounded-xl border bg-card p-6">
					<ServerFormFields
						name={name}
						setName={setName}
						ipAddress={ipAddress}
						setIpAddress={setIpAddress}
						port={port}
						setPort={setPort}
						username={username}
						setUsername={setUsername}
						sshKeyId={sshKeyId}
						setSshKeyId={setSshKeyId}
						description={description}
						setDescription={setDescription}
						sshKeys={sshKeys}
					/>

					<div className="mt-1 flex items-center justify-end gap-2 border-t border-border/50 pt-4">
						<Button
							type="button"
							variant="ghost"
							onClick={() => void navigate({to: '/remote-servers'})}
							className="h-9 text-xs">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={submitting}
							className="h-9 cursor-pointer px-6 text-xs font-semibold shadow-md">
							{submitting ? 'Saving...' : 'Add Server'}
						</Button>
					</div>
				</form>

				{/* ── Right rail: getting started ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<div className="overflow-hidden rounded-xl border bg-card">
						<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
								<Info className="size-[18px] text-muted-foreground" />
							</div>
							<div className="min-w-0">
								<h2 className="truncate text-[15px] font-semibold text-foreground">
									Getting Started
								</h2>
								<p className="text-xs text-muted-foreground">
									What you need before connecting
								</p>
							</div>
						</div>
						<ul className="space-y-3 p-5 text-sm text-muted-foreground">
							<li className="flex items-start gap-2">
								<Network className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									A Linux server reachable over the network — a VPS,
									bare-metal box, or a machine on your LAN.
								</span>
							</li>
							<li className="flex items-start gap-2">
								<KeyRound className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									SSH access: an authorized key (stored under SSH Keys) or
									password credentials for the account above.
								</span>
							</li>
							<li className="flex items-start gap-2">
								<Server className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									Docker on the host — after connecting, run Setup to
									install the monitoring agent and join the network.
								</span>
							</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	);
}
