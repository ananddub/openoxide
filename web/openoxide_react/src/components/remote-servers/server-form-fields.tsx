import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

import type {SshKeyResponse} from '#/types/api-helpers';

interface ServerFormFieldsProps {
	name: string;
	setName: (val: string) => void;
	ipAddress: string;
	setIpAddress: (val: string) => void;
	port: string;
	setPort: (val: string) => void;
	username: string;
	setUsername: (val: string) => void;
	sshKeyId: string;
	setSshKeyId: (val: string) => void;
	description: string;
	setDescription: (val: string) => void;
	sshKeys: SshKeyResponse[];
}

export function ServerFormFields({
	name,
	setName,
	ipAddress,
	setIpAddress,
	port,
	setPort,
	username,
	setUsername,
	sshKeyId,
	setSshKeyId,
	description,
	setDescription,
	sshKeys,
}: ServerFormFieldsProps) {
	return (
		<>
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Server Name *
				</label>
				<Input
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="e.g. EU Worker Node 1"
					className="h-10 rounded-md border-border bg-background px-3 text-xs"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					IP Address / Hostname *
				</label>
				<Input
					value={ipAddress}
					onChange={e => setIpAddress(e.target.value)}
					placeholder="192.168.1.100 or node.yourdomain.com"
					className="h-10 rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					SSH Port
				</label>
				<Input
					value={port}
					onChange={e => setPort(e.target.value)}
					placeholder="22"
					className="h-10 rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Username *
				</label>
				<Input
					value={username}
					onChange={e => setUsername(e.target.value)}
					placeholder="root"
					className="h-10 rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					SSH Key Credential
				</label>
				<Select
					value={sshKeyId}
					onValueChange={val => val && setSshKeyId(val)}>
					<SelectTrigger className="flex !h-10 w-full items-center justify-between rounded-md border-border bg-background px-3 font-sans text-xs">
						<SelectValue placeholder="Select SSH Key">
							{sshKeys?.find(k => Number(k.id) === Number(sshKeyId))
								?.name || 'Select SSH Key'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{sshKeys?.map(key => (
							<SelectItem
								key={key.id}
								value={String(key.id)}
								className="font-sans text-xs">
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Description (Optional)
				</label>
				<Input
					value={description}
					onChange={e => setDescription(e.target.value)}
					placeholder="Production worker node in Hetzner"
					className="h-10 rounded-md border-border bg-background px-3 text-xs"
				/>
			</div>
		</>
	);
}
