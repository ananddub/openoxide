import {useState, useEffect} from 'react';
import {
	CheckCircle2,
	Loader,
	Network,
	ShieldCheck,
	Terminal,
	XCircle,
} from 'lucide-react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from '#/components/ui/tabs';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {ServerFormFields} from '#/components/remote-servers/server-form-fields';
import {
	OptionRow,
	StepTimeline,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerCreate,
	useRemoteServerUpdate,
	useRemoteServerTestConnection,
} from '#/api/live';
import {serverControllerClient} from '#/api/live/servers/shared';
import {ServerSetupStep} from '#/gen/connect/openoxide/api/v1/server_controller_pb';
import {privateNetworkApi} from '#/components/remote-servers/private-network/api';
import {
	EXTERNAL_PROVIDERS,
	remoteHostFromTunnel,
} from '#/components/remote-servers/private-network/panel-parts';
import {PrivateNetworkCard} from '#/components/remote-servers/private-network/network-card';
import type {
	ConnectionMode,
	PrivateNetworkProvider,
} from '#/components/remote-servers/private-network/types';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';
import {useServerForm} from './use-server-form';
import {SetupLogsSurface, stampLogs} from './setup-logs-surface';
import {Input} from '#/components/ui/input';
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

const FORM_ID = 'create-remote-server-form';

/** Temporary build stamp — proves which bundle the browser actually loaded. */
const WIZARD_BUILD = 'netq-3';

const STEPS: WizardStep[] = [
	{id: 'credentials', label: 'Credentials'},
	{id: 'network', label: 'Network'},
];

/** Full journey, always visible — provisioning sits mid-flow and Ready ends it. */
const TIMELINE: WizardStep[] = [
	...STEPS,
	{id: 'provision', label: 'Provision'},
	{id: 'ready', label: 'Ready'},
];

const STEP_DESCRIPTIONS = [
	'Host address and SSH credentials for the remote deployment node.',
	'How should OpenOxide reach this server? Pick a connection method.',
];

type Phase = 'idle' | 'creating' | 'network' | 'setup' | 'done';

const PHASE_LABEL: Record<Phase, string> = {
	idle: 'Save',
	creating: 'Saving…',
	network: 'Configuring network…',
	setup: 'Provisioning…',
	done: 'Done',
};

