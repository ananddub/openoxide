import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerTestConnection,
	useRemoteServerAudit,
} from '#/api/live';
import {serverControllerClient} from '#/api/live/servers/shared';
import {ServerSetupStep} from '#/gen/connect/openoxide/api/v1/server_controller_pb';
import {stampLogs} from './setup-logs-surface';
import type {RemoteServerResponse} from '#/types/api-helpers';

/** Test / audit / streamed-setup actions behind the Setup & Audit panel. */
export function useServerSetup(server: RemoteServerResponse | null) {
	const [testingConn, setTestingConn] = useState(false);
	const [auditing, setAuditing] = useState(false);
	const [setupState, setSetupState] = useState<
		'idle' | 'running' | 'success' | 'failed'
	>('idle');
	// Last explicit SSH-test verdict — gates Run Setup so a failed host
	// isn't pushed into a minutes-long setup that can only stream errors.
	const [connOk, setConnOk] = useState<boolean | null>(null);
	const [setupLogs, setSetupLogs] = useState<string[]>([]);
	const [auditResult, setAuditResult] = useState<Record<
		string,
		unknown
	> | null>(null);

	const settingUp = setupState === 'running';

	const testConnMutation = useRemoteServerTestConnection();
	const auditMutation = useRemoteServerAudit();

	const testConnection = async () => {
		if (!server?.id) return;
		setTestingConn(true);
		try {
			await testConnMutation.mutateAsync(server.id);
			setConnOk(true);
			toast.success(
				`SSH Connection to ${server.name} verified successfully!`,
			);
		} catch (err: unknown) {
			setConnOk(false);
			toast.error(formatApiError(err));
		} finally {
			setTestingConn(false);
		}
	};

	const audit = async () => {
		if (!server?.id) return;
		setAuditing(true);
		try {
			const res = await auditMutation.mutateAsync(server.id);
			setAuditResult(res as Record<string, unknown>);
			toast.success('Server audit completed!');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setAuditing(false);
		}
	};

	const setup = async (advertiseAddr: string, sudoPassword?: string) => {
		if (!server?.id) return;
		setSetupState('running');
		setSetupLogs(
			stampLogs([
				`Running setup on ${server.name}… this can take a few minutes.`,
			]),
		);
		try {
			const call = serverControllerClient.streamSetupLogs({
				id: BigInt(server.id),
				installDependencies: true,
				hostKeyFingerprint: '',
				advertiseAddr: advertiseAddr.trim() || undefined,
				sudoPassword: sudoPassword?.trim() || undefined,
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
			toast.success(`Server ${server.name} setup completed successfully!`);
		} catch (err: unknown) {
			const message = formatApiError(err);
			setSetupLogs(prev => [
				...prev,
				...stampLogs([`Setup failed: ${message}`]),
			]);
			setSetupState('failed');
			toast.error(`Setup failed: ${message}`);
		}
	};

	return {
		testingConn,
		testConnection,
		auditing,
		audit,
		settingUp,
		setupState,
		connOk,
		setupLogs,
		setup,
		auditResult,
	};
}
