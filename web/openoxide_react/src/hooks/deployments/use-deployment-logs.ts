import * as React from 'react';
import {toast} from 'sonner';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/utils/log-lines';
import type {Deployment} from './use-deployments';

const deploymentClient = createServiceClient(DeploymentService);

/** Streams a deployment's build log over the panel's ConnectRPC stream, appending lines as the deployment progresses. The stored `log_content` (if any) renders immediately while the stream catches up. */
export function useDeploymentLogs(selectedDeployment: Deployment | null) {
	const [logs, setLogs] = React.useState<string>('');
	const [isLogsLoading, setIsLogsLoading] = React.useState(false);
	const [copied, setCopied] = React.useState(false);

	const handleCopyLogs = () => {
		navigator.clipboard.writeText(logs);
		setCopied(true);
		toast.success('Logs copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	React.useEffect(() => {
		if (!selectedDeployment || selectedDeployment.id === undefined) return;

		let isMounted = true;
		const controller = new AbortController();
		setLogs('');
		setIsLogsLoading(true);

		const depWithLogs = selectedDeployment as Deployment & {
			log_content?: string;
		};

		if (depWithLogs.log_content) {
			setLogs(depWithLogs.log_content);
		}

		const streamLogs = async () => {
			try {
				const call = deploymentClient.streamDeploymentLogs(
					{id: BigInt(selectedDeployment.id as number)},
					{signal: controller.signal},
				);
				for await (const event of call) {
					if (!isMounted) return;
					if (event.error) {
						setIsLogsLoading(false);
						return;
					}
					const lines = extractLogLines(event.payload);
					if (lines.length > 0) {
						setIsLogsLoading(false);
						setLogs(prev =>
							prev ? `${prev}\n${lines.join('\n')}` : lines.join('\n'),
						);
					}
				}
				if (isMounted) setIsLogsLoading(false);
			} catch {
				// Stream failed (network drop, abort); fall back to the stored snapshot.
				if (isMounted) {
					if (depWithLogs.log_content) {
						setLogs(depWithLogs.log_content);
					}
					setIsLogsLoading(false);
				}
			}
		};

		void streamLogs();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [selectedDeployment]);

	return {
		logs,
		isLogsLoading,
		copied,
		handleCopyLogs,
	};
}