/** Create response carries the new typed row; 0 means "not created". */
function createdServerId(response: {
	server?: {id: bigint} | undefined;
}): number {
	return Number(response.server?.id ?? 0n);
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

	const [stepIdx, setStepIdx] = useState(0);
	// Edit mode splits into tabs — the footer below only belongs to General;
	// the Private Network tab owns its own save inside the card.
	const [editTab, setEditTab] = useState<'general' | 'network'>('general');
	const [sudoPassword, setSudoPassword] = useState('');
	// No default — the Network step is a real question that must be answered.
	const [networkMode, setNetworkMode] = useState<ConnectionMode | null>(
		null,
	);
	const [wgPort, setWgPort] = useState('51820');
	const [wgKeepalive, setWgKeepalive] = useState('25');
	const [vpnProvider, setVpnProvider] =
		useState<PrivateNetworkProvider>('TAILSCALE');
	const [vpnHost, setVpnHost] = useState('');
	const [phase, setPhase] = useState<Phase>('idle');
	const [setupLogs, setSetupLogs] = useState<string[]>([]);
	const [setupState, setSetupState] = useState<
		'idle' | 'running' | 'success' | 'failed'
	>('idle');

	const createMutation = useRemoteServerCreate();
	const patchMutation = useRemoteServerUpdate();
	const testDirectMutation = useRemoteServerTestConnection();

	useEffect(() => {
		setTestResult('idle');
	}, [editingServer, isOpen]);

	useEffect(() => {
		if (!isOpen) return;
		setStepIdx(0);
		setEditTab('general');
		setNetworkMode(null);
		setWgPort('51820');
		setWgKeepalive('25');
		setVpnProvider('TAILSCALE');
		setVpnHost('');
		setPhase('idle');
		setSetupLogs([]);
		setSetupState('idle');
	}, [isOpen]);

	const last = stepIdx === STEPS.length - 1;
	const credentialsReady = Boolean(name && ipAddress && username);
	// A VPN mesh is useless without the address to reach it at.
	const networkReady =
		!!networkMode &&
		(networkMode !== 'EXTERNAL_PRIVATE_NETWORK' || !!vpnHost.trim());
	// A failed SSH test halts the wizard — no point saving/setting up a host
	// we just proved we can't reach.
	const canContinue =
		testResult !== 'failed' &&
		(stepIdx === 0 ? credentialsReady : networkReady);
	// Saving walks the checkpoints forward: Provision (live logs) in the middle, Ready at the end.
	const readyView = phase === 'done' && setupState !== 'failed';
	const provisionView = phase !== 'idle' && !readyView;
	const activeIdx =
		phase === 'idle' ? stepIdx : readyView ? TIMELINE.length - 1 : 2;
	const timelineNodeState =
		phase === 'idle'
			? undefined
			: readyView
				? ('success' as const)
				: setupState === 'failed'
					? ('failed' as const)
					: ('running' as const);

	const buildPayload = () => ({
		name,
		ip_address: ipAddress,
		port: parseInt(port) || 22,
		username,
		ssh_key_id: sshKeyId ? parseInt(sshKeyId) : undefined,
		server_type: 'DEPLOY',
		description: description || undefined,
	});

	const handleTestConnection = async () => {
		if (!ipAddress || !username) {
			toast.error(
				'IP Address and Username are required to test connection',
			);
			return;
		}
		if (!editingServer?.id && !sshKeyId) {
			toast.error('Select an SSH Key to test the connection');
			return;
		}
		setTestingConn(true);
		setTestResult('idle');
		try {
			if (editingServer?.id) {
				// Saved server: test through its stored connection config.
				await testDirectMutation.mutateAsync(editingServer.id);
			} else {
				// Create wizard: TestDirectConnection verifies raw credentials server-side without persisting anything.
				await serverControllerClient.testDirectConnection({
					ipAddress,
					port: parseInt(port) || 22,
					username,
					sshKeyId: BigInt(sshKeyId),
				});
			}
			setTestResult('success');
			toast.success(`SSH Connection to "${name || ipAddress}" verified!`);
		} catch (err: unknown) {
			setTestResult('failed');
			toast.error(formatApiError(err));
		} finally {
			setTestingConn(false);
		}
	};

	const streamSetup = async (serverId: number): Promise<string | null> => {
		setPhase('setup');
		setSetupState('running');
		setSetupLogs(
			stampLogs([
				'Connecting to server and running setup… this can take a few minutes.',
			]),
		);
		try {
			const call = serverControllerClient.streamSetupLogs({
				id: BigInt(serverId),
				installDependencies: true,
				hostKeyFingerprint: '',
				sudoPassword: sudoPassword.trim() || undefined,
			});
			for await (const event of call) {
				const lines = [...(event.lines ?? [])];
				const stepLines = (event.outcome?.completed ?? []).map(
					step => `Setup step completed: ${ServerSetupStep[step]}`,
				);
				const batch = [...lines, ...stepLines];
				if (batch.length > 0) {
					setSetupLogs(prev => [...prev, ...stampLogs(batch)]);
				}
			}
			setSetupLogs(prev => [
				...prev,
				...stampLogs(['Setup completed successfully.']),
			]);
			setSetupState('success');
			return null;
		} catch (err: unknown) {
			const message = formatApiError(err);
			setSetupLogs(prev => [
				...prev,
				...stampLogs([`Setup failed: ${message}`]),
			]);
			setSetupState('failed');
			return message;
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (phase === 'done') {
			// Setup finished — the wizard only closes when the user clicks Continue.
			setPhase('idle');
			onClose();
			return;
		}
		if (!editingServer && phase === 'idle' && stepIdx < STEPS.length - 1) {
			// Enter in a field advances the wizard — it must never jump straight to provisioning.
			if (canContinue) setStepIdx(i => i + 1);
			return;
		}
		if (!credentialsReady) {
			toast.error('Server Name, IP Address, and Username are required');
			return;
		}
		setSubmitting(true);
		try {
			const payload = buildPayload();

			if (editingServer?.id) {
				await patchMutation.mutateAsync({
					id: editingServer.id,
					...payload,
				});
				toast.success('Remote Server updated successfully');
				onSuccess();
				onClose();
				return;
			}

			// Create → optional network → always provision. Each post-create phase runs against the new server id; failures still surface the created server in the list.
			if (!networkReady) {
				toast.error(
					networkMode === 'EXTERNAL_PRIVATE_NETWORK'
						? 'Enter the private address of your VPN mesh'
						: 'Pick how OpenOxide should reach this server',
				);
				return;
			}
			setPhase('creating');
			const created = await createMutation.mutateAsync(payload);
			const serverId = createdServerId(created);

			if (networkMode === 'MANAGED_WIREGUARD' && serverId) {
				setPhase('network');
				const listenPort = parseInt(wgPort) || 51820;
				const tunnelAddress = `10.77.${(serverId % 250) + 1}.0/24`;
				await privateNetworkApi.update(serverId, {
					connection_mode: 'MANAGED_WIREGUARD',
					provider: 'WIREGUARD',
					private_host: remoteHostFromTunnel(tunnelAddress),
					tunnel_address: tunnelAddress,
					public_key: null,
					endpoint: `${ipAddress}:${listenPort}`,
					listen_port: listenPort,
					persistent_keepalive: parseInt(wgKeepalive) || 25,
					dns_name: null,
					routes: [],
				});
				await privateNetworkApi.setup(serverId);
			}

			if (networkMode === 'EXTERNAL_PRIVATE_NETWORK' && serverId) {
				// Mesh already exists — just point OpenOxide at the private address.
				setPhase('network');
				await privateNetworkApi.update(serverId, {
					connection_mode: 'EXTERNAL_PRIVATE_NETWORK',
					provider: vpnProvider,
					private_host: vpnHost.trim(),
					tunnel_address: null,
					public_key: null,
					endpoint: null,
					listen_port: null,
					persistent_keepalive: null,
					dns_name: null,
					routes: [],
				});
			}

			if (serverId) {
				// Provisioning finished (ok or failed) — stay open showing the log;
				// the wizard only closes when the user clicks Continue.
				const setupError = await streamSetup(serverId);
				setPhase('done');
				onSuccess();
				toast[setupError ? 'error' : 'success'](
					setupError
						? 'Server saved, but setup failed — see the setup log, or run setup again from the server row menu.'
						: 'Remote Server added and setup completed',
				);
				return;
			}

			setPhase('idle');
			toast.success('Remote Server added successfully');
			onSuccess();
			onClose();
		} catch (err: unknown) {
			setPhase('idle');
			toast.error(formatApiError(err));
			// The server may already exist even though a later phase failed — refresh the list so it shows up.
			if (phase !== 'creating') onSuccess();
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent className="sm:data-[side=right]:w-[680px]">
				<SheetHeader>
					<SheetTitle>
						{editingServer ? 'Edit Remote Server' : 'New Remote Server'}
						<span className="ml-2 align-middle font-mono text-[10px] text-muted-foreground/60">
							{WIZARD_BUILD}
						</span>
					</SheetTitle>
					<SheetDescription>
						{editingServer
							? 'Configure host IP address, SSH port, and credentials for remote deployment node'
							: phase !== 'idle'
								? 'Provisioning runs right inside the timeline — watch it work.'
								: STEP_DESCRIPTIONS[stepIdx]}
					</SheetDescription>
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
						{editingServer ? (
							<Tabs
								value={editTab}
								onValueChange={v => setEditTab(v as 'general' | 'network')}
								className="min-h-0 flex-1 gap-4">
								<TabsList className="w-full">
									<TabsTrigger value="general">General</TabsTrigger>
									<TabsTrigger value="network">
										Private Network
									</TabsTrigger>
								</TabsList>
								<TabsContent
									value="general"
									className="flex flex-col gap-4">
									<ServerFormFields {...form} sshKeys={sshKeys} />
								</TabsContent>
								{/* Only mounted once opened, so the dialog does not
								    fetch a network config nobody asked for. */}
								<TabsContent
									value="network"
									className="flex min-h-0 flex-1 flex-col">
									<PrivateNetworkCard server={editingServer} />
								</TabsContent>
							</Tabs>
						) : (
							<StepTimeline
								steps={TIMELINE}
								idx={activeIdx}
								nodeState={timelineNodeState}>
								<div
									key={
										readyView
											? 'ready'
											: provisionView
												? 'provision'
												: stepIdx
									}>
									{provisionView || readyView ? (
										<div className="flex flex-col gap-3">
											<p className="flex items-center gap-2 text-xs text-muted-foreground">
												{readyView ? (
													<>
														<CheckCircle2 className="size-3.5 text-emerald-500" />
														Setup completed — the server is ready.
													</>
												) : phase === 'creating' ? (
													<>
														<Loader className="size-3.5 animate-spin text-primary" />
														Saving the server…
													</>
												) : phase === 'network' ? (
													<>
														<Loader className="size-3.5 animate-spin text-primary" />
														Configuring the tunnel…
													</>
												) : setupState === 'failed' ? (
													<>
														<XCircle className="size-3.5 text-red-500" />
														Setup failed — re-run from the server row menu.
													</>
												) : (
													<>
														<Loader className="size-3.5 animate-spin text-primary" />
														Running setup on{' '}
														<span className="font-medium text-foreground">
															{name || ipAddress}
														</span>
														… this can take a few minutes.
													</>
												)}
											</p>
											{setupLogs.length > 0 && (
												<SetupLogsSurface
													logs={setupLogs}
													isLive={phase === 'setup'}
													state={setupState}
													className="h-[48vh] w-full rounded-lg border border-border"
												/>
											)}
										</div>
									) : stepIdx === 0 ? (
										<div className="flex flex-col gap-4">
											<ServerFormFields {...form} sshKeys={sshKeys} />
											<WizardField
												label="Sudo Password"
												htmlFor="server-sudo"
												optional
												hint="Used during setup. Required unless the user is root or has passwordless sudo.">
												<Input
													id="server-sudo"
													type="password"
													value={sudoPassword}
													onChange={e => setSudoPassword(e.target.value)}
													disabled={submitting}
													placeholder="Only needed when your SSH user runs commands via sudo"
												/>
											</WizardField>
											{testResult === 'failed' && (
												<p className="text-xs text-amber-600 dark:text-amber-500">
													SSH test failed — fix the details above and
													re-test before continuing.
												</p>
											)}
										</div>
									) : (
										<div className="flex flex-col gap-2">
											<OptionRow
												selected={networkMode === 'DIRECT_SSH'}
												onClick={() => setNetworkMode('DIRECT_SSH')}
												icon={<Terminal className="size-4.5" />}
												title="Direct SSH"
												subtitle="Reach the server over its public IP."
											/>
											<OptionRow
												selected={networkMode === 'MANAGED_WIREGUARD'}
												onClick={() => setNetworkMode('MANAGED_WIREGUARD')}
												icon={<ShieldCheck className="size-4.5" />}
												title="Managed WireGuard tunnel"
												subtitle="OpenOxide builds an encrypted private tunnel after save."
											/>
											<OptionRow
												selected={
													networkMode === 'EXTERNAL_PRIVATE_NETWORK'
												}
												onClick={() =>
													setNetworkMode('EXTERNAL_PRIVATE_NETWORK')
												}
												icon={<Network className="size-4.5" />}
												title="Existing VPN mesh"
												subtitle="Route via Tailscale, NetBird, ZeroTier — you run the mesh, we use it."
											/>
											{networkMode === 'MANAGED_WIREGUARD' && (
												<div className="mt-1 grid animate-in grid-cols-2 gap-3 duration-200 ease-out fade-in-0 slide-in-from-right-4">
													<WizardField label="Tunnel Port">
														<Input
															value={wgPort}
															onChange={e => setWgPort(e.target.value)}
															disabled={submitting}
															placeholder="51820"
															className="font-mono"
														/>
													</WizardField>
													<WizardField label="Keepalive (sec)">
														<Input
															value={wgKeepalive}
															onChange={e =>
																setWgKeepalive(e.target.value)
															}
															disabled={submitting}
															placeholder="25"
															className="font-mono"
														/>
													</WizardField>
													<p className="col-span-2 text-[13px] leading-relaxed text-muted-foreground">
														Full network management (routes, DNS, re-keys)
														stays in server row → Network.
													</p>
												</div>
											)}
											{networkMode === 'EXTERNAL_PRIVATE_NETWORK' && (
												<div className="mt-1 grid animate-in grid-cols-2 gap-3 duration-200 ease-out fade-in-0 slide-in-from-right-4">
													<WizardField label="Provider">
														<Select
															value={vpnProvider}
															onValueChange={val =>
																val &&
																setVpnProvider(
																	val as PrivateNetworkProvider,
																)
															}>
															<SelectTrigger>
																<SelectValue>
																	{
																		EXTERNAL_PROVIDERS.find(
																			p => p.value === vpnProvider,
																		)?.label
																	}
																</SelectValue>
															</SelectTrigger>
															<SelectContent>
																{EXTERNAL_PROVIDERS.map(p => (
																	<SelectItem
																		key={p.value}
																		value={p.value}
																		className="text-xs">
																		{p.label}
																	</SelectItem>
																))}
															</SelectContent>
														</Select>
													</WizardField>
													<WizardField label="Private Address">
														<Input
															value={vpnHost}
															onChange={e => setVpnHost(e.target.value)}
															disabled={submitting}
															placeholder="100.64.0.5 or mesh hostname"
															className="font-mono"
														/>
													</WizardField>
													<p className="col-span-2 text-[13px] leading-relaxed text-muted-foreground">
														Install and connect the mesh on the node first
														— OpenOxide will SSH via this after save.
													</p>
												</div>
											)}
										</div>
									)}
								</div>
							</StepTimeline>
						)}
					</div>

					{editingServer && editTab === 'general' ? (
						<SheetFooter className="justify-between">
							<TestConnectionButton
								testing={testingConn}
								result={testResult}
								disabled={testingConn || submitting}
								onClick={() => void handleTestConnection()}
							/>
							<Button
								type="submit"
								form={FORM_ID}
								disabled={submitting || testingConn}>
								{submitting ? 'Saving...' : 'Save Changes'}
							</Button>
						</SheetFooter>
					) : editingServer ? null : (
						<SheetFooter>
							<div className="mr-auto flex items-center gap-3">
								{stepIdx > 0 && phase === 'idle' && (
									<Button
										type="button"
										variant="outline"
										onClick={() => setStepIdx(i => i - 1)}>
										Back
									</Button>
								)}
								<TestConnectionButton
									testing={testingConn}
									result={testResult}
									disabled={testingConn || submitting || phase !== 'idle'}
									onClick={() => void handleTestConnection()}
								/>
							</div>
							{!last ? (
								<Button
									type="button"
									disabled={!canContinue}
									onClick={() => setStepIdx(i => i + 1)}>
									Continue
								</Button>
							) : (
								<Button
									type="submit"
									form={FORM_ID}
									disabled={
										submitting ||
										(phase === 'idle' &&
											(testResult === 'failed' || !networkReady))
									}>
									{phase === 'done' && setupState === 'failed'
										? 'Close'
										: PHASE_LABEL[phase]}
								</Button>
							)}
						</SheetFooter>
					)}
				</form>
			</SheetContent>
		</Sheet>
	);
}
