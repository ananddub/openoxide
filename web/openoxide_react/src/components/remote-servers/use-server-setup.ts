import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerTestConnection,
	useRemoteServerAudit,
} from '#/api/live';
import {createServiceClient} from '#/api/live/query';
import {ServerControllerService} from '#/gen/connect/openoxide/api/v1/server_controller_pb';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import type {RemoteServerResponse} from '#/types/api-helpers';

const serverControllerClient = createServiceClient(
	ServerControllerService,
);

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
		setSetupLogs([]);
		try {
			const call = serverControllerClient.streamSetupLogs({
				id: BigInt(server.id),
				filter: JSON.stringify({
					host_key_fingerprint: '',
					install_dependencies: true,
					...(advertiseAddr.trim()
						? {advertise_addr: advertiseAddr.trim()}
						: {}),
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

			toast.success(`Server ${server.name} setup completed successfully!`);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
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
