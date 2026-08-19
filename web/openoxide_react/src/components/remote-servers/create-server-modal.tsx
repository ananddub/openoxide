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
import {
	Server,
	RefreshCw,
	CheckCircle2,
	XCircle,
	Plug,
} from 'lucide-react';

import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

interface CreateServerModalProps {
	isOpen: boolean;
	sshKeys: SshKeyResponse[];
	editingServer?: RemoteServerResponse | null;
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
	const [testResult, setTestResult] = useState<
		'idle' | 'success' | 'failed'
	>('idle');

	const createMutation = $api.useMutation('post', '/remote-servers');
	const patchMutation = $api.useMutation('patch', '/remote-servers/{id}');
	const testDirectMutation = $api.useMutation(
		'post',
		'/servers/test-direct-connection' as any,
	);

	useEffect(() => {
		setTestResult('idle');
		if (editingServer) {
			setName(editingServer.name || '');
			setIpAddress(editingServer.ip_address || '');
			setPort(String(editingServer.port || 22));
			setUsername(editingServer.username || 'root');
			setSshKeyId(
				editingServer.ssh_key_id ? String(editingServer.ssh_key_id) : '',
			);
			setDescription(editingServer.description || '');
		} else {
			setName('');
			setIpAddress('');
			setPort('22');
			setUsername('root');
			setSshKeyId(
				sshKeys && sshKeys.length > 0 ? String(sshKeys[0].id) : '',
			);
			setDescription('');
		}
	}, [editingServer, isOpen, sshKeys]);

	const handleTestConnection = async () => {
		if (!ipAddress || !username) {
			toast.error(
				'IP Address and Username are required to test connection',
			);
			return;
		}
		setTestingConn(true);
		setTestResult('idle');
		try {
			await testDirectMutation.mutateAsync({
				body: {
					ip_address: ipAddress,
					port: parseInt(port) || 22,
					username,
					ssh_key_id: sshKeyId ? parseInt(sshKeyId) : undefined,
				} as any,
			});
			setTestResult('success');
			toast.success(`SSH Connection to "${name || ipAddress}" verified!`);
		} catch (err: unknown) {
			setTestResult('failed');
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

			if (editingServer?.id) {
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
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="w-full rounded-2xl border-border bg-card p-6 shadow-2xl sm:max-w-2xl md:max-w-3xl">
				<DialogHeader className="border-b border-border/50 pb-4">
					<DialogTitle className="flex items-center gap-2.5 text-base font-bold text-foreground">
						<div className="rounded-xl bg-primary/10 p-2 text-primary">
							<Server className="h-5 w-5" />
						</div>
						{editingServer ? 'Edit Remote Server' : 'Add Remote Server'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure host IP address, SSH port, and credentials for remote
						deployment node
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="mt-2 flex flex-col gap-4">
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

					<div className="mt-1 flex items-center justify-between border-t border-border/50 pt-4">
						{/* Clean Icon-Only Test Connection Button */}
						<Button
							type="button"
							variant="outline"
							size="icon"
							onClick={handleTestConnection}
							disabled={testingConn || submitting}
							title={
								testingConn
									? 'Testing SSH Connection...'
									: testResult === 'success'
										? 'SSH Connection Verified'
										: testResult === 'failed'
											? 'SSH Connection Failed'
											: 'Test SSH Connection'
							}
							className={`h-9 w-9 rounded-lg transition-all ${
								testResult === 'success'
									? 'border-emerald-500/50 bg-emerald-500/10 text-emerald-500 hover:bg-emerald-500/20'
									: testResult === 'failed'
										? 'border-rose-500/50 bg-rose-500/10 text-rose-500 hover:bg-rose-500/20'
										: 'border-border/80 hover:bg-muted/80'
							}`}>
							{testingConn ? (
								<RefreshCw className="h-4 w-4 animate-spin text-primary" />
							) : testResult === 'success' ? (
								<CheckCircle2 className="h-4 w-4 text-emerald-500" />
							) : testResult === 'failed' ? (
								<XCircle className="h-4 w-4 text-rose-500" />
							) : (
								<Plug className="h-4 w-4 text-muted-foreground hover:text-foreground" />
							)}
						</Button>

						<Button
							type="submit"
							disabled={submitting || testingConn}
							className="h-9 px-6 text-xs font-semibold shadow-md">
							{submitting
								? 'Saving...'
								: editingServer
									? 'Save Changes'
									: 'Add Server'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
