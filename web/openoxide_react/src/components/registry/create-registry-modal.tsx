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

import type {
	RegistryResponse,
	RemoteServerResponse,
} from '#/types/api-helpers';
import {useAppStore} from '#/stores/app-store';

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

	const serversData = useAppStore(state => state.servers);
	const servers = (Array.isArray(serversData)
		? serversData
		: []) as unknown as RemoteServerResponse[];

	const createMutation = $api.useMutation('post', '/registries');
	const patchMutation = $api.useMutation('patch', '/registries/{id}');
	const testMutation = $api.useMutation('post', '/registries/{id}/test');
	const testRawMutation = $api.useMutation('post', '/registries/test-raw');

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
			if (initialData?.id && !password) {
				await testMutation.mutateAsync({
					params: {path: {id: Number(initialData.id)}},
				});
			} else {
				if (!password) {
					toast.error('Password is required to test connection');
					setIsTesting(false);
					return;
				}
				await testRawMutation.mutateAsync({
					body: {
						registry_url: registryUrl,
						username,
						password,
					} as any,
				});
			}
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
					params: {path: {id: Number(initialData.id)}},
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
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-h-[90vh] w-full overflow-y-auto rounded-2xl border-border bg-card p-6 shadow-2xl sm:max-w-xl">
				<DialogHeader className="border-b border-border/50 pb-3">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
						<Database className="h-5 w-5 text-primary" />
						{initialData ? 'Edit Registry' : 'Add an external registry'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Fill in the following fields to add an external registry.
					</DialogDescription>
				</DialogHeader>

				<form
					onSubmit={handleSubmit}
					className="mt-3 flex flex-col gap-4 text-xs">
					{/* Registry Name */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Registry Name *
						</Label>
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
							className="h-9 font-mono text-xs"
						/>
						<p className="text-[11px] text-muted-foreground">
							Enter only the hostname (e.g.,{' '}
							<code className="font-mono text-foreground">
								aws_account_id.dkr.ecr.us-west-2.amazonaws.com
							</code>{' '}
							or{' '}
							<code className="font-mono text-foreground">docker.io</code>
							).
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
						<Label className="text-xs font-semibold">
							Password * {initialData && '(Leave blank to keep unchanged)'}
						</Label>
						<Input
							type="password"
							placeholder={
								initialData ? '••••••••' : 'Access Token / Password'
							}
							value={password}
							onChange={e => setPassword(e.target.value)}
							required={!initialData}
							className="h-9 font-mono text-xs"
						/>
					</div>

					{/* Server Authentication Host Dropdown */}
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Authenticate From Server
						</Label>
						<Select
							value={serverId}
							onValueChange={val => val && setServerId(val)}>
							<SelectTrigger className="flex !h-9 w-full items-center justify-between border-border/80 bg-card px-3 font-sans text-xs">
								<SelectValue placeholder="Select server">
									{serverId === 'local'
										? 'Local Server (Default)'
										: (() => {
												const srv = servers.find(
													(s: RemoteServerResponse) =>
														String(s.id) === String(serverId),
												);
												return srv ? srv.name : 'Select server';
											})()}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="z-50 border-border bg-card text-xs">
								<SelectItem value="local">
									Local Server (Default)
								</SelectItem>
								{servers.map((srv: RemoteServerResponse) => (
									<SelectItem key={srv.id} value={String(srv.id)}>
										{srv.name}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<p className="text-[11px] text-muted-foreground">
							Choose where to authenticate with the registry. By default,
							authentication occurs on the local server. Select a specific
							server to authenticate from that server instead.
						</p>
					</div>

					{/* Minimal Footer Buttons */}
					<div className="mt-2 flex items-center justify-between border-t border-border/50 pt-4">
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
			</DialogContent>
		</Dialog>
	);
}
