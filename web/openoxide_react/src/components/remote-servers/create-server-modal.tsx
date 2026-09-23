import {useState, useEffect} from 'react';
import {FastForward, Network, Server, SkipForward} from 'lucide-react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {ServerFormFields} from '#/components/remote-servers/server-form-fields';
import {
	OptionRow,
	StepBody,
	StepCheckpoints,
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
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import {privateNetworkApi} from '#/components/remote-servers/private-network/api';
import {remoteHostFromTunnel} from '#/components/remote-servers/private-network/panel-parts';
import type {ConnectionMode} from '#/components/remote-servers/private-network/types';
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

const FORM_ID = 'create-remote-server-form';

const STEPS: WizardStep[] = [
	{id: 'credentials', label: 'Credentials'},
	{id: 'setup', label: 'Setup'},
	{id: 'network', label: 'Network'},
];

const STEP_DESCRIPTIONS = [
	'Host address and SSH credentials for the remote deployment node.',
	'Run the one-time server setup (install Docker and dependencies) right after the server is saved.',
	'Choose how OpenOxide reaches this server — plain SSH or a managed WireGuard tunnel.',
];

type Phase = 'idle' | 'creating' | 'network' | 'setup';

const PHASE_LABEL: Record<Phase, string> = {
	idle: 'Save',
	creating: 'Saving…',
	network: 'Configuring network…',
	setup: 'Running setup…',
};

/** Create response carries the new row as id (or JSON payload) — cover both. */
function createdServerId(response: unknown): number {
	const rec = (response ?? {}) as {id?: number | bigint; payload?: string};
	if (rec.id != null && Number(rec.id) > 0) return Number(rec.id);
	try {
		return Number(JSON.parse(rec.payload ?? '{}').id) || 0;
	} catch {
		return 0;
	}
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
	const [runSetup, setRunSetup] = useState(true);
	const [networkMode, setNetworkMode] = useState<ConnectionMode>('DIRECT_SSH');
	const [phase, setPhase] = useState<Phase>('idle');
	const [setupLogs, setSetupLogs] = useState<string[]>([]);

	const createMutation = useRemoteServerCreate();
	const patchMutation = useRemoteServerUpdate();
	const testDirectMutation = useRemoteServerTestConnection();

	useEffect(() => {
		setTestResult('idle');
	}, [editingServer, isOpen]);

	useEffect(() => {
		if (!isOpen) return;
		setStepIdx(0);
		setRunSetup(true);
		setNetworkMode('DIRECT_SSH');
		setPhase('idle');
		setSetupLogs([]);
	}, [isOpen]);

	const last = stepIdx === STEPS.length - 1;
	const credentialsReady = Boolean(name && ipAddress && username);
	const canContinue = stepIdx === 0 ? credentialsReady : true;

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
				// Create wizard: TestDirectConnection verifies raw
				// credentials server-side without persisting anything.
				await serverControllerClient.testDirectConnection({
					payload: JSON.stringify({
						ip_address: ipAddress,
						port: parseInt(port) || 22,
						username,
						ssh_key_id: parseInt(sshKeyId),
					}),
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

	const streamSetup = async (serverId: number) => {
		setPhase('setup');
		const call = serverControllerClient.streamSetupLogs({
			id: BigInt(serverId),
			filter: JSON.stringify({
				host_key_fingerprint: '',
				install_dependencies: true,
			}),
		});
		let failedMessage = '';
		for await (const event of call) {
			const lines = extractLogLines(event.payload);
			if (lines.length > 0) {
				failedMessage ||=
					lines.find(line => line.includes('Setup Server failed')) || '';
				setSetupLogs(prev => [...prev, ...lines]);
			}
		}
		if (failedMessage) throw new Error(failedMessage);
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
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

			// Create → optional network → optional setup. Each post-create
			// phase runs against the new server id; failures still surface
			// the created server in the list.
			setPhase('creating');
			const created = await createMutation.mutateAsync(payload);
			const serverId = createdServerId(created);

			if (networkMode === 'MANAGED_WIREGUARD' && serverId) {
				setPhase('network');
				const tunnelAddress = `10.77.${(serverId % 250) + 1}.0/24`;
				await privateNetworkApi.update(serverId, {
					connection_mode: 'MANAGED_WIREGUARD',
					provider: 'WIREGUARD',
					private_host: remoteHostFromTunnel(tunnelAddress),
					tunnel_address: tunnelAddress,
					public_key: null,
					endpoint: `${ipAddress}:51820`,
					listen_port: 51820,
					persistent_keepalive: 25,
					dns_name: null,
					routes: [],
				});
				await privateNetworkApi.setup(serverId);
			}

			if (runSetup && serverId) {
				await streamSetup(serverId);
			} else {
				setPhase('idle');
			}

			toast.success('Remote Server added successfully');
			onSuccess();
			onClose();
		} catch (err: unknown) {
			setPhase('idle');
			toast.error(formatApiError(err));
			// The server may already exist even though a later phase failed —
			// refresh the list so it shows up.
			if (phase !== 'creating') onSuccess();
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{editingServer ? 'Edit Remote Server' : 'New Remote Server'}
					</SheetTitle>
					<SheetDescription>
						{editingServer
							? 'Configure host IP address, SSH port, and credentials for remote deployment node'
							: STEP_DESCRIPTIONS[stepIdx]}
					</SheetDescription>
					{!editingServer && (
						<StepCheckpoints steps={STEPS} idx={stepIdx} />
					)}
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
						{editingServer ? (
							<ServerFormFields {...form} sshKeys={sshKeys} />
						) : stepIdx === 0 ? (
							<StepBody key="credentials">
								<div className="flex flex-col gap-4">
									<ServerFormFields {...form} sshKeys={sshKeys} />
								</div>
							</StepBody>
						) : stepIdx === 1 ? (
							<StepBody key="setup">
								<div className="flex flex-col gap-2">
									<OptionRow
										selected={runSetup}
										onClick={() => setRunSetup(true)}
										icon={<FastForward className="size-4" />}
										title="Run setup after save"
										subtitle="Installs Docker and required dependencies on the server."
									/>
									<OptionRow
										selected={!runSetup}
										onClick={() => setRunSetup(false)}
										icon={<SkipForward className="size-4" />}
										title="Skip for now"
										subtitle="Save credentials only — run setup later from the server row menu."
									/>
								</div>
							</StepBody>
						) : (
							<StepBody key="network">
								<div className="flex flex-col gap-2">
									<OptionRow
										selected={networkMode === 'DIRECT_SSH'}
										onClick={() => setNetworkMode('DIRECT_SSH')}
										icon={<Server className="size-4" />}
										title="Direct SSH"
										subtitle="Reach the server over its public IP via SSH (default)."
									/>
									<OptionRow
										selected={networkMode === 'MANAGED_WIREGUARD'}
										onClick={() =>
											setNetworkMode('MANAGED_WIREGUARD')
										}
										icon={<Network className="size-4" />}
										title="Managed WireGuard"
										subtitle="OpenOxide creates an encrypted private tunnel to the server after save."
									/>
								</div>
							</StepBody>
						)}

						{setupLogs.length > 0 && (
							<div className="mt-2 overflow-hidden rounded-lg border bg-background">
								<div className="border-b px-3 py-2 text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
									Setup log
								</div>
								<pre className="max-h-56 overflow-y-auto p-3 font-mono text-[11px] leading-relaxed whitespace-pre-wrap text-muted-foreground">
									{setupLogs.join('\n')}
								</pre>
							</div>
						)}
					</div>

					{editingServer ? (
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
								disabled={submitting || testingConn}
								className="h-9 px-6 text-xs font-semibold shadow-md">
								{submitting ? 'Saving...' : 'Save Changes'}
							</Button>
						</SheetFooter>
					) : (
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
									disabled={
										testingConn || submitting || phase !== 'idle'
									}
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
									disabled={submitting}>
									{PHASE_LABEL[phase]}
								</Button>
							)}
						</SheetFooter>
					)}
				</form>
			</SheetContent>
		</Sheet>
	);
}
