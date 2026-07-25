import {useState, useEffect} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Server} from 'lucide-react';

interface CreateServerModalProps {
	isOpen: boolean;
	sshKeys: any[];
	editingServer?: any | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function CreateServerModal({
	isOpen,
	sshKeys,
	editingServer,
	onClose,
	onSuccess,
}: CreateServerModalProps) {
	const [name, setName] = useState('');
	const [ipAddress, setIpAddress] = useState('');
	const [port, setPort] = useState('22');
	const [username, setUsername] = useState('root');
	const [sshKeyId, setSshKeyId] = useState<string>('');
	const [description, setDescription] = useState('');
	const [submitting, setSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/remote-servers');
	const patchMutation = $api.useMutation('patch', '/remote-servers/{id}');

	useEffect(() => {
		if (editingServer) {
			setName(editingServer.name || '');
			setIpAddress(editingServer.ip_address || '');
			setPort(String(editingServer.port || 22));
			setUsername(editingServer.username || 'root');
			setSshKeyId(editingServer.ssh_key_id ? String(editingServer.ssh_key_id) : '');
			setDescription(editingServer.description || '');
		} else {
			setName('');
			setIpAddress('');
			setPort('22');
			setUsername('root');
			setSshKeyId(sshKeys && sshKeys.length > 0 ? String(sshKeys[0].id) : '');
			setDescription('');
		}
	}, [editingServer, isOpen, sshKeys]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !ipAddress || !username) {
			toast.error('Server Name, IP Address, and Username are required');
			return;
		}
		setSubmitting(true);
		try {
			const payload = {
				name,
				ip_address: ipAddress,
				port: parseInt(port) || 22,
				username,
				ssh_key_id: sshKeyId ? parseInt(sshKeyId) : undefined,
				server_type: 'REMOTE',
				description: description || undefined,
			};

			if (editingServer) {
				await patchMutation.mutateAsync({
					params: {path: {id: editingServer.id}},
					body: payload as any,
				});
				toast.success('Remote Server updated successfully');
			} else {
				await createMutation.mutateAsync({body: payload as any});
				toast.success('Remote Server added successfully');
			}
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Server className="w-5 h-5 text-primary" />
						{editingServer ? 'Edit Remote Server' : 'Add Remote Server'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure host IP address, SSH port, and credentials for remote Linux deployment node
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-2">
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
						<Select value={sshKeyId} onValueChange={val => setSshKeyId(val)}>
							<SelectTrigger className="h-10 text-xs font-sans bg-background border-border rounded-md w-full px-3">
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

					<div className="flex items-center justify-end gap-3 pt-4 border-t border-border/40 mt-1">
						<Button type="submit" disabled={submitting} className="h-9 text-xs font-semibold px-6">
							{submitting ? 'Saving...' : editingServer ? 'Save Changes' : 'Add Server'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
