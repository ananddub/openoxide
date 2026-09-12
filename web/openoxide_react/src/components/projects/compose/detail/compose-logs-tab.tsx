import {useState, useEffect, useMemo} from 'react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {ComposeLogsHeader} from './logs/compose-logs-header';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';
import {useDeploymentItems} from '#/api/live';

const deploymentClient = createServiceClient(DeploymentService);

interface ComposeLogsTabProps {
	compose: any;
}

// Robust service name extractor from docker-compose.yml YAML string
const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const services: string[] = [];
	const lines = yamlStr.split('\n');
	let inServices = false;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;

		const text = trimmed.trim();

		if (/^services\s*:/i.test(text)) {
			inServices = true;
			continue;
		}

		if (inServices) {
			if (/^(version|volumes|networks|configs|secrets)\s*:/i.test(text)) {
				inServices = false;
				continue;
			}

			const match = line.match(/^(\s{2,4}|\t)?([a-zA-Z0-9_\-]+)\s*:\s*$/);
			if (match) {
				const srv = match[2];
				if (
					srv &&
					![
						'version',
						'services',
						'volumes',
						'networks',
						'configs',
						'secrets',
						'environment',
						'ports',
						'build',
						'image',
					].includes(srv.toLowerCase())
				) {
					if (!services.includes(srv)) {
						services.push(srv);
					}
				}
			}
		}
	}

	return services;
};

export function ComposeLogsTab({compose}: ComposeLogsTabProps) {
	const [logMode, setLogMode] = useState<'container' | 'build'>(
		'container',
	);
	const [selectedContainer, setSelectedContainer] = useState('');
	const [lines, setLines] = useState('500');
	const [timestamps, setTimestamps] = useState(false);
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = useMemo(() => {
		if (availableServices.length > 0) return availableServices;
		return ['app'];
	}, [availableServices]);

	const activeService =
		selectedContainer.trim() || servicesList[0] || 'app';

	// ConnectRPC log streams. Container mode tails the compose project's
	// service logs; build mode streams the latest deployment's build log
	// (the old REST build endpoint never worked).
	const {data: allDeployments = []} = useDeploymentItems();
	const latestBuildDeploymentId = useMemo(() => {
		const mine = allDeployments.filter(
			(d: any) => String(d.compose_id) === String(compose?.id),
		);
		return mine.length > 0 ? Number(mine[0].id) : null;
	}, [allDeployments, compose?.id]);

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

				const tailParam = lines === 'all' ? 5000n : BigInt(lines);
				const call = deploymentClient.streamComposeLogs(
					{
						composeId: compose?.id ? BigInt(compose.id) : undefined,
						service: activeService,
						serverId: compose?.server_id
							? BigInt(compose.server_id)
							: undefined,
						tail: tailParam,
						timestamps,
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
		lines,
		timestamps,
		isLive,
		refetchTrigger,
		latestBuildDeploymentId,
	]);

	const handleDownload = () => {
		const blob = new Blob([streamedLogs.join('\n')], {type: 'text/plain'});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `${compose?.app_name || 'compose'}-${logMode}-logs.txt`;
		a.click();
		URL.revokeObjectURL(url);
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Controls Header Component (< 200 lines) */}
			<ComposeLogsHeader
				logMode={logMode}
				setLogMode={setLogMode}
				activeService={activeService}
				setSelectedContainer={setSelectedContainer}
				availableServices={servicesList}
				isLive={isLive}
				setIsLive={setIsLive}
				timestamps={timestamps}
				setTimestamps={setTimestamps}
				lines={lines}
				setLines={setLines}
				onRefresh={() => setRefetchTrigger(prev => prev + 1)}
				onDownload={handleDownload}
			/>

			{/* Stream Viewer Component (< 200 lines) */}
			<DeploymentViewer
				logs={streamedLogs}
				isLoading={isLoading}
				isLive={isLive}
				isDeployment={logMode === 'build'}
				loadingText={
					logMode === 'build'
						? 'Connecting to build log stream...'
						: `Connecting to '${activeService}' log stream...`
				}
				emptyText={
					logMode === 'build'
						? 'No deployment build logs found for this compose stack.'
						: `No runtime log output received for compose service '${activeService}'.`
				}
				onDownload={handleDownload}
				onReload={() => setRefetchTrigger(prev => prev + 1)}
			/>
		</div>
	);
}
