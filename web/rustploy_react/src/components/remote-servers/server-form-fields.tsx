import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

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
	sshKeys: any[];
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
				<label className="text-xs font-semibold text-foreground">Server Name *</label>
				<Input
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="e.g. EU Worker Node 1"
					className="h-10 text-xs bg-background border-border rounded-md px-3"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">IP Address / Hostname *</label>
				<Input
					value={ipAddress}
					onChange={e => setIpAddress(e.target.value)}
					placeholder="192.168.1.100 or node.yourdomain.com"
					className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">SSH Port</label>
				<Input
					value={port}
					onChange={e => setPort(e.target.value)}
					placeholder="22"
					className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">Username *</label>
				<Input
					value={username}
					onChange={e => setUsername(e.target.value)}
					placeholder="root"
					className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
				/>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">SSH Key Credential</label>
				<Select value={sshKeyId} onValueChange={val => val && setSshKeyId(val)}>
					<SelectTrigger className="!h-10 text-xs font-sans bg-background border-border rounded-md w-full px-3 flex items-center justify-between">
						<SelectValue placeholder="Select SSH Key">
							{sshKeys?.find((k: any) => Number(k.id) === Number(sshKeyId))?.name || 'Select SSH Key'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{sshKeys?.map(key => (
							<SelectItem key={key.id} value={String(key.id)} className="text-xs font-sans">
								{key.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
				<Input
					value={description}
					onChange={e => setDescription(e.target.value)}
					placeholder="Production worker node in Hetzner"
					className="h-10 text-xs bg-background border-border rounded-md px-3"
				/>
			</div>
		</>
	);
}
