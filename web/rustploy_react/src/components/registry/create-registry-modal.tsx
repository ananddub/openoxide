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
	const [registryUrl, setRegistryUrl] = useState('');
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isTesting, setIsTesting] = useState(false);

	const {data: serversData} = $api.useQuery('get', '/remote-servers');
	const servers = Array.isArray(serversData) ? serversData : [];

	const createMutation = $api.useMutation('post', '/registries');
	const patchMutation = $api.useMutation('patch', '/registries/{id}');
	const testRawMutation = $api.useMutation('post', '/registries/test-raw');

	useEffect(() => {
		if (initialData) {
			setName(initialData.name || '');
			setRegistryUrl(initialData.registry_url || '');
			setUsername(initialData.username || '');
			setPassword('');
			setServerId(initialData.server_id ? String(initialData.server_id) : 'local');
		} else {
			setName('');
			setRegistryUrl('');
			setUsername('');
			setPassword('');
			setServerId('local');
		}
	}, [initialData, isOpen]);

	const handleTestRaw = async () => {
		if (!registryUrl || !username || !password) {
			toast.error('Image Prefix (URL), Username, and Password are required to test');
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
			toast.success('Registry authentication test passed!');
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
			const srvIdNum = serverId === 'local' ? undefined : parseInt(serverId);

			if (initialData?.id) {
				await patchMutation.mutateAsync({
					params: {path: {id: initialData.id}},
					body: {
						name,
						registry_url: registryUrl,
						username,
						...(password ? {password} : {}),
						server_id: srvIdNum,
					} as any,
				});
				toast.success('Registry updated successfully');
			} else {
				if (!password) {
					toast.error('Password is required');
					return;
				}
				await createMutation.mutateAsync({
					body: {
						name,
						registry_url: registryUrl,
						username,
						password,
						server_id: srvIdNum,
					} as any,
				});
				toast.success('Registry added successfully');
			}
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl w-full bg-card border-border p-6 shadow-2xl rounded-2xl max-h-[90vh] overflow-y-auto">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Database className="w-5 h-5 text-primary" />
						{initialData ? 'Edit Registry' : 'Add an external registry'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Fill in the following fields to add an external registry.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-3 text-xs">
					{/* Registry Name */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Registry Name *</Label>
						<Input
							placeholder="e.g. AWS ECR / Docker Hub / GHCR"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							className="h-9 text-xs"
						/>
					</div>

					{/* Image Prefix / Hostname */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Image Prefix *</Label>
						<Input
							placeholder="aws_account_id.dkr.ecr.us-west-2.amazonaws.com or docker.io"
							value={registryUrl}
							onChange={e => setRegistryUrl(e.target.value)}
							required
							className="h-9 text-xs font-mono"
						/>
						<p className="text-[11px] text-muted-foreground">
							Enter only the hostname (e.g., <code className="font-mono text-foreground">aws_account_id.dkr.ecr.us-west-2.amazonaws.com</code> or <code className="font-mono text-foreground">docker.io</code>).
						</p>
					</div>

					{/* Username */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Username *</Label>
						<Input
							placeholder="AWS / Docker Hub Username"
							value={username}
							onChange={e => setUsername(e.target.value)}
							required
							className="h-9 text-xs"
						/>
					</div>

					{/* Password */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Password * {initialData && '(Leave blank to keep unchanged)'}</Label>
						<Input
							type="password"
							placeholder={initialData ? '••••••••' : 'Access Token / Password'}
							value={password}
							onChange={e => setPassword(e.target.value)}
							required={!initialData}
							className="h-9 text-xs font-mono"
						/>
					</div>

					{/* Server Authentication Host Dropdown */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Authenticate From Server</Label>
						<Select value={serverId} onValueChange={val => val && setServerId(val)}>
							<SelectTrigger className="h-9 text-xs bg-card border-border/80">
								<SelectValue placeholder="Select server" />
							</SelectTrigger>
							<SelectContent className="bg-card border-border text-xs z-50">
								<SelectItem value="local">Local Server (Default)</SelectItem>
								{servers.map((srv: any) => (
									<SelectItem key={srv.id} value={String(srv.id)}>
										{srv.name} ({srv.ip_address})
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<p className="text-[11px] text-muted-foreground">
							Choose where to authenticate with the registry. By default, authentication occurs on the local server. Select a specific server to authenticate from that server instead.
						</p>
					</div>

					{/* Footer Buttons */}
					<div className="flex items-center justify-between pt-4 border-t border-border/50 mt-2">
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={handleTestRaw}
							disabled={isTesting}
							className="h-9 text-xs font-semibold gap-1.5"
						>
							{isTesting ? <RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" /> : <Plug className="w-3.5 h-3.5" />}
							Test Registry
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
