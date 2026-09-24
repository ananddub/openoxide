import {useEffect, useState} from 'react';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/utils/log-lines';

const deploymentClient = createServiceClient(DeploymentService);

/** Streams live build logs for a deployment over ConnectRPC while `deploymentId` is set. An optional `fallback` text seeds the buffer (and covers a failed stream) for historical deployments. */
export function useDeploymentLogStream(
	deploymentId: number | null,
	fallback?: string,
): string[] {
	const [liveLogs, setLiveLogs] = useState<string[]>([]);

	useEffect(() => {
		if (!deploymentId) return;
		let isMounted = true;
		const controller = new AbortController();
		setLiveLogs(fallback ? fallback.split('\n') : []);

		const startStream = async () => {
			try {
				const call = deploymentClient.streamDeploymentLogs(
					{id: BigInt(deploymentId)},
					{signal: controller.signal},
				);
				for await (const event of call) {
					if (!isMounted) return;
					const lines = extractLogLines(event.payload);
					if (lines.length > 0) setLiveLogs(prev => [...prev, ...lines]);
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setLiveLogs(prev => [
						...prev,
						`Stream error: ${err.message || 'Connection lost'}`,
					]);
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [deploymentId, fallback]);

	return liveLogs;
}
