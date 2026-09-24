import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerTestConnection,
	useRemoteServerAudit,
} from '#/api/live';
import {serverControllerClient} from '#/api/live/servers/shared';
import {ServerSetupStep} from '#/gen/connect/openoxide/api/v1/server_controller_pb';
import type {RemoteServerResponse} from '#/types/api-helpers';

/** Test / audit / streamed-setup actions behind the Setup & Audit dialog. */
export function useServerSetup(server: RemoteServerResponse | null) {
	const [testingConn, setTestingConn] = useState(false);
	const [auditing, setAuditing] = useState(false);
	const [settingUp, setSettingUp] = useState(false);
	const [setupLogs, setSetupLogs] = useState<string[]>([]);
	const [auditResult, setAuditResult] = useState<Record<
		string,
		unknown
	> | null>(null);

	const testConnMutation = useRemoteServerTestConnection();
	const auditMutation = useRemoteServerAudit();

	const testConnection = async () => {
		if (!server?.id) return;
		setTestingConn(true);
		try {
			await testConnMutation.mutateAsync(server.id);
			toast.success(
				`SSH Connection to ${server.name} verified successfully!`,
			);
		} catch (err: unknown) {
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

	const setup = async (advertiseAddr: string) => {
		if (!server?.id) return;
		setSettingUp(true);
		setSetupLogs([
			`Running setup on ${server.name}… this can take a few minutes.`,
		]);
		try {
			const call = serverControllerClient.streamSetupLogs({
				id: BigInt(server.id),
				installDependencies: true,
				hostKeyFingerprint: '',
				advertiseAddr: advertiseAddr.trim() || undefined,
			});

			for await (const event of call) {
				const lines = (event.outcome?.completed ?? []).map(
					step => `Setup step completed: ${ServerSetupStep[step]}`,
				);
				if (lines.length > 0) {
					setSetupLogs(prev => [...prev, ...lines]);
				}
			}

			setSetupLogs(prev => [...prev, 'Setup completed successfully.']);
			toast.success(`Server ${server.name} setup completed successfully!`);
		} catch (err: unknown) {
			const message = formatApiError(err);
			setSetupLogs(prev => [...prev, `Setup failed: ${message}`]);
			toast.error(`Setup failed: ${message}`);
		} finally {
			setSettingUp(false);
		}
	};

	return {
		testingConn,
		testConnection,
		auditing,
		audit,
		settingUp,
		setupLogs,
		setup,
		auditResult,
	};
}
