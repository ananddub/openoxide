import {useState, useEffect} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Plug, RefreshCw, Save, Server as ServerIcon} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRegistryCreate,
	useRegistryTest,
	useRegistryUpdate,
	useRemoteServerList,
} from '#/api/live';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {
	OptionRow,
	StepBody,
	StepTimeline,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {SearchSelect} from '#/components/shared/search-select';

import type {RegistryResponse} from '#/types/api-helpers';

interface CreateRegistryModalProps {
	isOpen: boolean;
	initialData?: RegistryResponse | null;
	onClose: () => void;
	onSuccess: () => void;
}

const STEPS: WizardStep[] = [
	{id: 'registry', label: 'Registry'},
	{id: 'credentials', label: 'Credentials'},
	{id: 'server', label: 'Server'},
];

const FORM_ID = 'create-registry-sheet-form';

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
	const [stepIdx, setStepIdx] = useState(0);

	const createMutation = useRegistryCreate();
	const patchMutation = useRegistryUpdate();
	const testRawMutation = useRegistryTest();
	const {data: serversData} = useRemoteServerList();
	const servers = (Array.isArray(serversData)
		? serversData
		: []) as unknown as RemoteServerResponse[];

	const step = STEPS[stepIdx].id;
	const last = stepIdx === STEPS.length - 1;
	const editing = !!initialData;
	const formReady =
		!!name.trim() &&
		!!registryUrl.trim() &&
		!!username.trim() &&
		(!!password.trim() || editing);

	useEffect(() => {
		if (!isOpen) return;
		setStepIdx(0);
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
			// The RegistryService only exposes a raw-credentials test (TestRegistry); there is no test-by-id RPC, so a password is always required — even when editing an existing registry.
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

	const canContinue =
		step === 'registry'
			? !!name.trim() && !!registryUrl.trim()
			: step === 'credentials'
				? !!username.trim() && (!!password.trim() || !!initialData)
				: true;

	const stepDesc: Record<string, string> = {
		registry: 'Name the registry and give us the host we pull from.',
		credentials:
			'These are used for docker login when deploying your services.',
		server: 'Pick the server that authenticates against this registry.',
	};

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{initialData ? 'Edit Registry' : 'New External Registry'}
					</SheetTitle>
					<SheetDescription>
						{editing
							? 'Everything on one page — change only what you need.'
							: stepDesc[step]}
					</SheetDescription>
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex-1 overflow-y-auto px-6 py-6">
					{editing ? (
						<div className="flex flex-col gap-8">
							<div className="flex flex-col gap-5">
								<WizardField label="Registry Name" required>
									<Input
										placeholder="e.g. AWS ECR / Docker Hub / GHCR"
										value={name}
										onChange={e => setName(e.target.value)}
									/>
								</WizardField>
								<WizardField
									label="Image Prefix"
									required
									hint={
										<>
											Enter only the hostname (e.g.,{' '}
											<code className="font-mono text-foreground">
												aws_account_id.dkr.ecr.us-west-2.amazonaws.com
											</code>{' '}
											or{' '}
											<code className="font-mono text-foreground">
												docker.io
											</code>
											).
										</>
									}>
									<Input
										placeholder="aws_account_id.dkr.ecr.us-west-2.amazonaws.com"
										value={registryUrl}
										onChange={e => setRegistryUrl(e.target.value)}
										className="font-mono"
									/>
								</WizardField>
							</div>

							<div className="flex flex-col gap-5">
								<WizardField label="Username" required>
									<Input
										placeholder="AWS / Docker Hub username"
										value={username}
										onChange={e => setUsername(e.target.value)}
									/>
								</WizardField>
								<WizardField
									label="Password / Access Token"
									optional
									hint="Leave blank to keep the stored token.">
									<Input
										type="password"
										placeholder="••••••••"
										value={password}
										onChange={e => setPassword(e.target.value)}
										className="font-mono"
									/>
								</WizardField>
								<button
									type="button"
									onClick={handleTestRaw}
									disabled={isTesting}
									className="flex items-center justify-center gap-2 rounded-lg border border-border/60 px-4 py-2.5 text-[13px] font-medium text-muted-foreground transition-colors hover:bg-muted/30 hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
									{isTesting ? (
										<RefreshCw className="size-3.5 animate-spin text-amber-500" />
									) : (
										<Plug className="size-3.5" />
									)}
									{isTesting ? 'Testing…' : 'Test Connection'}
								</button>
							</div>

							<WizardField
								label="Server"
								hint="Which server authenticates against this registry.">
								<SearchSelect
									value={serverId}
									onChange={setServerId}
									placeholder="Pick the authenticating server"
									searchPlaceholder="Search servers…"
									options={[
										{
											id: 'local',
											title: 'Local Server',
											subtitle: 'Authenticate on this server (default).',
											icon: <ServerIcon />,
										},
										...servers.map((s: RemoteServerResponse) => ({
											id: String(s.id),
											title: s.name,
											subtitle: s.ip_address,
											icon: <ServerIcon />,
										})),
									]}
								/>
							</WizardField>
						</div>
					) : (
						<StepTimeline steps={STEPS} idx={stepIdx}>
							{step === 'registry' && (
								<StepBody key="registry">
									<div className="flex flex-col gap-5">
										<WizardField label="Registry Name" required>
											<Input
												placeholder="e.g. AWS ECR / Docker Hub / GHCR"
												value={name}
												onChange={e => setName(e.target.value)}
												autoFocus
											/>
										</WizardField>
										<WizardField
											label="Image Prefix"
											required
											hint={
												<>
													Enter only the hostname (e.g.,{' '}
													<code className="font-mono text-foreground">
														aws_account_id.dkr.ecr.us-west-2.amazonaws.com
													</code>{' '}
													or{' '}
													<code className="font-mono text-foreground">
														docker.io
													</code>
													).
												</>
											}>
											<Input
												placeholder="aws_account_id.dkr.ecr.us-west-2.amazonaws.com"
												value={registryUrl}
												onChange={e => setRegistryUrl(e.target.value)}
												className="font-mono"
											/>
										</WizardField>
									</div>
								</StepBody>
							)}

							{step === 'credentials' && (
								<StepBody key="credentials">
									<div className="flex flex-col gap-5">
										<WizardField label="Username" required>
											<Input
												placeholder="AWS / Docker Hub username"
												value={username}
												onChange={e => setUsername(e.target.value)}
												autoFocus
											/>
										</WizardField>
										<WizardField
											label="Password / Access Token"
											required={!initialData}
											optional={!!initialData}
											hint={
												initialData
													? 'Leave blank to keep the stored token.'
													: undefined
											}>
											<Input
												type="password"
												placeholder={
													initialData
														? '••••••••'
														: 'Access Token / Password'
												}
												value={password}
												onChange={e => setPassword(e.target.value)}
												className="font-mono"
											/>
										</WizardField>
										<button
											type="button"
											onClick={handleTestRaw}
											disabled={isTesting}
											className="flex items-center justify-center gap-2 rounded-lg border border-border/60 px-4 py-2.5 text-[13px] font-medium text-muted-foreground transition-colors hover:bg-muted/30 hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
											{isTesting ? (
												<RefreshCw className="size-3.5 animate-spin text-amber-500" />
											) : (
												<Plug className="size-3.5" />
											)}
											{isTesting ? 'Testing…' : 'Test Connection'}
										</button>
									</div>
								</StepBody>
							)}

							{step === 'server' && (
								<StepBody key="server">
									<div className="flex flex-col gap-2">
										<OptionRow
											selected={serverId === 'local'}
											onClick={() => setServerId('local')}
											icon={<ServerIcon className="size-5" />}
											title="Local Server"
											subtitle="Authenticate on this server (default)."
										/>
										{servers.map((s: RemoteServerResponse) => (
											<OptionRow
												key={String(s.id)}
												selected={serverId === String(s.id)}
												onClick={() => setServerId(String(s.id))}
												icon={<ServerIcon className="size-5" />}
												title={s.name}
												subtitle={s.ip_address}
											/>
										))}
									</div>
								</StepBody>
							)}
						</StepTimeline>
					)}
				</form>

				{editing ? (
					<SheetFooter className="justify-between">
						<p className="self-center text-xs text-muted-foreground">
							Images pull from{' '}
							<span className="font-semibold text-foreground">
								{registryUrl.trim() || 'your registry'}
							</span>
						</p>
						<Button
							type="submit"
							form={FORM_ID}
							disabled={!formReady}
							className="h-10 gap-1.5 px-5">
							<Save className="size-3.5" />
							Save Changes
						</Button>
					</SheetFooter>
				) : last ? (
					<SheetFooter className="justify-between">
						<p className="self-center text-xs text-muted-foreground">
							Images pull from{' '}
							<span className="font-semibold text-foreground">
								{registryUrl.trim() || 'your registry'}
							</span>
						</p>
						<div className="flex items-center gap-3">
							{stepIdx > 0 && (
								<Button
									variant="outline"
									onClick={() => setStepIdx(i => i - 1)}>
									Back
								</Button>
							)}
							<Button
								type="submit"
								form={FORM_ID}
								disabled={!canContinue}
								className="h-10 gap-1.5 px-5">
								<Save className="size-3.5" />
								{initialData ? 'Save Changes' : 'Add Registry'}
							</Button>
						</div>
					</SheetFooter>
				) : (
					<SheetFooter>
						{stepIdx > 0 && (
							<Button
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						)}
						<Button
							disabled={!canContinue}
							onClick={() => setStepIdx(i => i + 1)}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
