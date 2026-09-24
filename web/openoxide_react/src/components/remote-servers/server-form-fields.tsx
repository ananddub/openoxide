import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {WizardField} from '#/components/shared/sheet-wizard';

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
			<WizardField label="Server Name" htmlFor="server-name" required>
				<Input
					id="server-name"
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="e.g. EU Worker Node 1"
				/>
			</WizardField>

			<WizardField
				label="IP Address / Hostname"
				htmlFor="server-ip"
				required>
				<Input
					id="server-ip"
					value={ipAddress}
					onChange={e => setIpAddress(e.target.value)}
					placeholder="192.168.1.100 or node.yourdomain.com"
					className="font-mono"
				/>
			</WizardField>

			<WizardField label="SSH Port" htmlFor="server-port">
				<Input
					id="server-port"
					value={port}
					onChange={e => setPort(e.target.value)}
					placeholder="22"
					className="font-mono"
				/>
			</WizardField>

			<WizardField label="Username" htmlFor="server-user" required>
				<Input
					id="server-user"
					value={username}
					onChange={e => setUsername(e.target.value)}
					placeholder="root"
					className="font-mono"
				/>
			</WizardField>

			<WizardField label="SSH Key Credential" htmlFor="server-ssh-key">
				<Select
					value={sshKeyId}
					onValueChange={val => val && setSshKeyId(val)}>
					<SelectTrigger id="server-ssh-key">
						<SelectValue placeholder="Select SSH Key">
							{sshKeys?.find(
								k => Number(k.id) === Number(sshKeyId),
							)?.name || 'Select SSH Key'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{sshKeys?.map(key => (
							<SelectItem key={key.id} value={String(key.id)}>
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</WizardField>

			<WizardField
				label="Description"
				htmlFor="server-description"
				optional>
				<Input
					id="server-description"
					value={description}
					onChange={e => setDescription(e.target.value)}
					placeholder="Production worker node in Hetzner"
				/>
			</WizardField>
		</>
	);
}
