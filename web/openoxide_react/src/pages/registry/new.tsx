import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {ArrowLeft, Database, Plug, RefreshCw} from 'lucide-react';
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
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerList,
	useRegistryCreate,
	useRegistryTest,
} from '#/api/live';
import type {RemoteServerResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/registry/new')({
	component: RegistryCreatePage,
});

function RegistryCreatePage() {
	const navigate = useNavigate();
	const [name, setName] = useState('');
	const [registryUrl, setRegistryUrl] = useState('');
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isTesting, setIsTesting] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const {data: serversData} = useRemoteServerList();
	const servers = (Array.isArray(serversData)
		? serversData
		: []) as unknown as RemoteServerResponse[];

	const createMutation = useRegistryCreate();
	const testRawMutation = useRegistryTest();

	const handleTestRaw = async () => {
		if (!registryUrl || !username) {
			toast.error('Image Prefix (URL) and Username are required to test');
			return;
		}
		if (!password) {
			toast.error('Password is required to test connection');
			return;
		}
		setIsTesting(true);
		try {
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
		if (!name || !registryUrl || !username || !password) {
			toast.error('Please fill in all required fields');
			return;
		}
		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name,
				registry_url: registryUrl,
				username,
				password,
				server_id: serverId === 'local' ? undefined : parseInt(serverId),
			});
			toast.success('Registry added successfully');
			void navigate({to: '/registry'});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Page header ── */}
			<div className="flex items-center gap-3">
				<button
					type="button"
					onClick={() => void navigate({to: '/registry'})}
					title="Back to Registry"
					className="flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
					<ArrowLeft className="size-4" />
				</button>
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted">
					<Database className="size-4 text-foreground/70" />
				</div>
				<div className="min-w-0">
					<h1 className="truncate text-2xl font-medium tracking-tight text-foreground/80">
						Add an external registry
					</h1>
					<p className="text-sm text-muted-foreground/70">
						Fill in the following fields to add an external registry.
					</p>
				</div>
			</div>

			{/* ── Form card ── */}
			<div className="mx-auto w-full min-w-0 max-w-2xl rounded-xl bg-card p-6 ring-1 ring-border/50">
				<form onSubmit={handleSubmit} className="flex flex-col gap-4 text-xs">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Registry Name *
						</Label>
						<Input
							placeholder="e.g. AWS ECR / Docker Hub / GHCR"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							autoFocus
							className="h-9 text-xs"
						/>
					</div>

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

					<div className="grid gap-4 sm:grid-cols-2">
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
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Password *</Label>
							<Input
								type="password"
								placeholder="Access Token / Password"
								value={password}
								onChange={e => setPassword(e.target.value)}
								required
								className="h-9 font-mono text-xs"
							/>
						</div>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Authenticate From Server
						</Label>
						<Select
							value={serverId}
							onValueChange={val => val && setServerId(val)}>
							<SelectTrigger className="flex !h-9 w-full items-center justify-between bg-card px-3 font-sans text-xs">
								<SelectValue placeholder="Select server">
									{serverId === 'local'
										? 'Local Server (Default)'
										: servers.find(
												(s: RemoteServerResponse) =>
													String(s.id) === String(serverId),
											)?.name || 'Select server'}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="z-50 bg-card text-xs">
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
							authentication occurs on the local server.
						</p>
					</div>

					<div className="mt-1 flex items-center justify-between border-t border-border/50 pt-4">
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
							disabled={submitting}
							className="h-9 px-5 text-xs font-semibold">
							{submitting ? 'Saving...' : 'Save Registry'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
