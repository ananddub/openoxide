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
import {Server, ShieldCheck, RefreshCw, CheckCircle2, AlertCircle} from 'lucide-react';

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
	const [testStatus, setTestStatus] = useState<'idle' | 'success' | 'failed'>('idle');
	const [draftCreatedId, setDraftCreatedId] = useState<number | null>(null);

	const createMutation = $api.useMutation('post', '/remote-servers');
	const patchMutation = $api.useMutation('patch', '/remote-servers/{id}');
	const deleteMutation = $api.useMutation('delete', '/remote-servers/{id}');
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	useEffect(() => {
		setTestStatus('idle');
		setDraftCreatedId(null);
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

	const handleCancelClose = async () => {
		if (draftCreatedId) {
			try {
				await deleteMutation.mutateAsync({params: {path: {id: draftCreatedId}}});
				onSuccess();
			} catch (_) {}
		}
		setDraftCreatedId(null);
		onClose();
	};

	const handleTestConnection = async () => {
		if (!ipAddress || !username) {
			toast.error('IP Address and Username are required to test connection');
			return;
		}
		setTestingConn(true);
		setTestStatus('idle');
		try {
			let targetId = editingServer?.id || draftCreatedId;
			if (!targetId) {
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
				targetId = (newServer as any).id;
				setDraftCreatedId(targetId);
			} else {
				await patchMutation.mutateAsync({
					params: {path: {id: targetId}},
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
			}

			await testConnMutation.mutateAsync({
				params: {path: {id: targetId}},
				body: {host_key_fingerprint: ''} as any,
			});
			setTestStatus('success');
			toast.success(`SSH Connection to "${name || ipAddress}" verified successfully!`);
		} catch (err: any) {
			setTestStatus('failed');
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

			const targetId = editingServer?.id || draftCreatedId;
			if (targetId) {
				await patchMutation.mutateAsync({
					params: {path: {id: targetId}},
					body: payload as any,
				});
				toast.success(editingServer ? 'Remote Server updated successfully' : 'Remote Server added successfully');
			} else {
				await createMutation.mutateAsync({body: payload as any});
				toast.success('Remote Server added successfully');
			}
			setDraftCreatedId(null);
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleCancelClose()}>
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card/95 backdrop-blur-md border-border/80 p-6 shadow-2xl rounded-2xl">
				<DialogHeader className="pb-4 border-b border-border/50">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2.5">
						<div className="p-2 rounded-xl bg-primary/10 text-primary">
							<Server className="w-5 h-5" />
						</div>
						{editingServer ? 'Edit Remote Server' : 'Add Remote Server'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure host IP address, SSH port, and authentication credentials for remote Linux deployment node
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

					{testStatus !== 'idle' && (
						<div className={`p-3 rounded-xl border text-xs font-semibold flex items-center gap-2 transition-all ${testStatus === 'success' ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-500' : 'bg-rose-500/10 border-rose-500/30 text-rose-500'}`}>
							{testStatus === 'success' ? <CheckCircle2 className="w-4 h-4 shrink-0" /> : <AlertCircle className="w-4 h-4 shrink-0" />}
							{testStatus === 'success' ? 'SSH Connection Verified & Active' : 'SSH Connection Test Failed. Check credentials and firewall.'}
						</div>
					)}

					<div className="flex items-center justify-between pt-4 border-t border-border/50 mt-1">
						<Button
							type="button"
							variant="outline"
							onClick={handleTestConnection}
							disabled={testingConn || submitting}
							className="h-9 text-xs font-semibold gap-2 border-border/80 hover:bg-muted/80"
						>
							{testingConn ? <RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" /> : <ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />}
							{testingConn ? 'Testing SSH...' : 'Test Connection'}
						</Button>

						<div className="flex items-center gap-2">
							<Button type="button" variant="ghost" onClick={handleCancelClose} className="h-9 text-xs font-semibold px-4">
								Cancel
							</Button>
							<Button type="submit" disabled={submitting || testingConn} className="h-9 text-xs font-semibold px-6 shadow-md">
								{submitting ? 'Saving...' : editingServer ? 'Save Changes' : 'Add Server'}
							</Button>
						</div>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
