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
import {Label} from '#/components/ui/label';
import {Checkbox} from '#/components/ui/checkbox';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Database, Plug, RefreshCw} from 'lucide-react';

interface CreateRegistryModalProps {
	isOpen: boolean;
	initialData?: any | null;
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
	const [registryUrl, setRegistryUrl] = useState('docker.io');
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [isDefault, setIsDefault] = useState(false);
	const [isTesting, setIsTesting] = useState(false);

	const createMutation = $api.useMutation('post', '/registries');
	const patchMutation = $api.useMutation('patch', '/registries/{id}');
	const testRawMutation = $api.useMutation('post', '/registries/test-raw');

	useEffect(() => {
		if (initialData) {
			setName(initialData.name || '');
			setRegistryUrl(initialData.registry_url || 'docker.io');
			setUsername(initialData.username || '');
			setPassword('');
			setIsDefault(initialData.is_default || false);
		} else {
			setName('');
			setRegistryUrl('docker.io');
			setUsername('');
			setPassword('');
			setIsDefault(false);
		}
	}, [initialData, isOpen]);

	const handleTestRaw = async () => {
		if (!registryUrl || !username || !password) {
			toast.error('URL, Username, and Password are required to test connection');
			return;
		}
		setIsTesting(true);
		try {
			await testRawMutation.mutateAsync({
				body: {
					registry_url: registryUrl,
					username,
					password,
				} as any,
			});
			toast.success('Registry credentials verified successfully!');
		} catch (err: any) {
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
			if (initialData?.id) {
				await patchMutation.mutateAsync({
					params: {path: {id: initialData.id}},
					body: {
						name,
						registry_url: registryUrl,
						username,
						...(password ? {password} : {}),
						is_default: isDefault,
					} as any,
				});
				toast.success('Registry updated successfully');
			} else {
				if (!password) {
					toast.error('Password / Access Token is required');
					return;
				}
				await createMutation.mutateAsync({
					body: {
						name,
						registry_url: registryUrl,
						username,
						password,
						is_default: isDefault,
					} as any,
				});
				toast.success('Registry created successfully');
			}
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md w-full bg-card border-border p-6 shadow-2xl rounded-2xl">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Database className="w-5 h-5 text-primary" />
						{initialData ? 'Edit Registry' : 'Add Container Registry'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure Docker Hub, GHCR, or a private registry for deployments
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-3 text-xs">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Registry Name *</Label>
						<Input
							placeholder="e.g. Docker Hub, GHCR, Production Registry"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							className="h-9 text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Registry URL *</Label>
						<Input
							placeholder="e.g. docker.io, ghcr.io, registry.gitlab.com"
							value={registryUrl}
							onChange={e => setRegistryUrl(e.target.value)}
							required
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Username *</Label>
							<Input
								placeholder="e.g. goploy-bot"
								value={username}
								onChange={e => setUsername(e.target.value)}
								required
								className="h-9 text-xs"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Password / Token {initialData ? '(Optional)' : '*'}</Label>
							<Input
								type="password"
								placeholder={initialData ? '••••••••' : 'Access Token'}
								value={password}
								onChange={e => setPassword(e.target.value)}
								required={!initialData}
								className="h-9 text-xs font-mono"
							/>
						</div>
					</div>

					<div className="flex items-center gap-2 pt-1">
						<Checkbox
							id="is_default"
							checked={isDefault}
							onCheckedChange={checked => setIsDefault(!!checked)}
						/>
						<Label htmlFor="is_default" className="text-xs cursor-pointer select-none">
							Set as Default Registry
						</Label>
					</div>

					<div className="flex items-center justify-between pt-4 border-t border-border/50 mt-2">
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={handleTestRaw}
							disabled={isTesting}
							className="h-9 text-xs font-semibold gap-1.5"
						>
							{isTesting ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <Plug className="w-3.5 h-3.5" />}
							Test Connection
						</Button>

						<div className="flex items-center gap-2">
							<Button type="button" variant="ghost" onClick={onClose} className="h-9 text-xs font-semibold px-3">
								Cancel
							</Button>
							<Button type="submit" className="h-9 text-xs font-semibold px-4">
								{initialData ? 'Update' : 'Save'} Registry
							</Button>
						</div>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
