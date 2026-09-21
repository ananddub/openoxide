import {useState, useEffect} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {ServerFormFields} from '#/components/remote-servers/server-form-fields';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerCreate,
	useRemoteServerUpdate,
	useRemoteServerTestConnection,
} from '#/api/live';
import {Server} from 'lucide-react';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';
import {useServerForm} from './use-server-form';
import {
	TestConnectionButton,
	type TestResult,
} from './test-connection-button';

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
	const form = useServerForm({isOpen, editingServer, sshKeys});
	const {name, ipAddress, username, sshKeyId, port, description} = form;
	const [submitting, setSubmitting] = useState(false);
	const [testingConn, setTestingConn] = useState(false);
	const [testResult, setTestResult] = useState<TestResult>('idle');

	const createMutation = useRemoteServerCreate();
	const patchMutation = useRemoteServerUpdate();
	const testDirectMutation = useRemoteServerTestConnection();

	useEffect(() => {
		setTestResult('idle');
	}, [editingServer, isOpen]);

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
			// The ConnectRPC TestConnection RPC only tests a SAVED server by
			// id (no raw-credential test exists in the proto), so the server
			// must be saved before the connection can be verified.
			if (!editingServer?.id) {
				toast.error('Save the server first, then test the connection');
				return;
			}
			await testDirectMutation.mutateAsync(editingServer.id);
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
					id: editingServer.id,
					...payload,
				});
				toast.success('Remote Server updated successfully');
			} else {
				await createMutation.mutateAsync(payload);
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
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Server className="size-4 text-primary" />
						{editingServer ? 'Edit Remote Server' : 'Add Remote Server'}
					</SheetTitle>
					<SheetDescription>
						Configure host IP address, SSH port, and credentials for remote
						deployment node
					</SheetDescription>
				</SheetHeader>

				<form onSubmit={handleSubmit} className="mt-2 flex flex-col gap-4">
					<ServerFormFields {...form} sshKeys={sshKeys} />

					<div className="mt-1 flex items-center justify-between border-t  pt-4">
						<TestConnectionButton
							testing={testingConn}
							result={testResult}
							disabled={testingConn || submitting}
							onClick={() => void handleTestConnection()}
						/>

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
			</SheetContent>
		</Sheet>
	);
}
