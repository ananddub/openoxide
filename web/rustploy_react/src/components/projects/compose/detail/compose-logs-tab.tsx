import {useState, useEffect, useRef, useMemo} from 'react';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import {ComposeLogsHeader} from './logs/compose-logs-header';

interface ComposeLogsTabProps {
	compose: any;
}

// Extract service names defined under 'services:' in docker-compose.yml content
const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const lines = yamlStr.split('\n');
	const services: string[] = [];
	let inServicesBlock = false;
	let servicesIndent = 0;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;

		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text === 'services:' || text.startsWith('services:')) {
			inServicesBlock = true;
			servicesIndent = indent;
			continue;
		}

		if (inServicesBlock) {
			if (indent <= servicesIndent && text.endsWith(':') && !text.startsWith('-')) {
				inServicesBlock = false;
			} else if (indent > servicesIndent && text.endsWith(':') && !text.includes(' ') && !text.includes('.')) {
				const serviceName = text.slice(0, -1).trim();
				if (serviceName && !services.includes(serviceName)) {
					services.push(serviceName);
				}
			}
		}
	}
	return services;
};

export function ComposeLogsTab({compose}: ComposeLogsTabProps) {
	const [logMode, setLogMode] = useState<'container' | 'build'>('container');
	const [selectedContainer, setSelectedContainer] = useState('');
	const [lines, setLines] = useState('100');
	const [timestamps, setTimestamps] = useState(false);
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [streamedLogs, setStreamedLogs] = useState<string[]>([]);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = availableServices.length > 0 ? availableServices : ['app'];
	const activeService = selectedContainer.trim() || servicesList[0] || 'app';

	// Connect to backend Server-Sent Events (SSE) log stream
	useEffect(() => {
		let isMounted = true;
		let controller = new AbortController();

		setStreamedLogs([]);
		setIsLoading(true);

		const startStream = async () => {
			try {
				const sessionRaw = localStorage.getItem('rustploy-auth-session');
				let accessToken = '';
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const streamUrl = logMode === 'build'
					? `/api/deployments/compose/${compose?.id}/logs`
					: `/api/deployments/docker/service/${activeService}/logs?tail=${lines}&timestamps=${timestamps}&follow=${isLive}`;

				const response = await fetch(streamUrl, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				if (!response.ok) {
					setIsLoading(false);
					return;
				}

				setIsLoading(false);
				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const rawLines = buffer.split('\n');
					buffer = rawLines.pop() || '';

					for (const line of rawLines) {
						const trimmed = line.trim();
						if (
							!trimmed ||
							trimmed.startsWith('event:') ||
							trimmed.includes('event: log') ||
							trimmed.includes('event: deployment') ||
							trimmed.startsWith('id:') ||
							trimmed.startsWith(':') ||
							trimmed.includes('keep-alive')
						) {
							continue;
						}

						if (line.startsWith('data:')) {
							try {
								const jsonStr = line.slice(5).trim();
								if (!jsonStr || jsonStr.includes('keep-alive')) continue;
								const data = JSON.parse(jsonStr);
								const text = data.line !== undefined ? data.line : (data.message || jsonStr);
								if (text && isMounted) setStreamedLogs(prev => [...prev, text]);
							} catch {
								const raw = line.slice(5).trim();
								if (raw && !raw.includes('keep-alive') && isMounted) setStreamedLogs(prev => [...prev, raw]);
							}
						} else if (line.trim() && isMounted) {
							setStreamedLogs(prev => [...prev, line]);
						}
					}
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
	}, [compose?.id, activeService, logMode, lines, timestamps, isLive, refetchTrigger]);

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
				loadingText={logMode === 'build' ? "Connecting to build log stream..." : `Connecting to '${activeService}' log stream...`}
				emptyText={logMode === 'build' ? "No deployment build logs found for this compose stack." : `No runtime log output received for compose service '${activeService}'.`}
				onDownload={handleDownload}
				onReload={() => setRefetchTrigger(prev => prev + 1)}
			/>
		</div>
	);
}
