import {useState, useEffect} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Database, Plug, RefreshCw} from 'lucide-react';
import {
	useRegistryCreate,
	useRegistryUpdate,
	useRegistryTest,
} from '#/api/live';
import {RegistryFormFields} from './registry-form-fields';

import type {RegistryResponse} from '#/types/api-helpers';

interface CreateRegistryModalProps {
	isOpen: boolean;
	initialData?: RegistryResponse | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function CreateRegistryModal({
	isOpen,
	initialData,
	onClose,
	onSuccess,
}: CreateRegistryModalProps) {
	const [name, setName] = useState('');
	const [registryUrl, setRegistryUrl] = useState('');
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isTesting, setIsTesting] = useState(false);

	const createMutation = useRegistryCreate();
	const patchMutation = useRegistryUpdate();
	const testRawMutation = useRegistryTest();

	useEffect(() => {
		if (initialData) {
			const d = initialData as any;
			setName(d.name || d.registry_name || '');
			setRegistryUrl(d.registry_url || '');
			setUsername(d.username || '');
			setPassword('');
			setServerId(d.server_id ? String(d.server_id) : 'local');
		} else {
			setName('');
			setRegistryUrl('');
			setUsername('');
			setPassword('');
			setServerId('local');
		}
	}, [initialData, isOpen]);

	const handleTestRaw = async () => {
		if (!registryUrl || !username) {
			toast.error('Image Prefix (URL) and Username are required to test');
			return;
		}
		setIsTesting(true);
		try {
			// The RegistryService only exposes a raw-credentials test
			// (TestRegistry); there is no test-by-id RPC, so a password is
			// always required — even when editing an existing registry.
			if (!password) {
				toast.error('Password is required to test connection');
				setIsTesting(false);
				return;
			}
			await testRawMutation.mutateAsync({
				registry_url: registryUrl,
				username,
				password,
			});
			toast.success('Registry authentication test passed!');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsTesting(false);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !registryUrl || !username) {
			toast.error('Please fill in all required fields');
			return;
		}

		try {
			const srvIdNum =
				serverId === 'local' ? undefined : parseInt(serverId);

			if (initialData?.id) {
				await patchMutation.mutateAsync({
					id: Number(initialData.id),
					name,
					registry_url: registryUrl,
					username,
					...(password ? {password} : {}),
					server_id: srvIdNum,
				});
				toast.success('Registry updated successfully');
			} else {
				if (!password) {
					toast.error('Password is required');
					return;
				}
				await createMutation.mutateAsync({
					name,
					registry_url: registryUrl,
					username,
					password,
					server_id: srvIdNum,
				});
				toast.success('Registry added successfully');
			}
			onSuccess();
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Database className="size-4 text-primary" />
						{initialData ? 'Edit Registry' : 'Add an external registry'}
					</SheetTitle>
					<SheetDescription>
						Fill in the following fields to add an external registry.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSubmit}
					className="mt-3 flex flex-col gap-4 text-xs">
					<RegistryFormFields
						name={name}
						setName={setName}
						registryUrl={registryUrl}
						setRegistryUrl={setRegistryUrl}
						username={username}
						setUsername={setUsername}
						password={password}
						setPassword={setPassword}
						serverId={serverId}
						setServerId={setServerId}
						editing={!!initialData}
					/>

					{/* Minimal Footer Buttons */}
					<div className="mt-2 flex items-center justify-between border-t pt-4">
						<Button
							type="button"
							variant="outline"
							size="icon"
							onClick={handleTestRaw}
							disabled={isTesting}
							title="Test Registry Connection"
							className="h-9 w-9 shrink-0">
							{isTesting ? (
								<RefreshCw className="h-4 w-4 animate-spin text-primary" />
							) : (
								<Plug className="h-4 w-4 text-muted-foreground hover:text-foreground" />
							)}
						</Button>

						<Button
							type="submit"
							className="h-9 px-5 text-xs font-semibold">
							{initialData ? 'Update Registry' : 'Save Registry'}
						</Button>
					</div>
				</form>
			</SheetContent>
		</Sheet>
	);
}
