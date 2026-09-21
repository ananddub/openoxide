import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useRemoteServerList} from '#/api/live';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface RegistryFormFieldsProps {
	name: string;
	setName: (v: string) => void;
	registryUrl: string;
	setRegistryUrl: (v: string) => void;
	username: string;
	setUsername: (v: string) => void;
	password: string;
	setPassword: (v: string) => void;
	serverId: string;
	setServerId: (v: string) => void;
	editing?: boolean;
}

/** Controlled field set shared by the registry modal and create page. */
export function RegistryFormFields({
	name,
	setName,
	registryUrl,
	setRegistryUrl,
	username,
	setUsername,
	password,
	setPassword,
	serverId,
	setServerId,
	editing,
}: RegistryFormFieldsProps) {
	const {data: serversData} = useRemoteServerList();
	const servers = (Array.isArray(serversData)
		? serversData
		: []) as unknown as RemoteServerResponse[];

	const currentServerName =
		serverId === 'local'
			? 'Local Server (Default)'
			: servers.find(
					(s: RemoteServerResponse) => String(s.id) === String(serverId),
				)?.name || 'Select server';

	return (
		<>
			{/* Registry Name */}
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-semibold">Registry Name *</Label>
				<Input
					placeholder="e.g. AWS ECR / Docker Hub / GHCR"
					value={name}
					onChange={e => setName(e.target.value)}
					required
					className="h-9 text-xs"
				/>
			</div>

			{/* Image Prefix / Hostname */}
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-semibold">Image Prefix *</Label>
				<Input
					placeholder="aws_account_id.dkr.ecr.us-west-2.amazonaws.com or docker.io"
					value={registryUrl}
					onChange={e => setRegistryUrl(e.target.value)}
					required
					className="h-9 font-mono text-xs"
				/>
				<p className="text-[11px] text-muted-foreground">
					Enter only the hostname (e.g.,{' '}
					<code className="font-mono text-foreground">
						aws_account_id.dkr.ecr.us-west-2.amazonaws.com
					</code>{' '}
					or <code className="font-mono text-foreground">docker.io</code>
					).
				</p>
			</div>

			{/* Username + Password */}
			<div className="grid gap-4 sm:grid-cols-2">
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-semibold">Username *</Label>
					<Input
						placeholder="AWS / Docker Hub Username"
						value={username}
						onChange={e => setUsername(e.target.value)}
						required
						className="h-9 text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-semibold">
						Password * {editing && '(Leave blank to keep unchanged)'}
					</Label>
					<Input
						type="password"
						placeholder={editing ? '••••••••' : 'Access Token / Password'}
						value={password}
						onChange={e => setPassword(e.target.value)}
						required={!editing}
						className="h-9 font-mono text-xs"
					/>
				</div>
			</div>

			{/* Server Authentication Host */}
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-semibold">
					Authenticate From Server
				</Label>
				<Select
					value={serverId}
					onValueChange={val => val && setServerId(val)}>
					<SelectTrigger className="flex !h-9 w-full items-center justify-between bg-card px-3 font-sans text-xs">
						<SelectValue placeholder="Select server">
							{currentServerName}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="z-50 bg-card text-xs">
						<SelectItem value="local">Local Server (Default)</SelectItem>
						{servers.map((srv: RemoteServerResponse) => (
							<SelectItem key={srv.id} value={String(srv.id)}>
								{srv.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
				<p className="text-[11px] text-muted-foreground">
					Choose where to authenticate with the registry. By default,
					authentication occurs on the local server.
				</p>
			</div>
		</>
	);
}
