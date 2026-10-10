import {useState, useEffect, useMemo} from 'react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {ComposeLogsHeader} from './logs/compose-logs-header';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/utils/log-lines';
import {downloadLogs} from '#/components/projects/detail/logs/use-log-stream';
import {useDeploymentItems} from '#/api/live';
import {extractServicesFromYaml} from '#/components/shared/terminal-card';
import {SectionHeader} from '#/components/settings/list-card';

const deploymentClient = createServiceClient(DeploymentService);

interface ComposeLogsTabProps {
	compose: any;
}

export function ComposeLogsTab({compose}: ComposeLogsTabProps) {
	const [logMode, setLogMode] = useState<'container' | 'build'>(
		'container',
	);
	const [selectedContainer, setSelectedContainer] = useState('');
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	// Empty means the whole stack: the backend tails every container carrying
	// the project's labels, so no invented default service name is needed.
	const activeService = selectedContainer.trim();

	// ConnectRPC log streams. Container mode tails the compose project's service logs; build mode streams the latest deployment's build log (the old REST build endpoint never worked).
	const {data: allDeployments = []} = useDeploymentItems();
	const latestBuild = useMemo(() => {
		const mine = allDeployments.filter(
			(d: any) => String(d.compose_id) === String(compose?.id),
		);
		return mine.length > 0 ? mine[0] : null;
	}, [allDeployments, compose?.id]);
	const latestBuildDeploymentId = latestBuild
		? Number(latestBuild.id)
		: null;
	const latestBuildFailed = /ERROR|FAIL|CRASH/i.test(
		String(latestBuild?.status || latestBuild?.state || ''),
	);

	useEffect(() => {
		let isMounted = true;
		let controller = new AbortController();

		setStreamedLogs([]);
		setIsLoading(true);

		const startStream = async () => {
			try {
				if (logMode === 'build') {
					if (!latestBuildDeploymentId) {
						setIsLoading(false);
						setStreamedLogs([
							'No deployment build logs found for this compose stack.',
						]);
						return;
					}
					const call = deploymentClient.streamDeploymentLogs(
						{id: BigInt(latestBuildDeploymentId)},
						{signal: controller.signal},
					);
					setIsLoading(false);
					for await (const event of call) {
						if (!isMounted) return;
						const logLines = extractLogLines(event.payload);
						if (logLines.length > 0 && isMounted)
							setStreamedLogs(prev => [...prev, ...logLines]);
					}
					return;
				}

				const call = deploymentClient.streamComposeLogs(
					{
						composeId: compose?.id ? BigInt(compose.id) : undefined,
						service: activeService || undefined,
						serverId: compose?.server_id
							? BigInt(compose.server_id)
							: undefined,
						tail: 500n,
						timestamps: false,
						follow: isLive,
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
	}, [
		compose?.id,
		activeService,
		logMode,
		isLive,
		refetchTrigger,
		latestBuildDeploymentId,
	]);

	const handleDownload = () =>
		downloadLogs(
			streamedLogs,
			`${compose?.app_name || 'compose'}-${logMode}-logs.txt`,
		);

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Logs"
				description="Container output and build logs streamed live from the server."
			/>
			<ComposeLogsHeader
				logMode={logMode}
				setLogMode={setLogMode}
				activeService={activeService}
				setSelectedContainer={setSelectedContainer}
				availableServices={availableServices}
			/>

			{/* Stream Viewer Component (< 200 lines) */}
			<DeploymentViewer
				heightClass="h-[calc(100dvh_-_310px)] min-h-[360px]"
				aiContext={logMode === 'build' ? 'BUILD' : 'RUNTIME'}
				deploymentId={
					logMode === 'build' ? (latestBuildDeploymentId ?? 0) : undefined
				}
				failed={logMode === 'build' && latestBuildFailed}
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={isLive}
				onToggleLive={
					logMode === 'container' ? () => setIsLive(!isLive) : undefined
				}
				isDeployment={logMode === 'build'}
				loadingText={
					logMode === 'build'
						? 'Connecting to build log stream...'
						: activeService
							? `Connecting to '${activeService}' log stream...`
							: 'Connecting to stack log stream...'
				}
				emptyText={
					logMode === 'build'
						? 'No deployment build logs found for this compose stack.'
						: activeService
							? `No runtime log output received for compose service '${activeService}'.`
							: 'No runtime log output received for this stack.'
				}
				onDownload={handleDownload}
				onReload={() => setRefetchTrigger(prev => prev + 1)}
			/>
		</div>
	);
}
