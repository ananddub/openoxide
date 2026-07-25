import {useState, useEffect} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {ServerFormFields} from '#/components/remote-servers/server-form-fields';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Server, ShieldCheck, RefreshCw} from 'lucide-react';

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
	const [testingConn, setTestingConn] = useState(false);

	const createMutation = $api.useMutation('post', '/remote-servers');
	const patchMutation = $api.useMutation('patch', '/remote-servers/{id}');
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

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

	const handleTestConnection = async () => {
		if (!ipAddress || !username) {
			toast.error('IP Address and Username are required to test connection');
			return;
		}
		setTestingConn(true);
		try {
			if (editingServer?.id) {
				await testConnMutation.mutateAsync({
					params: {path: {id: editingServer.id}},
					body: {host_key_fingerprint: ''} as any,
				});
				toast.success(`SSH Connection to "${name || ipAddress}" verified successfully!`);
			} else {
				const newServer = await createMutation.mutateAsync({
					body: {
						name: name || `Server-${ipAddress}`,
						ip_address: ipAddress,
						port: parseInt(port) || 22,
						username,
						ssh_key_id: sshKeyId ? parseInt(sshKeyId) : undefined,
						server_type: 'DEPLOY',
						description: description || undefined,
					} as any,
				});
				await testConnMutation.mutateAsync({
					params: {path: {id: (newServer as any).id}},
					body: {host_key_fingerprint: ''} as any,
				});
				toast.success(`Server node added & SSH Connection verified successfully!`);
				onSuccess();
				onClose();
			}
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setTestingConn(false);
		}
	};

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
				server_type: 'DEPLOY',
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

					<div className="flex items-center justify-between pt-4 border-t border-border/40 mt-1">
						<Button
							type="button"
							variant="outline"
							onClick={handleTestConnection}
							disabled={testingConn || submitting}
							className="h-9 text-xs font-semibold gap-1.5"
						>
							{testingConn ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <ShieldCheck className="w-3.5 h-3.5 text-primary" />}
							{testingConn ? 'Testing...' : 'Test Connection'}
						</Button>

						<Button type="submit" disabled={submitting || testingConn} className="h-9 text-xs font-semibold px-6">
							{submitting ? 'Saving...' : editingServer ? 'Save Changes' : 'Add Server'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
