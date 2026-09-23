import {useState, useEffect} from 'react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {SectionHeader} from '#/components/settings/list-card';

const deploymentClient = createServiceClient(DeploymentService);

interface DatabaseLogsTabProps {
	database: any;
}

export function DatabaseLogsTab({database}: DatabaseLogsTabProps) {
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const appName = database?.app_name || database?.name || 'database';

	useEffect(() => {
		let isMounted = true;
		const controller = new AbortController();

		setStreamedLogs([]);
		setIsLoading(true);

		const startStream = async () => {
			try {
				const call = deploymentClient.streamServiceLogs(
					{
						target: appName,
						serverId: database?.server_id
							? BigInt(database.server_id)
							: undefined,
						tail: 200n,
						timestamps: false,
						follow: true,
					},
					{signal: controller.signal},
				);
				setIsLoading(false);

				let buffer = '';
				for await (const event of call) {
					if (!isMounted) return;
					if (event.error) {
						setStreamedLogs(prev => [...prev, event.error!]);
						continue;
					}
					buffer += event.payload;
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';
					for (const rawLine of rawLines) {
						const cleaned = rawLine.replace(/\r$/, '').trim();
						if (!cleaned || cleaned.includes('keep-alive')) continue;
						if (isMounted) setStreamedLogs(prev => [...prev, cleaned]);
					}
				}
				if (buffer.trim() && isMounted) {
					setStreamedLogs(prev => [...prev, buffer.trim()]);
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setIsLoading(false);
				}
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [database?.id, appName, refetchTrigger]);

	const handleDownload = () => {
		const blob = new Blob([streamedLogs.join('\n')], {type: 'text/plain'});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `${appName}-logs.txt`;
		a.click();
		URL.revokeObjectURL(url);
	};

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Logs"
				description={`Container output for '${appName}', streamed live from the server.`}
			/>
			<DeploymentViewer
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={true}
				isDeployment={false}
				loadingText={`Connecting to '${appName}' container logs...`}
				emptyText={`No runtime log output received for database '${appName}'.`}
				onDownload={handleDownload}
				onReload={() => setRefetchTrigger(prev => prev + 1)}
			/>
		</div>
	);
}
